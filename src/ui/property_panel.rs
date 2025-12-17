//! 属性编辑面板

use crate::script::{BlockDefinition, DataType, Value};
use crate::workflow::Block;
use egui::{DragValue, Ui};

/// 属性面板
pub struct PropertyPanel;

/// 属性变更事件
pub struct PropertyChange {
    pub property_id: String,
    pub new_value: Value,
}

impl PropertyPanel {
    /// 绘制属性面板
    pub fn draw(
        ui: &mut Ui,
        block: &Block,
        definition: &BlockDefinition,
    ) -> Vec<PropertyChange> {
        let mut changes = Vec::new();

        ui.heading(&definition.meta.name);
        ui.label(&definition.meta.description);
        ui.separator();

        ui.label("属性");
        
        for prop_def in &definition.properties {
            let current_value = block.properties.get(&prop_def.id);
            
            ui.horizontal(|ui| {
                ui.label(&prop_def.name);
                
                if let Some(change) = Self::draw_property_editor(
                    ui,
                    &prop_def.id,
                    &prop_def.data_type,
                    current_value,
                    prop_def.min,
                    prop_def.max,
                ) {
                    changes.push(change);
                }
            });
        }

        ui.separator();
        ui.label("输出值");
        
        for output_def in &definition.outputs {
            if let Some(value) = block.output_values.get(&output_def.id) {
                ui.horizontal(|ui| {
                    ui.label(&output_def.name);
                    ui.label(format!("{:?}", value));
                });
            }
        }

        changes
    }

    fn draw_property_editor(
        ui: &mut Ui,
        prop_id: &str,
        data_type: &DataType,
        current: Option<&Value>,
        min: Option<f64>,
        max: Option<f64>,
    ) -> Option<PropertyChange> {
        match data_type {
            DataType::Number => {
                let mut val = current
                    .and_then(|v| v.as_number())
                    .unwrap_or(0.0);
                
                let mut drag = DragValue::new(&mut val).speed(0.1);
                if let Some(min) = min {
                    drag = drag.range(min..=max.unwrap_or(f64::MAX));
                }
                
                if ui.add(drag).changed() {
                    return Some(PropertyChange {
                        property_id: prop_id.to_string(),
                        new_value: Value::Number(val),
                    });
                }
            }
            DataType::String => {
                let mut val = current
                    .and_then(|v| v.as_string())
                    .unwrap_or("")
                    .to_string();
                
                if ui.text_edit_singleline(&mut val).changed() {
                    return Some(PropertyChange {
                        property_id: prop_id.to_string(),
                        new_value: Value::String(val),
                    });
                }
            }
            DataType::Boolean => {
                let mut val = current
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                
                if ui.checkbox(&mut val, "").changed() {
                    return Some(PropertyChange {
                        property_id: prop_id.to_string(),
                        new_value: Value::Boolean(val),
                    });
                }
            }
            _ => {
                ui.label(format!("{:?}", current));
            }
        }
        None
    }
}

