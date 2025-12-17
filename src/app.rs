//! 应用状态

use crate::script::{ScriptRegistry, ScriptWatcher};
use crate::ui::{BlockWidget, Canvas, ConnectionMode, ConnectionWidget, MenuEvent, PropertyPanel, SideMenu};
use crate::workflow::{Block, BlueprintStorage, Clipboard, Connection, Vec2, Workflow, WorkflowExecutor};
use anyhow::Result;
use egui::{CentralPanel, Context, Key, Pos2, SidePanel};
use std::collections::HashSet;
use std::path::PathBuf;
use uuid::Uuid;

/// 正在拖拽的端口信息
#[derive(Debug, Clone)]
struct DraggingPort {
    block_id: Uuid,
    port_id: String,
    is_output: bool,
    port_index: usize,
}

/// 交互状态
#[derive(Debug, Clone, Default)]
enum InteractionState {
    #[default]
    Idle,
    DraggingBlock(Uuid),
    Panning,
    BoxSelecting { start: Pos2 },
    DraggingFromMenu(String),
    DraggingConnection { from: DraggingPort, mouse_pos: Pos2 },
    EditingBlockName { block_id: Uuid, edit_text: String },
}

/// 日志条目
#[derive(Clone)]
struct LogEntry {
    level: String,
    message: String,
}

/// 撤销/重做历史快照
#[derive(Clone)]
struct HistorySnapshot {
    workflow_json: String,
}

/// 主应用
pub struct WorkflowApp {
    registry: ScriptRegistry,
    watcher: Option<ScriptWatcher>,
    workflow: Workflow,
    executor: WorkflowExecutor,
    clipboard: Clipboard,
    state: InteractionState,
    canvas_rect: egui::Rect,
    logs: Vec<LogEntry>,
    show_log_panel: bool,
    selected_connections: HashSet<Uuid>,
    box_select_end: Option<Pos2>,
    last_execute_time: std::time::Instant,
    space_pressed: bool,
    auto_execute: bool,
    execution_speed: f32,
    // 文件对话框状态
    show_save_dialog: bool,
    show_password_dialog: bool,
    password_input: String,
    pending_operation: Option<FileOperation>,
    current_file_path: Option<std::path::PathBuf>,
    save_options: SaveOptions,
    // 流动效果
    flow_phase: f32,
    use_bezier_mode: bool,
    // 右键菜单
    context_menu_pos: Option<Pos2>,
    context_menu_target: ContextMenuTarget,
    // 撤销/重做
    undo_stack: Vec<HistorySnapshot>,
    redo_stack: Vec<HistorySnapshot>,
    last_snapshot_time: std::time::Instant,
}

/// 右键菜单目标
#[derive(Clone, Default)]
enum ContextMenuTarget {
    #[default]
    Canvas,
    Block(Uuid),
    Connection(Uuid),
}

#[derive(Clone)]
enum FileOperation {
    Save(std::path::PathBuf),
    SaveDual(std::path::PathBuf),
    Load(std::path::PathBuf),
}

/// 保存选项
#[derive(Clone, Default)]
struct SaveOptions {
    encrypted: bool,
    readonly: bool,
    dual_save: bool,
}

impl WorkflowApp {
    pub fn new(script_dir: PathBuf) -> Result<Self> {
        let registry = ScriptRegistry::new(&script_dir)?;
        let watcher = ScriptWatcher::new(&script_dir).ok();
        let executor = WorkflowExecutor::new()?;

        // 收集加载信息
        let mut logs = Vec::new();
        logs.push(LogEntry {
            level: "INFO".to_string(),
            message: format!("脚本目录: {}", script_dir.display()),
        });
        for def in registry.all() {
            logs.push(LogEntry {
                level: "INFO".to_string(),
                message: format!("已加载: [{}] {}", def.meta.category, def.meta.name),
            });
        }

        Ok(Self {
            registry,
            watcher,
            workflow: Workflow::new("新工作流"),
            executor,
            clipboard: Clipboard::new(),
            state: InteractionState::Idle,
            canvas_rect: egui::Rect::NOTHING,
            logs,
            show_log_panel: true,
            selected_connections: HashSet::new(),
            box_select_end: None,
            last_execute_time: std::time::Instant::now(),
            space_pressed: false,
            auto_execute: true,
            execution_speed: 10.0,
            show_save_dialog: false,
            show_password_dialog: false,
            password_input: String::new(),
            pending_operation: None,
            current_file_path: None,
            save_options: SaveOptions::default(),
            flow_phase: 0.0,
            use_bezier_mode: false,
            context_menu_pos: None,
            context_menu_target: ContextMenuTarget::Canvas,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_snapshot_time: std::time::Instant::now(),
        })
    }

    /// 保存当前状态到撤销栈
    fn save_undo_snapshot(&mut self) {
        // 防止频繁保存（至少间隔100ms）
        if self.last_snapshot_time.elapsed().as_millis() < 100 {
            return;
        }

        if let Ok(json) = serde_json::to_string(&self.workflow) {
            self.undo_stack.push(HistorySnapshot { workflow_json: json });
            // 保留最近50次操作
            if self.undo_stack.len() > 50 {
                self.undo_stack.remove(0);
            }
            // 新操作清空重做栈
            self.redo_stack.clear();
            self.last_snapshot_time = std::time::Instant::now();
        }
    }

    /// 撤销
    fn undo(&mut self) {
        if self.workflow.readonly {
            self.add_log("WARN", "只读模式，无法撤销".to_string());
            return;
        }

        if let Some(snapshot) = self.undo_stack.pop() {
            // 保存当前状态到重做栈
            if let Ok(current_json) = serde_json::to_string(&self.workflow) {
                self.redo_stack.push(HistorySnapshot { workflow_json: current_json });
                if self.redo_stack.len() > 50 {
                    self.redo_stack.remove(0);
                }
            }
            // 恢复之前的状态
            if let Ok(workflow) = serde_json::from_str::<Workflow>(&snapshot.workflow_json) {
                self.workflow = workflow;
                self.selected_connections.clear();
                self.add_log("INFO", "已撤销".to_string());
            }
        } else {
            self.add_log("INFO", "没有可撤销的操作".to_string());
        }
    }

    /// 重做
    fn redo(&mut self) {
        if self.workflow.readonly {
            self.add_log("WARN", "只读模式，无法重做".to_string());
            return;
        }

        if let Some(snapshot) = self.redo_stack.pop() {
            // 保存当前状态到撤销栈
            if let Ok(current_json) = serde_json::to_string(&self.workflow) {
                self.undo_stack.push(HistorySnapshot { workflow_json: current_json });
                if self.undo_stack.len() > 50 {
                    self.undo_stack.remove(0);
                }
            }
            // 恢复重做状态
            if let Ok(workflow) = serde_json::from_str::<Workflow>(&snapshot.workflow_json) {
                self.workflow = workflow;
                self.selected_connections.clear();
                self.add_log("INFO", "已重做".to_string());
            }
        } else {
            self.add_log("INFO", "没有可重做的操作".to_string());
        }
    }

    /// 执行工作流（自动调用）
    fn run_workflow(&mut self) {
        // 标记所有block为脏，触发执行
        let all_ids: Vec<Uuid> = self.workflow.blocks.keys().cloned().collect();
        for id in all_ids {
            self.workflow.mark_dirty(id);
        }

        if let Err(e) = self.executor.execute_dirty(&mut self.workflow, &self.registry) {
            self.add_log("ERROR", format!("执行错误: {}", e));
        }
    }

    /// 添加日志
    fn add_log(&mut self, level: &str, message: String) {
        self.logs.push(LogEntry {
            level: level.to_string(),
            message,
        });
        // 保持最多100条
        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }

    /// 格式化值为JSON风格字符串（紧凑）
    #[allow(dead_code)]
    fn format_value_json(value: &crate::script::Value) -> String {
        use crate::script::Value;
        match value {
            Value::Nil => "null".to_string(),
            Value::Boolean(b) => if *b { "true" } else { "false" }.to_string(),
            Value::Number(n) => format!("{}", n),
            Value::String(s) => format!("\"{}\"", s),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(Self::format_value_json).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Object(map) => {
                let items: Vec<String> = map.iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, Self::format_value_json(v)))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
        }
    }

    /// 格式化值为易读的字符串（支持换行，美观）
    fn format_value_pretty(value: &crate::script::Value) -> String {
        Self::format_value_pretty_indent(value, 0)
    }

    fn format_value_pretty_indent(value: &crate::script::Value, indent: usize) -> String {
        use crate::script::Value;
        let prefix = "  ".repeat(indent);
        let child_prefix = "  ".repeat(indent + 1);

        match value {
            Value::Nil => "null".to_string(),
            Value::Boolean(b) => if *b { "true" } else { "false" }.to_string(),
            Value::Number(n) => {
                // 整数显示为整数，浮点数保留精度
                if n.fract() == 0.0 && n.abs() < 1e10 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::String(s) => {
                // 字符串不加引号，更易读
                s.clone()
            }
            Value::Array(arr) => {
                if arr.is_empty() {
                    "[]".to_string()
                } else if arr.len() <= 3 && arr.iter().all(|v| match v {
                    Value::Number(_) | Value::Boolean(_) => true,
                    Value::String(s) => s.len() < 20,
                    _ => false,
                }) {
                    // 短数组单行显示
                    let items: Vec<String> = arr.iter().map(|v| Self::format_value_pretty_indent(v, 0)).collect();
                    format!("[{}]", items.join(", "))
                } else {
                    // 长数组多行显示
                    let items: Vec<String> = arr.iter()
                        .map(|v| format!("{}{}", child_prefix, Self::format_value_pretty_indent(v, indent + 1)))
                        .collect();
                    format!("[\n{}\n{}]", items.join(",\n"), prefix)
                }
            }
            Value::Object(map) => {
                if map.is_empty() {
                    "{}".to_string()
                } else {
                    let items: Vec<String> = map.iter()
                        .map(|(k, v)| format!("{}{}: {}", child_prefix, k, Self::format_value_pretty_indent(v, indent + 1)))
                        .collect();
                    format!("{{\n{}\n{}}}", items.join(",\n"), prefix)
                }
            }
        }
    }

    /// 紧凑格式化（用于侧边栏日志）
    fn format_value_compact(value: &crate::script::Value) -> String {
        use crate::script::Value;
        match value {
            Value::Nil => "null".to_string(),
            Value::Boolean(b) => if *b { "true" } else { "false" }.to_string(),
            Value::Number(n) => {
                if n.fract() == 0.0 && n.abs() < 1e10 {
                    format!("{}", *n as i64)
                } else {
                    format!("{:.2}", n)
                }
            }
            Value::String(s) => s.clone(),
            Value::Array(arr) => {
                if arr.is_empty() { return "[]".to_string(); }
                if arr.len() <= 5 {
                    let items: Vec<String> = arr.iter().map(Self::format_value_compact).collect();
                    format!("[{}]", items.join(", "))
                } else {
                    format!("[...{}项]", arr.len())
                }
            }
            Value::Object(map) => {
                if map.is_empty() { return "{}".to_string(); }
                if map.len() <= 3 {
                    let items: Vec<String> = map.iter()
                        .map(|(k, v)| format!("{}: {}", k, Self::format_value_compact(v)))
                        .collect();
                    format!("{{{}}}", items.join(", "))
                } else {
                    format!("{{...{}项}}", map.len())
                }
            }
        }
    }

    /// 处理热重载
    fn handle_hot_reload(&mut self) {
        if let Some(watcher) = &self.watcher {
            let changed = watcher.poll_changes();
            for path in changed {
                log::info!("热重载: {}", path.display());
                if let Err(e) = self.registry.reload_script(&path) {
                    log::error!("重载失败: {}", e);
                }
            }
        }
    }

    /// 处理快捷键
    fn handle_shortcuts(&mut self, ctx: &Context) {
        // 处理Block名称编辑状态
        if let InteractionState::EditingBlockName { block_id, ref edit_text } = self.state.clone() {
            let enter = ctx.input(|i| i.key_pressed(Key::Enter));
            let escape = ctx.input(|i| i.key_pressed(Key::Escape));

            if enter {
                // Enter: 保存编辑
                self.save_undo_snapshot();
                if let Some(block) = self.workflow.blocks.get_mut(&block_id) {
                    if edit_text.trim().is_empty() {
                        block.custom_name = None;
                    } else {
                        block.custom_name = Some(edit_text.clone());
                    }
                }
                self.state = InteractionState::Idle;
                self.add_log("INFO", "Block名称已修改".to_string());
                return;
            }
            if escape {
                // Escape: 取消编辑
                self.state = InteractionState::Idle;
                return;
            }
            // 编辑状态时不处理其他快捷键
            return;
        }

        let modifiers = ctx.input(|i| i.modifiers);

        ctx.input(|i| {
            // 跨平台修饰键：Mac用Cmd，Windows/Linux用Ctrl
            let cmd_or_ctrl = modifiers.command || modifiers.ctrl;

            // Ctrl/Cmd+Z 撤销 / Ctrl/Cmd+Shift+Z 重做
            if cmd_or_ctrl && i.key_pressed(Key::Z) {
                if modifiers.shift {
                    self.redo();
                } else {
                    self.undo();
                }
            }

            // Ctrl/Cmd+Y 重做（Windows风格）
            if cmd_or_ctrl && i.key_pressed(Key::Y) {
                self.redo();
            }

            // Delete 删除
            if i.key_pressed(Key::Delete) || i.key_pressed(Key::Backspace) {
                self.delete_selected();
            }

            // Ctrl/Cmd+C 复制
            if cmd_or_ctrl && i.key_pressed(Key::C) {
                self.copy_selected();
            }

            // Ctrl/Cmd+V 粘贴
            if cmd_or_ctrl && i.key_pressed(Key::V) {
                self.paste_at_cursor();
            }

            // Ctrl/Cmd+A 全选
            if cmd_or_ctrl && i.key_pressed(Key::A) {
                for block in self.workflow.blocks.values_mut() {
                    block.selected = true;
                }
            }

            // Ctrl/Cmd+G 分组
            if cmd_or_ctrl && i.key_pressed(Key::G) {
                if modifiers.shift {
                    // 取消分组
                    let groups: Vec<_> = self.workflow.groups.keys().cloned().collect();
                    for id in groups {
                        self.workflow.ungroup(id);
                    }
                } else {
                    self.workflow.create_group("新分组".to_string());
                }
            }
        });
    }
}

impl eframe::App for WorkflowApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.handle_hot_reload();
        self.handle_shortcuts(ctx);

        // 更新流动效果
        if self.auto_execute {
            self.flow_phase = (self.flow_phase + 0.02) % 1.0;
        }

        // 更新连线模式
        ConnectionWidget::set_mode(if self.use_bezier_mode {
            ConnectionMode::Bezier
        } else {
            ConnectionMode::Orthogonal
        });

        // 自动执行工作流（根据速度设置）
        let interval = 1.0 / self.execution_speed.max(0.1);
        if self.auto_execute && self.last_execute_time.elapsed().as_secs_f32() >= interval {
            self.last_execute_time = std::time::Instant::now();
            if !self.workflow.blocks.is_empty() {
                self.run_workflow();
            }
        }
        // 请求持续重绘
        ctx.request_repaint();

        // 顶部工具栏
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("WorkflowEngine");
                ui.separator();

                // 文件操作
                if ui.button("📂 打开").clicked() {
                    self.open_file_dialog();
                }
                if ui.button("💾 保存").clicked() {
                    self.show_save_dialog = true;
                    self.save_options = SaveOptions::default();
                }

                ui.separator();

                // 执行控制
                let play_text = if self.auto_execute { "⏸ 暂停" } else { "▶ 运行" };
                if ui.button(play_text).clicked() {
                    self.auto_execute = !self.auto_execute;
                }

                ui.label("速度:");
                ui.add(egui::Slider::new(&mut self.execution_speed, 1.0..=60.0).suffix(" Hz"));

                if ui.button("⏯ 单步").clicked() {
                    self.run_workflow();
                }

                ui.separator();

                // 连线模式切换
                let mode_text = if self.use_bezier_mode { "〰️ 曲线" } else { "⌐ 折线" };
                if ui.button(mode_text).clicked() {
                    self.use_bezier_mode = !self.use_bezier_mode;
                }

                // 自动布局
                if ui.button("📐 布局").clicked() {
                    self.workflow.auto_layout();
                    self.add_log("INFO", "已自动布局".to_string());
                }

                // 显示/隐藏日志
                let log_text = if self.show_log_panel { "📋" } else { "📋 输出" };
                if ui.button(log_text).clicked() {
                    self.show_log_panel = !self.show_log_panel;
                }

                ui.separator();

                // 只读模式提示
                if self.workflow.readonly {
                    ui.colored_label(egui::Color32::from_rgb(255, 100, 100), "🔒 只读模式");
                }

                // 当前文件名
                if let Some(path) = &self.current_file_path {
                    ui.label(format!("📄 {}", path.file_name().unwrap_or_default().to_string_lossy()));
                }

                ui.label(format!("Blocks: {}", self.workflow.blocks.len()));

                if !self.selected_connections.is_empty() {
                    ui.separator();
                    let count = self.selected_connections.len();
                    ui.colored_label(egui::Color32::from_rgb(255, 100, 100), format!("连线已选中: {}", count));
                    if ui.button("🗑 删除连线").clicked() {
                        let to_remove: Vec<_> = self.selected_connections.drain().collect();
                        for conn_id in to_remove {
                            self.workflow.remove_connection(conn_id);
                        }
                        self.add_log("INFO", format!("删除 {} 条连接", count));
                    }
                }
            });
        });

        // 对话框
        self.draw_save_dialog(ctx);
        self.draw_password_dialog(ctx);

        // 侧边菜单
        // 左侧Block菜单
        SidePanel::left("menu").min_width(160.0).show(ctx, |ui| {
            if let Some(event) = SideMenu::draw(ui, &self.registry) {
                match event {
                    MenuEvent::DragBlock(script_id) => {
                        self.state = InteractionState::DraggingFromMenu(script_id);
                    }
                }
            }
        });

        // 底部属性面板（先绘制，这样右侧面板可以占据剩余全高）
        egui::TopBottomPanel::bottom("properties")
            .resizable(true)
            .show(ctx, |ui| {

                let selected = self.workflow.selected_blocks();
                if selected.len() == 1 {
                    if let Some(block) = self.workflow.blocks.get(&selected[0]) {
                        if let Some(def) = self.registry.get(&block.script_id) {
                            let changes = PropertyPanel::draw(ui, block, def);
                            if !changes.is_empty() {
                                let block_id = selected[0];
                                if let Some(block) = self.workflow.blocks.get_mut(&block_id) {
                                    for change in changes {
                                        block.properties.insert(change.property_id, change.new_value);
                                    }
                                }
                                self.workflow.mark_dirty(block_id);
                            }
                        }
                    }
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.label(egui::RichText::new("选择Block查看属性").weak().size(11.0));
                    });
                }
            });

        // 右侧日志面板（后绘制，占据底部面板上方的全高）
        if self.show_log_panel {
            SidePanel::right("log_panel")
                .min_width(200.0)
                .max_width(400.0)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.strong("📋 输出");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("✕").clicked() {
                                self.show_log_panel = false;
                            }
                        });
                    });
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            for block in self.workflow.blocks.values() {
                                if let Some(def) = self.registry.get(&block.script_id) {
                                    let display_name = block.display_name(def);
                                    let header_id = egui::Id::new(block.id).with("log_header");

                                    egui::CollapsingHeader::new(
                                        egui::RichText::new(display_name).size(11.0)
                                    )
                                        .id_salt(header_id)
                                        .default_open(true)
                                        .show(ui, |ui| {
                                            ui.spacing_mut().item_spacing.y = 2.0;
                                            for output in &def.outputs {
                                                if let Some(value) = block.output_values.get(&output.id) {
                                                    let val_str = Self::format_value_compact(value);
                                                    ui.horizontal_wrapped(|ui| {
                                                        ui.colored_label(
                                                            egui::Color32::from_rgb(100, 160, 220),
                                                            egui::RichText::new(format!("{}:", output.name)).size(10.0)
                                                        );
                                                        ui.add(egui::Label::new(
                                                            egui::RichText::new(&val_str)
                                                                .monospace()
                                                                .size(10.0)
                                                                .color(egui::Color32::from_rgb(180, 200, 180))
                                                        ).wrap());
                                                    });
                                                }
                                            }
                                        });
                                }
                            }
                        });
                });
        }

        // 主画布
        CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(
                ui.available_size(),
                egui::Sense::click_and_drag(),
            );
            self.canvas_rect = response.rect;
            let canvas_offset = response.rect.min;

            // 绘制网格
            Canvas::draw_grid(&painter, &self.workflow.viewport, response.rect);

            // 绘制分组
            for group in self.workflow.groups.values() {
                let min = Canvas::vec2_to_pos2(group.position, &self.workflow.viewport, canvas_offset);
                let max = Canvas::vec2_to_pos2(
                    Vec2::new(group.position.x + group.size.x, group.position.y + group.size.y),
                    &self.workflow.viewport,
                    canvas_offset,
                );
                let rect = egui::Rect::from_min_max(min, max);
                let color = egui::Color32::from_rgba_unmultiplied(
                    group.color[0], group.color[1], group.color[2], 30
                );
                painter.rect_filled(rect, 8.0, color);
                painter.text(
                    Pos2::new(min.x + 8.0, min.y + 4.0),
                    egui::Align2::LEFT_TOP,
                    &group.name,
                    egui::FontId::proportional(12.0),
                    egui::Color32::WHITE,
                );
            }

            // 绘制连接（视口裁剪优化）
            let viewport_rect = response.rect;
            for (conn_id, conn) in &self.workflow.connections {
                if let (Some(from_block), Some(to_block)) = (
                    self.workflow.blocks.get(&conn.from_block),
                    self.workflow.blocks.get(&conn.to_block),
                ) {
                    if let Some(from_def) = self.registry.get(&from_block.script_id) {
                        if let Some(to_def) = self.registry.get(&to_block.script_id) {
                            let from_idx = from_def.outputs.iter()
                                .position(|p| p.id == conn.from_port)
                                .unwrap_or(0);
                            let to_idx = to_def.inputs.iter()
                                .position(|p| p.id == conn.to_port)
                                .unwrap_or(0);

                            let from_pos = BlockWidget::get_port_screen_pos(
                                from_block, from_idx, true, &self.workflow.viewport, canvas_offset
                            );
                            let to_pos = BlockWidget::get_port_screen_pos(
                                to_block, to_idx, false, &self.workflow.viewport, canvas_offset
                            );

                            // 视口裁剪：检查连线是否在可见区域
                            let conn_rect = egui::Rect::from_two_pos(from_pos, to_pos).expand(50.0);
                            if !conn_rect.intersects(viewport_rect) {
                                continue;
                            }

                            let is_selected = self.selected_connections.contains(conn_id);
                            let activation = self.workflow.get_connection_activation(*conn_id);
                            ConnectionWidget::draw_with_flow(&painter, from_pos, to_pos, is_selected, activation);
                        }
                    }
                }
            }

            // 绘制Block（视口裁剪优化）
            for block in self.workflow.blocks.values() {
                // 计算Block屏幕位置
                let screen_pos = Pos2::new(
                    block.position.x * self.workflow.viewport.zoom + self.workflow.viewport.offset.x + canvas_offset.x,
                    block.position.y * self.workflow.viewport.zoom + self.workflow.viewport.offset.y + canvas_offset.y,
                );
                let screen_size = egui::Vec2::new(
                    block.size.x * self.workflow.viewport.zoom,
                    block.size.y * self.workflow.viewport.zoom,
                );
                let block_rect = egui::Rect::from_min_size(screen_pos, screen_size);

                // 只渲染可见区域内的Block
                if block_rect.intersects(viewport_rect) {
                    if let Some(def) = self.registry.get(&block.script_id) {
                        BlockWidget::draw(&painter, block, def, &self.workflow.viewport, canvas_offset);
                    }
                }
            }

            // 显示Block名称编辑框
            if let InteractionState::EditingBlockName { block_id, ref mut edit_text } = &mut self.state {
                if let Some(block) = self.workflow.blocks.get(block_id) {
                    let pos = self.workflow.viewport.canvas_to_screen(block.position);
                    let screen_pos = Pos2::new(pos.x + canvas_offset.x + 4.0, pos.y + canvas_offset.y + 2.0);
                    let width = block.size.x * self.workflow.viewport.zoom - 8.0;

                    egui::Area::new(egui::Id::new("block_name_edit"))
                        .fixed_pos(screen_pos)
                        .order(egui::Order::Foreground)
                        .show(&response.ctx, |ui| {
                            let resp = ui.add(
                                egui::TextEdit::singleline(edit_text)
                                    .desired_width(width)
                                    .font(egui::FontId::proportional(12.0 * self.workflow.viewport.zoom))
                            );

                            // 自动获取焦点
                            if !resp.has_focus() {
                                resp.request_focus();
                            }

                            // Enter确认或失去焦点保存
                            let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
                            let escape_pressed = ui.input(|i| i.key_pressed(egui::Key::Escape));

                            if enter_pressed || escape_pressed || (resp.lost_focus() && !resp.has_focus()) {
                                // 这里不能直接修改，标记需要保存
                            }
                        });
                }
            }

            // 绘制正在拖拽的临时连接
            if let InteractionState::DraggingConnection { ref from, mouse_pos } = self.state {
                if let Some(block) = self.workflow.blocks.get(&from.block_id) {
                    let port_pos = BlockWidget::get_port_screen_pos(
                        block, from.port_index, from.is_output, &self.workflow.viewport, canvas_offset
                    );
                    if from.is_output {
                        ConnectionWidget::draw(&painter, port_pos, mouse_pos, true);
                    } else {
                        ConnectionWidget::draw(&painter, mouse_pos, port_pos, true);
                    }
                }
            }

            // 绘制框选矩形
            if let InteractionState::BoxSelecting { start } = self.state {
                if let Some(end) = self.box_select_end {
                    let rect = egui::Rect::from_two_pos(start, end);
                    painter.rect_filled(rect, 0.0, egui::Color32::from_rgba_unmultiplied(100, 150, 255, 30));
                    painter.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 150, 255)));
                }
            }

            // 处理交互
            self.handle_canvas_interaction(&response, canvas_offset);

            // 执行脏Block
            if !self.workflow.dirty_blocks.is_empty() {
                if let Err(e) = self.executor.execute_dirty(&mut self.workflow, &self.registry) {
                    log::error!("执行错误: {}", e);
                }
            }

            // 衰减激活状态（每帧调用，约60fps时0.05表示约20帧淡出）
            self.workflow.decay_activation(0.03);
        });

        // 右键菜单
        self.show_context_menu(ctx);

        // 请求持续重绘
        ctx.request_repaint();
    }
}

impl WorkflowApp {
    /// 显示右键菜单
    fn show_context_menu(&mut self, ctx: &Context) {
        if self.context_menu_pos.is_none() {
            return;
        }

        let menu_pos = self.context_menu_pos.unwrap();
        let target = self.context_menu_target.clone();

        egui::Area::new(egui::Id::new("context_menu"))
            .fixed_pos(menu_pos)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.set_min_width(120.0);

                    let readonly = self.workflow.readonly;

                    match target {
                        ContextMenuTarget::Block(_) => {
                            if ui.button("📋 复制 (Ctrl+C)").clicked() {
                                self.copy_selected();
                                self.context_menu_pos = None;
                            }
                            if !readonly {
                                if ui.button("📥 粘贴 (Ctrl+V)").clicked() {
                                    self.paste_at_cursor();
                                    self.context_menu_pos = None;
                                }
                                ui.separator();
                                if ui.button("🗑 删除 (Delete)").clicked() {
                                    self.delete_selected();
                                    self.context_menu_pos = None;
                                }
                            }
                        }
                        ContextMenuTarget::Connection(_) => {
                            if !readonly {
                                if ui.button("🗑 删除连线").clicked() {
                                    self.delete_selected();
                                    self.context_menu_pos = None;
                                }
                            } else {
                                ui.label("🔒 只读模式");
                            }
                        }
                        ContextMenuTarget::Canvas => {
                            if !readonly {
                                if ui.button("📥 粘贴 (Ctrl+V)").clicked() {
                                    self.paste_at_cursor();
                                    self.context_menu_pos = None;
                                }
                            }
                            if ui.button("🔍 全选 (Ctrl+A)").clicked() {
                                for block in self.workflow.blocks.values_mut() {
                                    block.selected = true;
                                }
                                self.context_menu_pos = None;
                            }
                        }
                    }
                });
            });

        // 点击其他区域关闭菜单
        if ctx.input(|i| i.pointer.any_click()) {
            let click_pos = ctx.input(|i| i.pointer.interact_pos());
            if let Some(pos) = click_pos {
                let menu_rect = egui::Rect::from_min_size(menu_pos, egui::vec2(150.0, 100.0));
                if !menu_rect.contains(pos) {
                    self.context_menu_pos = None;
                }
            }
        }
    }

    /// 复制选中的Block
    fn copy_selected(&mut self) {
        let selected: Vec<_> = self.workflow
            .selected_blocks()
            .iter()
            .filter_map(|id| self.workflow.blocks.get(id))
            .collect();
        let connections: Vec<_> = self.workflow.connections.values().collect();
        self.clipboard.copy(&selected, &connections);
        self.add_log("INFO", format!("已复制 {} 个Block", selected.len()));
    }

    /// 粘贴到当前位置
    fn paste_at_cursor(&mut self) {
        if self.workflow.readonly {
            self.add_log("WARN", "只读模式，无法粘贴".to_string());
            return;
        }
        self.save_undo_snapshot();

        let offset = Vec2::new(50.0, 50.0);
        let (blocks, connections) = self.clipboard.paste(offset);
        let count = blocks.len();
        self.workflow.clear_selection();
        for mut block in blocks {
            block.selected = true;
            self.workflow.add_block(block);
        }
        for conn in connections {
            self.workflow.add_connection(conn);
        }
        if count > 0 {
            self.add_log("INFO", format!("已粘贴 {} 个Block", count));
        }
    }

    /// 删除选中的Block和连线
    fn delete_selected(&mut self) {
        if self.workflow.readonly {
            self.add_log("WARN", "只读模式，无法删除".to_string());
            return;
        }

        let selected_blocks: Vec<_> = self.workflow.selected_blocks();
        let has_selection = !selected_blocks.is_empty() || !self.selected_connections.is_empty();
        if has_selection {
            self.save_undo_snapshot();
        }
        for id in &selected_blocks {
            self.workflow.remove_block(*id);
        }

        let selected_conns: Vec<_> = self.selected_connections.drain().collect();
        for conn_id in &selected_conns {
            self.workflow.remove_connection(*conn_id);
        }

        if !selected_blocks.is_empty() || !selected_conns.is_empty() {
            self.add_log("INFO", format!(
                "删除: {} Block, {} 连线",
                selected_blocks.len(),
                selected_conns.len()
            ));
        }
    }

    fn handle_canvas_interaction(&mut self, response: &egui::Response, canvas_offset: Pos2) {
        let pointer_pos = response.hover_pos().unwrap_or(Pos2::ZERO);
        let canvas_pos = Canvas::pos2_to_vec2(pointer_pos, &self.workflow.viewport, canvas_offset);

        // 检测空格键状态
        response.ctx.input(|i| {
            if i.key_pressed(Key::Space) {
                self.space_pressed = true;
            }
            if i.key_released(Key::Space) {
                self.space_pressed = false;
            }
        });

        // 触控板和滚轮处理
        if response.hovered() {
            let (scroll_delta, modifiers, zoom_delta, multi_touch) = response.ctx.input(|i| {
                (i.raw_scroll_delta, i.modifiers, i.zoom_delta(), i.multi_touch())
            });

            // 1. 优先处理捏合缩放手势（触控板双指捏合）
            if (zoom_delta - 1.0).abs() > 0.001 {
                let old_zoom = self.workflow.viewport.zoom;
                self.workflow.viewport.zoom *= zoom_delta;
                self.workflow.viewport.clamp_zoom();

                let zoom_ratio = self.workflow.viewport.zoom / old_zoom;
                self.workflow.viewport.offset.x = pointer_pos.x - canvas_offset.x
                    - (pointer_pos.x - canvas_offset.x - self.workflow.viewport.offset.x) * zoom_ratio;
                self.workflow.viewport.offset.y = pointer_pos.y - canvas_offset.y
                    - (pointer_pos.y - canvas_offset.y - self.workflow.viewport.offset.y) * zoom_ratio;
            }
            // 2. 多点触控缩放（备用方案）
            else if let Some(touch) = multi_touch {
                if (touch.zoom_delta - 1.0).abs() > 0.001 {
                    let old_zoom = self.workflow.viewport.zoom;
                    self.workflow.viewport.zoom *= touch.zoom_delta;
                    self.workflow.viewport.clamp_zoom();

                    let zoom_ratio = self.workflow.viewport.zoom / old_zoom;
                    self.workflow.viewport.offset.x = pointer_pos.x - canvas_offset.x
                        - (pointer_pos.x - canvas_offset.x - self.workflow.viewport.offset.x) * zoom_ratio;
                    self.workflow.viewport.offset.y = pointer_pos.y - canvas_offset.y
                        - (pointer_pos.y - canvas_offset.y - self.workflow.viewport.offset.y) * zoom_ratio;
                }
            }
            // 3. Command/Ctrl + 滚轮 = 缩放
            else if (modifiers.command || modifiers.ctrl) && scroll_delta.y != 0.0 {
                let zoom_factor = 1.0 + scroll_delta.y * 0.002;
                let old_zoom = self.workflow.viewport.zoom;
                self.workflow.viewport.zoom *= zoom_factor;
                self.workflow.viewport.clamp_zoom();

                let zoom_ratio = self.workflow.viewport.zoom / old_zoom;
                self.workflow.viewport.offset.x = pointer_pos.x - canvas_offset.x
                    - (pointer_pos.x - canvas_offset.x - self.workflow.viewport.offset.x) * zoom_ratio;
                self.workflow.viewport.offset.y = pointer_pos.y - canvas_offset.y
                    - (pointer_pos.y - canvas_offset.y - self.workflow.viewport.offset.y) * zoom_ratio;
            }
            // 4. 双指滑动平移（无修饰键）
            else if !modifiers.command && !modifiers.ctrl && (scroll_delta.x != 0.0 || scroll_delta.y != 0.0) {
                self.workflow.viewport.offset.x += scroll_delta.x;
                self.workflow.viewport.offset.y += scroll_delta.y;
            }
        }

        // 中键平移 或 空格+左键平移
        let is_panning = response.dragged_by(egui::PointerButton::Middle)
            || (self.space_pressed && response.dragged_by(egui::PointerButton::Primary));
        if is_panning {
            let delta = response.drag_delta();
            self.workflow.viewport.offset.x += delta.x;
            self.workflow.viewport.offset.y += delta.y;
            return; // 平移时不处理其他交互
        }

        // ESC取消当前操作
        if response.ctx.input(|i| i.key_pressed(Key::Escape)) {
            if !matches!(self.state, InteractionState::Idle) {
                self.state = InteractionState::Idle;
                return;
            }
        }

        // 右键菜单
        if response.clicked_by(egui::PointerButton::Secondary) {
            if !matches!(self.state, InteractionState::Idle) {
                // 取消当前操作
                self.state = InteractionState::Idle;
            } else {
                // 检测右键点击目标
                self.context_menu_pos = Some(pointer_pos);

                // 先检测Block
                let mut hit_block = None;
                for (id, block) in &self.workflow.blocks {
                    if block.contains(canvas_pos) {
                        hit_block = Some(*id);
                        break;
                    }
                }

                if let Some(block_id) = hit_block {
                    self.context_menu_target = ContextMenuTarget::Block(block_id);
                    // 如果点击的Block未选中，单选它
                    if !self.workflow.blocks.get(&block_id).map(|b| b.selected).unwrap_or(false) {
                        self.workflow.clear_selection();
                        if let Some(b) = self.workflow.blocks.get_mut(&block_id) {
                            b.selected = true;
                        }
                    }
                } else if let Some(conn_id) = self.find_connection_at(pointer_pos, canvas_offset) {
                    self.context_menu_target = ContextMenuTarget::Connection(conn_id);
                    self.selected_connections.clear();
                    self.selected_connections.insert(conn_id);
                } else {
                    self.context_menu_target = ContextMenuTarget::Canvas;
                }
            }
        }

        // 从菜单拖入Block（只读模式禁用）
        if let InteractionState::DraggingFromMenu(ref script_id) = self.state.clone() {
            // 检测鼠标释放（拖拽结束）
            let released = response.ctx.input(|i| {
                i.pointer.any_released() || !i.pointer.any_down()
            });

            if released {
                // 只读模式禁止添加
                if self.workflow.readonly {
                    self.add_log("WARN", "只读模式，无法添加Block".to_string());
                } else if let Some(pos) = response.ctx.pointer_hover_pos() {
                    if response.rect.contains(pos) {
                        // 先克隆定义，避免借用冲突
                        let def_opt = self.registry.get(&script_id).cloned();
                        if let Some(def) = def_opt {
                            self.save_undo_snapshot();
                            let name = def.meta.name.clone();
                            let block = Block::new(&def, canvas_pos);
                            self.workflow.add_block(block);
                            self.add_log("INFO", format!("添加Block: {}", name));
                        }
                    }
                }
                self.state = InteractionState::Idle;
            }
        }

        // 左键按下 - 开始拖拽
        if response.drag_started_by(egui::PointerButton::Primary) {
            let modifiers = response.ctx.input(|i| i.modifiers);

            // 先检测端口碰撞
            if let Some(port_hit) = self.find_port_at(pointer_pos, canvas_offset) {
                self.state = InteractionState::DraggingConnection {
                    from: port_hit,
                    mouse_pos: pointer_pos,
                };
            } else {
                // 检测Block碰撞
                let mut hit_block = None;
                for (id, block) in &self.workflow.blocks {
                    if block.contains(canvas_pos) {
                        hit_block = Some(*id);
                        break;
                    }
                }

                if let Some(id) = hit_block {
                    let is_multi_select = modifiers.ctrl || modifiers.command;
                    let was_selected = self.workflow.blocks.get(&id).map(|b| b.selected).unwrap_or(false);

                    if is_multi_select {
                        // Ctrl/Cmd+点击：切换选中状态
                        if let Some(block) = self.workflow.blocks.get_mut(&id) {
                            block.selected = !block.selected;
                        }
                    } else if !was_selected {
                        // 点击未选中的Block：清除其他选择，选中这个
                        self.workflow.clear_selection();
                        self.selected_connections.clear();
                        if let Some(block) = self.workflow.blocks.get_mut(&id) {
                            block.selected = true;
                        }
                    }
                    // 如果已选中，不做任何操作（允许拖拽多个）
                    self.state = InteractionState::DraggingBlock(id);
                } else {
                    // 检测连线碰撞
                    let hit_conn = self.find_connection_at(pointer_pos, canvas_offset);
                    if let Some(conn_id) = hit_conn {
                        let is_multi_select = modifiers.ctrl || modifiers.command;
                        if is_multi_select {
                            if self.selected_connections.contains(&conn_id) {
                                self.selected_connections.remove(&conn_id);
                            } else {
                                self.selected_connections.insert(conn_id);
                            }
                        } else {
                            self.selected_connections.clear();
                            self.selected_connections.insert(conn_id);
                        }
                        self.workflow.clear_selection();
                    } else {
                        // 点击空白：开始框选（松开时如果没拖动则清除选择）
                        self.state = InteractionState::BoxSelecting { start: pointer_pos };
                        self.box_select_end = Some(pointer_pos);
                    }
                }
            }
        }

        // 左键单击（无拖拽）- 处理选择
        if response.clicked_by(egui::PointerButton::Primary) {
            let modifiers = response.ctx.input(|i| i.modifiers);
            let is_multi_select = modifiers.ctrl || modifiers.command;

            // 检测Block碰撞
            let mut hit_block = None;
            for (id, block) in &self.workflow.blocks {
                if block.contains(canvas_pos) {
                    hit_block = Some(*id);
                    break;
                }
            }

            if let Some(id) = hit_block {
                let was_selected = self.workflow.blocks.get(&id).map(|b| b.selected).unwrap_or(false);

                if is_multi_select {
                    // Ctrl/Cmd+点击：切换选中状态
                    if let Some(block) = self.workflow.blocks.get_mut(&id) {
                        block.selected = !block.selected;
                    }
                } else {
                    // 普通点击：只选中这个Block
                    self.workflow.clear_selection();
                    self.selected_connections.clear();
                    if let Some(block) = self.workflow.blocks.get_mut(&id) {
                        block.selected = true;
                    }
                }
            } else {
                // 检测连线碰撞
                let hit_conn = self.find_connection_at(pointer_pos, canvas_offset);
                if let Some(conn_id) = hit_conn {
                    if is_multi_select {
                        if self.selected_connections.contains(&conn_id) {
                            self.selected_connections.remove(&conn_id);
                        } else {
                            self.selected_connections.insert(conn_id);
                        }
                    } else {
                        self.selected_connections.clear();
                        self.selected_connections.insert(conn_id);
                        self.workflow.clear_selection();
                    }
                } else {
                    // 点击空白：清除所有选择
                    self.workflow.clear_selection();
                    self.selected_connections.clear();
                }
            }
        }

        // 双击Block名称 - 开始编辑（只读模式禁止）
        if response.double_clicked_by(egui::PointerButton::Primary) && !self.workflow.readonly {
            // 检测Block碰撞
            for (id, block) in &self.workflow.blocks {
                if block.contains(canvas_pos) {
                    // 检测是否点击在标题区域（Block顶部28像素）
                    let header_height = 28.0;
                    let block_top = block.position.y;
                    if canvas_pos.y <= block_top + header_height {
                        // 获取当前显示名称
                        let current_name = if let Some(def) = self.registry.get(&block.script_id) {
                            block.display_name(def).to_string()
                        } else {
                            block.custom_name.clone().unwrap_or_default()
                        };
                        self.state = InteractionState::EditingBlockName {
                            block_id: *id,
                            edit_text: current_name,
                        };
                    }
                    break;
                }
            }
        }

        // 拖拽Block
        if let InteractionState::DraggingBlock(_) = self.state {
            if response.dragged_by(egui::PointerButton::Primary) {
                // 只读模式禁止移动Block
                if !self.workflow.readonly {
                    let delta = response.drag_delta();
                    // 只有真正移动时才保存快照（避免点击也保存）
                    if delta.x.abs() > 1.0 || delta.y.abs() > 1.0 {
                        self.save_undo_snapshot();
                    }
                    let scale_delta = Vec2::new(
                        delta.x / self.workflow.viewport.zoom,
                        delta.y / self.workflow.viewport.zoom,
                    );
                    for block in self.workflow.blocks.values_mut() {
                        if block.selected {
                            block.position.x += scale_delta.x;
                            block.position.y += scale_delta.y;
                        }
                    }
                }
            }
        }

        // 拖拽连接 - 更新鼠标位置
        if let InteractionState::DraggingConnection { ref mut mouse_pos, .. } = self.state {
            *mouse_pos = pointer_pos;
        }

        // 框选拖拽 - 更新结束位置
        if let InteractionState::BoxSelecting { .. } = self.state {
            self.box_select_end = Some(pointer_pos);
        }

        // 释放
        if response.drag_stopped() {
            match &self.state {
                InteractionState::DraggingBlock(_) => {
                    const GRID_SIZE: f32 = 20.0;
                    for block in self.workflow.blocks.values_mut() {
                        if block.selected {
                            block.snap_to_grid(GRID_SIZE);
                        }
                    }
                }
                InteractionState::BoxSelecting { start } => {
                    // 框选完成，选中框内的Block和连线
                    if let Some(end) = self.box_select_end {
                        let min_x = start.x.min(end.x);
                        let max_x = start.x.max(end.x);
                        let min_y = start.y.min(end.y);
                        let max_y = start.y.max(end.y);
                        let rect_min = Pos2::new(min_x, min_y);
                        let rect_max = Pos2::new(max_x, max_y);

                        // 选中框内的Block
                        for block in self.workflow.blocks.values_mut() {
                            let block_screen = Pos2::new(
                                block.position.x * self.workflow.viewport.zoom + self.workflow.viewport.offset.x + self.canvas_rect.min.x,
                                block.position.y * self.workflow.viewport.zoom + self.workflow.viewport.offset.y + self.canvas_rect.min.y,
                            );
                            let block_end = Pos2::new(
                                block_screen.x + block.size.x * self.workflow.viewport.zoom,
                                block_screen.y + block.size.y * self.workflow.viewport.zoom,
                            );

                            // 检查Block是否与框选区域相交
                            if block_screen.x < max_x && block_end.x > min_x &&
                               block_screen.y < max_y && block_end.y > min_y {
                                block.selected = true;
                            }
                        }

                        // 选中框内的连线
                        let conn_hits: Vec<Uuid> = self.workflow.connections.iter()
                            .filter_map(|(conn_id, conn)| {
                                if let (Some(from_block), Some(to_block)) = (
                                    self.workflow.blocks.get(&conn.from_block),
                                    self.workflow.blocks.get(&conn.to_block),
                                ) {
                                    if let Some(from_def) = self.registry.get(&from_block.script_id) {
                                        if let Some(to_def) = self.registry.get(&to_block.script_id) {
                                            let from_idx = from_def.outputs.iter()
                                                .position(|p| p.id == conn.from_port)
                                                .unwrap_or(0);
                                            let to_idx = to_def.inputs.iter()
                                                .position(|p| p.id == conn.to_port)
                                                .unwrap_or(0);

                                            let from_pos = BlockWidget::get_port_screen_pos(
                                                from_block, from_idx, true, &self.workflow.viewport, canvas_offset
                                            );
                                            let to_pos = BlockWidget::get_port_screen_pos(
                                                to_block, to_idx, false, &self.workflow.viewport, canvas_offset
                                            );

                                            if ConnectionWidget::intersects_rect(from_pos, to_pos, rect_min, rect_max) {
                                                return Some(*conn_id);
                                            }
                                        }
                                    }
                                }
                                None
                            })
                            .collect();

                        for conn_id in conn_hits {
                            self.selected_connections.insert(conn_id);
                        }
                    }
                    self.box_select_end = None;
                }
                InteractionState::DraggingConnection { from, .. } => {
                    // 克隆数据避免借用冲突
                    let from = from.clone();

                    // 只读模式禁止创建连线
                    if self.workflow.readonly {
                        self.add_log("WARN", "只读模式，无法创建连线".to_string());
                    } else if let Some(to_port) = self.find_port_at(pointer_pos, canvas_offset) {
                        let mut log_msg: Option<String> = None;
                        // 确保连接方向正确：output -> input
                        if from.is_output && !to_port.is_output && from.block_id != to_port.block_id {
                            self.save_undo_snapshot();
                            let conn = Connection::new(
                                from.block_id,
                                from.port_id.clone(),
                                to_port.block_id,
                                to_port.port_id.clone(),
                            );
                            self.workflow.add_connection(conn);
                            log_msg = Some(format!("连接: {} -> {}", from.port_id, to_port.port_id));
                        } else if !from.is_output && to_port.is_output && from.block_id != to_port.block_id {
                            self.save_undo_snapshot();
                            let conn = Connection::new(
                                to_port.block_id,
                                to_port.port_id.clone(),
                                from.block_id,
                                from.port_id.clone(),
                            );
                            self.workflow.add_connection(conn);
                            log_msg = Some(format!("连接: {} -> {}", to_port.port_id, from.port_id));
                        }
                        if let Some(msg) = log_msg {
                            self.add_log("INFO", msg);
                        }
                    }
                }
                _ => {}
            }
            self.state = InteractionState::Idle;
        }
    }

    /// 在指定屏幕位置查找端口
    fn find_port_at(&self, screen_pos: Pos2, canvas_offset: Pos2) -> Option<DraggingPort> {
        const PORT_HIT_RADIUS: f32 = 12.0;

        for (block_id, block) in &self.workflow.blocks {
            if let Some(def) = self.registry.get(&block.script_id) {
                // 检查输入端口
                for (i, input) in def.inputs.iter().enumerate() {
                    let port_pos = BlockWidget::get_port_screen_pos(
                        block, i, false, &self.workflow.viewport, canvas_offset
                    );
                    let dist = ((screen_pos.x - port_pos.x).powi(2) + (screen_pos.y - port_pos.y).powi(2)).sqrt();
                    if dist < PORT_HIT_RADIUS * self.workflow.viewport.zoom {
                        return Some(DraggingPort {
                            block_id: *block_id,
                            port_id: input.id.clone(),
                            is_output: false,
                            port_index: i,
                        });
                    }
                }
                // 检查输出端口
                for (i, output) in def.outputs.iter().enumerate() {
                    let port_pos = BlockWidget::get_port_screen_pos(
                        block, i, true, &self.workflow.viewport, canvas_offset
                    );
                    let dist = ((screen_pos.x - port_pos.x).powi(2) + (screen_pos.y - port_pos.y).powi(2)).sqrt();
                    if dist < PORT_HIT_RADIUS * self.workflow.viewport.zoom {
                        return Some(DraggingPort {
                            block_id: *block_id,
                            port_id: output.id.clone(),
                            is_output: true,
                            port_index: i,
                        });
                    }
                }
            }
        }
        None
    }

    /// 在指定屏幕位置查找连线
    fn find_connection_at(&self, screen_pos: Pos2, canvas_offset: Pos2) -> Option<Uuid> {
        const HIT_DISTANCE: f32 = 10.0;

        for (conn_id, conn) in &self.workflow.connections {
            let (from_block, to_block) = match (
                self.workflow.blocks.get(&conn.from_block),
                self.workflow.blocks.get(&conn.to_block),
            ) {
                (Some(f), Some(t)) => (f, t),
                _ => continue,
            };

            let (from_def, to_def) = match (
                self.registry.get(&from_block.script_id),
                self.registry.get(&to_block.script_id),
            ) {
                (Some(f), Some(t)) => (f, t),
                _ => continue,
            };

            let from_idx = from_def.outputs.iter()
                .position(|p| p.id == conn.from_port)
                .unwrap_or(0);
            let to_idx = to_def.inputs.iter()
                .position(|p| p.id == conn.to_port)
                .unwrap_or(0);

            let from_pos = BlockWidget::get_port_screen_pos(
                from_block, from_idx, true, &self.workflow.viewport, canvas_offset
            );
            let to_pos = BlockWidget::get_port_screen_pos(
                to_block, to_idx, false, &self.workflow.viewport, canvas_offset
            );

            // 使用 ConnectionWidget 的碰撞检测（支持折线和曲线模式）
            if ConnectionWidget::hit_test(from_pos, to_pos, screen_pos, HIT_DISTANCE) {
                return Some(*conn_id);
            }
        }
        None
    }

    /// 打开文件对话框
    fn open_file_dialog(&mut self) {
        let file = rfd::FileDialog::new()
            .add_filter("蓝图文件", &["L", "LZ", "l", "lz"])
            .add_filter("明文蓝图", &["L", "l"])
            .add_filter("加密蓝图", &["LZ", "lz"])
            .set_directory(std::env::current_dir().unwrap_or_default())
            .pick_file();

        if let Some(path) = file {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if ext.eq_ignore_ascii_case("lz") {
                self.pending_operation = Some(FileOperation::Load(path));
                self.show_password_dialog = true;
            } else {
                self.load_workflow_file(&path, None);
            }
        }
    }

    /// 加载工作流文件
    fn load_workflow_file(&mut self, path: &std::path::Path, password: Option<&str>) {
        match BlueprintStorage::load(path, password) {
            Ok(mut wf) => {
                wf.update_execution_order();
                self.workflow = wf;
                self.add_log("INFO", format!("已加载: {}", path.display()));
                self.current_file_path = Some(path.to_path_buf());
            }
            Err(e) => {
                self.add_log("ERROR", format!("加载失败: {}", e));
            }
        }
    }

    /// 保存文件对话框
    fn save_file_dialog(&mut self) {
        let ext = if self.save_options.encrypted { "LZ" } else { "L" };
        let default_name = self.workflow.name.clone() + "." + ext;

        let file = rfd::FileDialog::new()
            .add_filter("蓝图文件", &[ext])
            .set_file_name(&default_name)
            .set_directory(std::env::current_dir().unwrap_or_default())
            .save_file();

        if let Some(path) = file {
            if self.save_options.encrypted {
                if self.save_options.dual_save {
                    self.pending_operation = Some(FileOperation::SaveDual(path));
                } else {
                    self.pending_operation = Some(FileOperation::Save(path));
                }
                self.show_password_dialog = true;
            } else {
                self.save_workflow_file(&path, None);
            }
        }
    }

    /// 保存工作流文件
    fn save_workflow_file(&mut self, path: &std::path::Path, password: Option<&str>) {
        let mut workflow = self.workflow.clone();
        workflow.readonly = self.save_options.readonly;

        if self.save_options.dual_save {
            let base_name = path.with_extension("").to_string_lossy().to_string();
            match BlueprintStorage::save_dual(&workflow, &base_name, password.is_some(), password) {
                Ok((edit_path, dist_path)) => {
                    self.add_log("INFO", format!("可编辑: {}", edit_path.display()));
                    self.add_log("INFO", format!("可分发: {}", dist_path.display()));
                    self.current_file_path = Some(edit_path);
                }
                Err(e) => self.add_log("ERROR", format!("保存失败: {}", e)),
            }
        } else {
            match BlueprintStorage::save(&workflow, path, password) {
                Ok(()) => {
                    self.add_log("INFO", format!("已保存: {}", path.display()));
                    self.current_file_path = Some(path.to_path_buf());
                }
                Err(e) => self.add_log("ERROR", format!("保存失败: {}", e)),
            }
        }
    }

    fn draw_save_dialog(&mut self, ctx: &Context) {
        if !self.show_save_dialog {
            return;
        }

        egui::Window::new("💾 保存蓝图")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.heading("保存选项");
                ui.add_space(8.0);

                // 加密选项
                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.save_options.encrypted, false, "📄 明文 (.L)");
                    ui.radio_value(&mut self.save_options.encrypted, true, "🔒 加密 (.LZ)");
                });

                ui.add_space(4.0);

                // 只读选项
                ui.checkbox(&mut self.save_options.readonly, "📛 只读模式（不可编辑）");

                // 双份保存选项
                ui.checkbox(&mut self.save_options.dual_save, "📦 双份保存（可编辑 + 可分发）");

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("📁 选择位置并保存").clicked() {
                        self.show_save_dialog = false;
                        self.save_file_dialog();
                    }

                    if ui.button("取消").clicked() {
                        self.show_save_dialog = false;
                    }
                });
            });
    }

    fn draw_password_dialog(&mut self, ctx: &Context) {
        if !self.show_password_dialog {
            return;
        }

        egui::Window::new("🔐 输入密码")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("密码 (最长32位):");
                ui.add(egui::TextEdit::singleline(&mut self.password_input).password(true));

                if self.password_input.len() > 32 {
                    ui.colored_label(egui::Color32::RED, "密码不能超过32位!");
                }

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    let valid = !self.password_input.is_empty() && self.password_input.len() <= 32;

                    if ui.add_enabled(valid, egui::Button::new("确定")).clicked() {
                        let password = self.password_input.clone();
                        if let Some(op) = self.pending_operation.take() {
                            match op {
                                FileOperation::Save(path) => {
                                    self.save_workflow_file(&path, Some(&password));
                                }
                                FileOperation::SaveDual(path) => {
                                    self.save_workflow_file(&path, Some(&password));
                                }
                                FileOperation::Load(path) => {
                                    self.load_workflow_file(&path, Some(&password));
                                }
                            }
                        }
                        self.show_password_dialog = false;
                        self.password_input.clear();
                    }

                    if ui.button("取消").clicked() {
                        self.show_password_dialog = false;
                        self.password_input.clear();
                        self.pending_operation = None;
                    }
                });
            });
    }
}
