//! WorkflowPlayer - 游戏播放器
//!
//! 用法: workflow_player [game.lpack]
//!
//! 从加密的游戏数据包中加载工作流和脚本运行游戏。
//! 如果不指定文件，会自动扫描同目录下的 .lpack 文件。
//! 多个 .lpack 时会显示选择界面。

mod script;
mod workflow;

use script::{BlockDefinition, ScriptParser, Value};
use workflow::{GamePackage, Workflow, Vec2};

use anyhow::{anyhow, Result};
use egui::{CentralPanel, Context, FontData, FontDefinitions, FontFamily};
use mlua::{Lua, Table, Value as LuaValue};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/// 内存脚本注册表（从 GamePackage 加载，不读取文件）
pub struct MemoryRegistry {
    definitions: HashMap<String, BlockDefinition>,
    sources: HashMap<String, String>,
}

impl MemoryRegistry {
    pub fn from_package(package: &GamePackage) -> Result<Self> {
        let parser = ScriptParser::new()?;
        let mut definitions = HashMap::new();

        for (script_id, source) in &package.scripts {
            let virtual_path = PathBuf::from(format!("memory://{}.lua", script_id));
            match parser.parse_from_source(source, &virtual_path) {
                Ok(def) => { definitions.insert(script_id.clone(), def); }
                Err(e) => { log::warn!("解析脚本 {} 失败: {}", script_id, e); }
            }
        }

        Ok(Self { definitions, sources: package.scripts.clone() })
    }

    pub fn get(&self, id: &str) -> Option<&BlockDefinition> {
        self.definitions.get(id)
    }

    pub fn get_source(&self, id: &str) -> Option<&String> {
        self.sources.get(id)
    }
}

/// 内存执行器
pub struct MemoryExecutor {
    lua: Lua,
}

impl MemoryExecutor {
    pub fn new() -> Result<Self> {
        Ok(Self { lua: Lua::new() })
    }

    pub fn execute_all(&self, workflow: &mut Workflow, registry: &MemoryRegistry) -> Result<()> {
        let order = workflow.execution_order.clone();
        for block_id in order {
            self.execute_block(workflow, registry, block_id)?;
        }
        workflow.dirty_blocks.clear();
        Ok(())
    }

    fn execute_block(&self, workflow: &mut Workflow, registry: &MemoryRegistry, block_id: Uuid) -> Result<()> {
        let block = match workflow.blocks.get(&block_id) { Some(b) => b, None => return Ok(()) };
        let source = match registry.get_source(&block.script_id) { Some(s) => s, None => return Ok(()) };

        // 收集输入
        let mut inputs: HashMap<String, Value> = block.input_values.clone();
        let input_conns: Vec<_> = workflow.get_input_connections(block_id).iter()
            .filter_map(|c| workflow.blocks.get(&c.from_block)
                .and_then(|b| b.output_values.get(&c.from_port).map(|v| (c.to_port.clone(), v.clone()))))
            .collect();
        for (port, val) in input_conns { inputs.insert(port, val); }

        // 执行
        let script_table: Table = self.lua.load(source).eval().map_err(|e| anyhow!("Lua错误: {}", e))?;
        let self_table = self.lua.create_table().map_err(|e| anyhow!("{}", e))?;

        let props_table = self.lua.create_table().map_err(|e| anyhow!("{}", e))?;
        for (k, v) in &block.properties { props_table.set(k.as_str(), self.value_to_lua(v)?).ok(); }
        self_table.set("properties", props_table).ok();

        let state_table = self.lua.create_table().map_err(|e| anyhow!("{}", e))?;
        for (k, v) in &block.state { state_table.set(k.as_str(), self.value_to_lua(v)?).ok(); }
        self_table.set("state", state_table).ok();

        let inputs_table = self.lua.create_table().map_err(|e| anyhow!("{}", e))?;
        for (k, v) in &inputs { inputs_table.set(k.as_str(), self.value_to_lua(v)?).ok(); }

        if let Ok(execute_fn) = script_table.get::<mlua::Function>("execute") {
            if let Ok(result) = execute_fn.call::<Table>((self_table.clone(), inputs_table)) {
                // 更新state
                if let Ok(new_state) = self_table.get::<Table>("state") {
                    if let Some(block) = workflow.blocks.get_mut(&block_id) {
                        for pair in new_state.pairs::<String, LuaValue>() {
                            if let Ok((k, v)) = pair {
                                if let Ok(val) = self.lua_to_value(v) { block.state.insert(k, val); }
                            }
                        }
                    }
                }
                // 更新outputs
                if let Some(block) = workflow.blocks.get_mut(&block_id) {
                    for pair in result.pairs::<String, LuaValue>() {
                        if let Ok((k, v)) = pair {
                            if let Ok(val) = self.lua_to_value(v) { block.output_values.insert(k, val); }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn value_to_lua(&self, value: &Value) -> Result<LuaValue> {
        Ok(match value {
            Value::Nil => LuaValue::Nil,
            Value::Boolean(b) => LuaValue::Boolean(*b),
            Value::Number(n) => LuaValue::Number(*n),
            Value::String(s) => LuaValue::String(self.lua.create_string(s).map_err(|e| anyhow!("{}", e))?),
            Value::Array(arr) => {
                let t = self.lua.create_table().map_err(|e| anyhow!("{}", e))?;
                for (i, v) in arr.iter().enumerate() { t.set(i + 1, self.value_to_lua(v)?).ok(); }
                LuaValue::Table(t)
            }
            Value::Object(map) => {
                let t = self.lua.create_table().map_err(|e| anyhow!("{}", e))?;
                for (k, v) in map { t.set(k.as_str(), self.value_to_lua(v)?).ok(); }
                LuaValue::Table(t)
            }
        })
    }

    fn lua_to_value(&self, value: LuaValue) -> Result<Value> {
        Ok(match value {
            LuaValue::Nil => Value::Nil,
            LuaValue::Boolean(b) => Value::Boolean(b),
            LuaValue::Integer(i) => Value::Number(i as f64),
            LuaValue::Number(n) => Value::Number(n),
            LuaValue::String(s) => Value::String(s.to_str().map(|s| s.to_string()).unwrap_or_default()),
            LuaValue::Table(t) => {
                let mut is_array = true;
                let mut max_idx = 0usize;
                for pair in t.clone().pairs::<LuaValue, LuaValue>() {
                    if let Ok((k, _)) = pair {
                        match k { LuaValue::Integer(i) if i > 0 => { max_idx = max_idx.max(i as usize); }
                            _ => { is_array = false; break; } }
                    }
                }
                if is_array && max_idx > 0 {
                    let mut arr = Vec::with_capacity(max_idx);
                    for i in 1..=max_idx {
                        let v = t.get::<LuaValue>(i).unwrap_or(LuaValue::Nil);
                        arr.push(self.lua_to_value(v)?);
                    }
                    Value::Array(arr)
                } else {
                    let mut map = HashMap::new();
                    for pair in t.pairs::<String, LuaValue>() {
                        if let Ok((k, v)) = pair { map.insert(k, self.lua_to_value(v)?); }
                    }
                    Value::Object(map)
                }
            }
            _ => Value::Nil,
        })
    }
}

/// 播放器应用
struct PlayerApp {
    registry: MemoryRegistry,
    workflow: Workflow,
    executor: MemoryExecutor,
    package_name: String,
    last_execute_time: std::time::Instant,
    execution_speed: f32,
    auto_execute: bool,
    error_message: Option<String>,
}

impl PlayerApp {
    fn new(package_path: PathBuf) -> Result<Self> {
        log::info!("加载游戏包: {}", package_path.display());

        let package = GamePackage::load(&package_path)?;
        let registry = MemoryRegistry::from_package(&package)?;

        let mut workflow = package.workflow;
        // 更新Block尺寸
        for block in workflow.blocks.values_mut() {
            if let Some(def) = registry.get(&block.script_id) {
                block.size = Vec2::new(def.calculate_width(), def.calculate_height());
            }
        }
        workflow.update_execution_order();

        log::info!("游戏: {} v{}", package.name, package.version);
        log::info!("Block数: {}, 脚本数: {}", workflow.blocks.len(), registry.sources.len());

        let executor = MemoryExecutor::new()?;

        Ok(Self {
            registry,
            workflow,
            executor,
            package_name: package.name,
            last_execute_time: std::time::Instant::now(),
            execution_speed: 10.0,
            auto_execute: true,
            error_message: None,
        })
    }

    fn run_workflow(&mut self) {
        let all_ids: Vec<Uuid> = self.workflow.blocks.keys().cloned().collect();
        for id in all_ids {
            self.workflow.mark_dirty(id);
        }

        if let Err(e) = self.executor.execute_all(&mut self.workflow, &self.registry) {
            self.error_message = Some(format!("执行错误: {}", e));
            log::error!("{}", self.error_message.as_ref().unwrap());
        }
    }
}

impl eframe::App for PlayerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // 自动执行
        let interval = 1.0 / self.execution_speed.max(0.1);
        if self.auto_execute && self.last_execute_time.elapsed().as_secs_f32() >= interval {
            self.last_execute_time = std::time::Instant::now();
            self.run_workflow();
        }

        // 顶部控制栏
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(&self.package_name);
                ui.separator();

                let play_text = if self.auto_execute { "⏸ 暂停" } else { "▶ 运行" };
                if ui.button(play_text).clicked() {
                    self.auto_execute = !self.auto_execute;
                }

                ui.label("速度:");
                ui.add(egui::Slider::new(&mut self.execution_speed, 1.0..=60.0).suffix(" Hz"));

                if ui.button("⏯ 单步").clicked() {
                    self.run_workflow();
                }

                if let Some(ref err) = self.error_message {
                    ui.separator();
                    ui.colored_label(egui::Color32::RED, err);
                }
            });
        });

        // 主内容区 - 显示输出
        CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for block in self.workflow.blocks.values() {
                    if let Some(def) = self.registry.get(&block.script_id) {
                        let name = block.custom_name.as_ref().unwrap_or(&def.meta.name);
                        ui.collapsing(name, |ui| {
                            for output in &def.outputs {
                                if let Some(value) = block.output_values.get(&output.id) {
                                    ui.horizontal(|ui| {
                                        ui.label(&output.name);
                                        ui.label(":");
                                        ui.monospace(format!("{:?}", value));
                                    });
                                }
                            }
                        });
                    }
                }
            });
        });

        ctx.request_repaint();
    }
}

/// 配置中文字体
fn setup_chinese_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    #[cfg(target_os = "macos")]
    let font_paths: &[&str] = &[
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/STHeiti Light.ttc",
    ];

    #[cfg(target_os = "windows")]
    let font_paths: &[&str] = &[
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\simsun.ttc",
    ];

    #[cfg(target_os = "linux")]
    let font_paths: &[&str] = &[
        "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
    ];

    for path in font_paths {
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert("chinese".to_owned(), FontData::from_owned(font_data));
            if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
                family.insert(0, "chinese".to_owned());
            }
            if let Some(family) = fonts.families.get_mut(&FontFamily::Monospace) {
                family.insert(0, "chinese".to_owned());
            }
            ctx.set_fonts(fonts);
            return;
        }
    }
}

/// 扫描目录下的 .lpack 文件
fn scan_lpack_files() -> Vec<PathBuf> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&exe_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "lpack").unwrap_or(false) {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

/// 解析 lpack 文件获取游戏名（用于选择界面）
fn get_package_info(path: &PathBuf) -> Option<(String, String)> {
    match GamePackage::load(path) {
        Ok(pkg) => Some((pkg.name, pkg.version)),
        Err(_) => None,
    }
}

/// 启动模式
enum LaunchMode {
    /// 选择游戏
    Selecting { games: Vec<(PathBuf, String, String)> },
    /// 运行游戏
    Playing(PlayerApp),
    /// 加载失败
    Error(String),
}

/// 统一应用（支持选择和运行两种模式）
struct UnifiedApp {
    mode: LaunchMode,
}

impl UnifiedApp {
    fn new_selector(files: Vec<PathBuf>) -> Self {
        let games: Vec<_> = files.into_iter()
            .filter_map(|path| {
                get_package_info(&path).map(|(name, ver)| (path, name, ver))
            })
            .collect();
        Self { mode: LaunchMode::Selecting { games } }
    }

    fn new_player(path: PathBuf) -> Self {
        match PlayerApp::new(path) {
            Ok(app) => Self { mode: LaunchMode::Playing(app) },
            Err(e) => Self { mode: LaunchMode::Error(format!("加载失败: {}", e)) },
        }
    }
}

impl eframe::App for UnifiedApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        match &mut self.mode {
            LaunchMode::Selecting { games } => {
                let mut selected_path: Option<PathBuf> = None;

                CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(40.0);
                        ui.heading("🎮 选择游戏");
                        ui.add_space(20.0);

                        for (path, name, version) in games.iter() {
                            let btn_text = format!("{} (v{})", name, version);
                            if ui.add_sized([300.0, 40.0], egui::Button::new(&btn_text)).clicked() {
                                selected_path = Some(path.clone());
                            }
                            ui.add_space(8.0);
                        }

                        if games.is_empty() {
                            ui.label("未找到任何有效游戏包 (.lpack)");
                        }

                        ui.add_space(20.0);
                        if ui.button("退出").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });

                // 选择后切换到播放模式
                if let Some(path) = selected_path {
                    match PlayerApp::new(path) {
                        Ok(app) => self.mode = LaunchMode::Playing(app),
                        Err(e) => self.mode = LaunchMode::Error(format!("加载失败: {}", e)),
                    }
                }
            }
            LaunchMode::Playing(app) => {
                app.update(ctx, frame);
            }
            LaunchMode::Error(msg) => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.colored_label(egui::Color32::RED, "❌ 错误");
                        ui.add_space(10.0);
                        ui.label(msg.as_str());
                        ui.add_space(20.0);
                        if ui.button("退出").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
            }
        }
    }
}

fn main() -> eframe::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args: Vec<String> = std::env::args().collect();

    // 创建统一应用
    let app: UnifiedApp = if args.len() >= 2 {
        // 命令行指定
        let path = PathBuf::from(&args[1]);
        if !path.exists() {
            eprintln!("错误: 文件不存在: {}", path.display());
            std::process::exit(1);
        }
        UnifiedApp::new_player(path)
    } else {
        // 自动扫描
        let files = scan_lpack_files();
        match files.len() {
            0 => {
                eprintln!("错误: 未找到任何 .lpack 游戏包");
                eprintln!("用法: workflow_player [game.lpack]");
                std::process::exit(1);
            }
            1 => UnifiedApp::new_player(files.into_iter().next().unwrap()),
            _ => UnifiedApp::new_selector(files),
        }
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Workflow Player",
        options,
        Box::new(|cc| {
            setup_chinese_fonts(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

