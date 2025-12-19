//! WorkflowEngine - 可视化工作流引擎

mod app;
mod script;
mod ui;
mod usb;
mod workflow;

use app::WorkflowApp;
use egui::{FontData, FontDefinitions, FontFamily};
use std::path::PathBuf;

/// Windows 控制台 UTF-8 支持
#[cfg(target_os = "windows")]
fn setup_windows_console() {
    // 设置控制台代码页为 UTF-8
    unsafe {
        #[link(name = "kernel32")]
        extern "system" {
            fn SetConsoleOutputCP(code_page: u32) -> i32;
            fn SetConsoleCP(code_page: u32) -> i32;
        }
        SetConsoleOutputCP(65001); // UTF-8
        SetConsoleCP(65001);
    }
}

#[cfg(not(target_os = "windows"))]
fn setup_windows_console() {
    // 非 Windows 平台无需处理
}

/// 获取脚本目录 - 优先使用可执行文件所在目录
fn get_script_dir() -> PathBuf {
    // 尝试获取可执行文件所在目录
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let script_dir = exe_dir.join("scripts");
            if script_dir.exists() {
                return script_dir;
            }
            // macOS .app bundle: 检查 ../Resources/scripts
            let resources_dir = exe_dir.join("../Resources/scripts");
            if resources_dir.exists() {
                return resources_dir;
            }
        }
    }

    // 回退到当前工作目录
    let cwd_scripts = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("scripts");
    if cwd_scripts.exists() {
        return cwd_scripts;
    }

    // 最后尝试相对路径
    PathBuf::from("scripts")
}

/// 配置中文字体
fn setup_chinese_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // 跨平台中文字体路径
    #[cfg(target_os = "macos")]
    let font_paths: &[&str] = &[
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/Supplemental/Songti.ttc",
        "/Library/Fonts/Hiragino Sans GB.ttc",
        "/System/Library/Fonts/STHeiti Light.ttc",
        "/Library/Fonts/Arial Unicode.ttf",
    ];

    #[cfg(target_os = "windows")]
    let font_paths: &[&str] = &[
        "C:\\Windows\\Fonts\\msyh.ttc",      // 微软雅黑
        "C:\\Windows\\Fonts\\msyhbd.ttc",    // 微软雅黑粗体
        "C:\\Windows\\Fonts\\simsun.ttc",    // 宋体
        "C:\\Windows\\Fonts\\simhei.ttf",    // 黑体
        "C:\\Windows\\Fonts\\mingliub.ttc",  // 细明体
    ];

    #[cfg(target_os = "linux")]
    let font_paths: &[&str] = &[
        "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
        "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
    ];

    for path in font_paths {
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert(
                "chinese".to_owned(),
                FontData::from_owned(font_data),
            );

            // 添加到字体族首位
            if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
                family.insert(0, "chinese".to_owned());
            }
            if let Some(family) = fonts.families.get_mut(&FontFamily::Monospace) {
                family.insert(0, "chinese".to_owned());
            }

            log::info!("加载中文字体: {}", path);
            ctx.set_fonts(fonts);
            return;
        }
    }

    log::warn!("未能加载中文字体，界面可能显示方块");
}

fn main() -> eframe::Result<()> {
    // Windows 控制台 UTF-8 支持
    setup_windows_console();

    // 初始化日志
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 获取脚本目录
    let script_dir = get_script_dir();

    log::info!("脚本目录: {}", script_dir.display());
    log::info!("目录存在: {}", script_dir.exists());

    // 创建应用
    let app = match WorkflowApp::new(script_dir) {
        Ok(app) => app,
        Err(e) => {
            log::error!("初始化失败: {}", e);
            return Ok(());
        }
    };

    // 运行GUI
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "WorkflowEngine - 可视化工作流编辑器",
        options,
        Box::new(|cc| {
            // 配置中文字体
            setup_chinese_fonts(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}
