//! 图层面板 - 画布区域快捷跳转

use crate::workflow::Workflow;
use egui::{Color32, RichText, ScrollArea, Ui};

/// 图层面板事件
#[derive(Debug, Clone)]
pub enum LayerEvent {
    /// 跳转到图层
    GotoLayer(usize),
    /// 新建图层
    CreateLayer,
    /// 删除图层
    DeleteLayer(usize),
    /// 开始重命名
    StartRename(usize),
}

/// 图层面板
pub struct LayerPanel;

impl LayerPanel {
    /// 绘制图层面板
    pub fn draw(ui: &mut Ui, workflow: &Workflow, editing_layer: &mut Option<(usize, String)>) -> Option<LayerEvent> {
        let mut event = None;

        ui.horizontal(|ui| {
            ui.strong("📍 图层");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.small_button("➕").on_hover_text("新建图层").clicked() {
                    event = Some(LayerEvent::CreateLayer);
                }
            });
        });
        ui.separator();

        if workflow.layers.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("暂无图层").weak().size(11.0));
            });
            ui.add_space(4.0);
            if ui.button("创建第一个图层").clicked() {
                event = Some(LayerEvent::CreateLayer);
            }
        } else {
            ScrollArea::vertical()
                .max_height(200.0)
                .show(ui, |ui| {
                    for (index, layer) in workflow.layers.iter().enumerate() {
                        let is_current = workflow.current_layer_index == Some(index);
                        
                        ui.horizontal(|ui| {
                            // 当前图层指示器
                            if is_current {
                                ui.label(RichText::new("▶").color(Color32::from_rgb(100, 200, 100)));
                            } else {
                                ui.label("  ");
                            }

                            // 检查是否正在编辑此图层名称
                            if let Some((edit_index, ref mut edit_text)) = editing_layer {
                                if *edit_index == index {
                                    let response = ui.text_edit_singleline(edit_text);
                                    if response.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                        // 编辑完成，返回重命名事件
                                        // 实际重命名在app.rs中处理
                                    }
                                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                        *editing_layer = None;
                                    }
                                } else {
                                    Self::draw_layer_item(ui, layer, index, is_current, &mut event);
                                }
                            } else {
                                Self::draw_layer_item(ui, layer, index, is_current, &mut event);
                            }
                        });
                    }
                });
        }

        event
    }

    fn draw_layer_item(
        ui: &mut Ui, 
        layer: &crate::workflow::Layer, 
        index: usize, 
        is_current: bool,
        event: &mut Option<LayerEvent>
    ) {
        // 图层颜色指示器
        let color = Color32::from_rgb(layer.color[0], layer.color[1], layer.color[2]);
        ui.painter().rect_filled(
            egui::Rect::from_min_size(ui.cursor().min, egui::vec2(4.0, 16.0)),
            2.0,
            color,
        );
        ui.add_space(8.0);

        // 图层名称（可点击跳转）
        let name_text = if is_current {
            RichText::new(&layer.name).strong()
        } else {
            RichText::new(&layer.name)
        };

        let response = ui.selectable_label(is_current, name_text);
        if response.clicked() {
            *event = Some(LayerEvent::GotoLayer(index));
        }
        if response.double_clicked() {
            *event = Some(LayerEvent::StartRename(index));
        }

        // 右键菜单
        response.context_menu(|ui| {
            if ui.button("✏️ 重命名").clicked() {
                *event = Some(LayerEvent::StartRename(index));
                ui.close_menu();
            }
            if ui.button("🗑 删除").clicked() {
                *event = Some(LayerEvent::DeleteLayer(index));
                ui.close_menu();
            }
        });
    }
}

