//! 侧边分类菜单

use crate::script::ScriptRegistry;
use egui::{Color32, CursorIcon, Id, LayerId, Order, ScrollArea, Sense, Ui};

/// 侧边菜单
pub struct SideMenu;

/// 菜单事件
pub enum MenuEvent {
    /// 开始拖拽Block
    DragBlock(String), // script_id
}

impl SideMenu {
    /// 绘制侧边菜单
    pub fn draw(ui: &mut Ui, registry: &ScriptRegistry) -> Option<MenuEvent> {
        let mut event = None;

        ui.heading("Block库");
        ui.separator();

        ScrollArea::vertical().show(ui, |ui| {
            // 按分类显示
            let mut categories: Vec<_> = registry.categories().collect();
            categories.sort_by(|a, b| a.0.cmp(b.0));

            for (category, _) in categories {
                ui.collapsing(category, |ui| {
                    for def in registry.get_by_category(category) {
                        let id = Id::new(&def.meta.id);

                        // 使用可拖拽的Label
                        let response = ui.add(
                            egui::Label::new(&def.meta.name)
                                .sense(Sense::click_and_drag())
                        ).on_hover_text(&def.meta.description);

                        // 开始拖拽
                        if response.drag_started() {
                            event = Some(MenuEvent::DragBlock(def.meta.id.clone()));
                        }

                        // 拖拽中 - 显示预览
                        if response.dragged() {
                            ui.ctx().set_cursor_icon(CursorIcon::Grabbing);

                            // 在鼠标位置绘制拖拽预览
                            if let Some(pointer_pos) = ui.ctx().pointer_hover_pos() {
                                let layer = LayerId::new(Order::Tooltip, id);
                                let painter = ui.ctx().layer_painter(layer);

                                let text = format!("📦 {}", def.meta.name);
                                painter.text(
                                    pointer_pos + egui::vec2(10.0, 10.0),
                                    egui::Align2::LEFT_TOP,
                                    text,
                                    egui::FontId::proportional(14.0),
                                    Color32::WHITE,
                                );
                            }
                        }
                    }
                });
            }
        });

        event
    }
}

