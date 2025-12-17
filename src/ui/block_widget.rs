//! Block渲染组件

use crate::script::{BlockDefinition, DataType, Value};
use crate::workflow::{Block, Viewport};
use egui::{Color32, FontId, Painter, Pos2, Rect, Rounding, Stroke, Vec2 as EguiVec2};

/// Block渲染器
pub struct BlockWidget;

impl BlockWidget {
    const HEADER_HEIGHT: f32 = 28.0;
    const PORT_HEIGHT: f32 = 22.0;
    const PORT_RADIUS: f32 = 5.0;
    const ROUNDING: f32 = 6.0;

    /// 绘制Block
    pub fn draw(
        painter: &Painter,
        block: &Block,
        definition: &BlockDefinition,
        viewport: &Viewport,
        canvas_offset: Pos2,
    ) {
        let pos = Self::block_screen_pos(block, viewport, canvas_offset);
        let size = EguiVec2::new(block.size.x * viewport.zoom, block.size.y * viewport.zoom);
        let rect = Rect::from_min_size(pos, size);

        // 解析颜色
        let header_color = Self::parse_color(&definition.meta.color);
        let body_color = Color32::from_rgb(40, 40, 44);
        let border_color = if block.selected {
            Color32::from_rgb(255, 100, 100)  // 红色选中边框
        } else {
            Color32::from_gray(70)
        };
        let border_width = if block.selected { 2.5 } else { 1.0 };

        // 绘制阴影
        if viewport.zoom > 0.5 {
            let shadow_offset = 3.0 * viewport.zoom;
            let shadow_rect = Rect::from_min_size(
                Pos2::new(pos.x + shadow_offset, pos.y + shadow_offset),
                size,
            );
            painter.rect_filled(
                shadow_rect,
                Rounding::same(Self::ROUNDING * viewport.zoom),
                Color32::from_black_alpha(40),
            );
        }

        // 绘制主体
        painter.rect_filled(rect, Rounding::same(Self::ROUNDING * viewport.zoom), body_color);

        // 绘制头部
        let header_rect = Rect::from_min_size(
            pos,
            EguiVec2::new(size.x, Self::HEADER_HEIGHT * viewport.zoom),
        );
        painter.rect_filled(
            header_rect,
            Rounding {
                nw: Self::ROUNDING * viewport.zoom,
                ne: Self::ROUNDING * viewport.zoom,
                sw: 0.0,
                se: 0.0,
            },
            header_color,
        );

        // 绘制边框
        painter.rect_stroke(
            rect,
            Rounding::same(Self::ROUNDING * viewport.zoom),
            Stroke::new(border_width, border_color),
        );

        // 绘制标题（使用自定义名称或定义名称）
        let display_name = block.display_name(definition);
        let title_pos = Pos2::new(pos.x + 8.0 * viewport.zoom, pos.y + 6.0 * viewport.zoom);
        painter.text(
            title_pos,
            egui::Align2::LEFT_TOP,
            display_name,
            FontId::proportional(12.0 * viewport.zoom),
            Color32::WHITE,
        );

        // 绘制端口
        let port_y_start = pos.y + Self::HEADER_HEIGHT * viewport.zoom;

        // 输入端口(左侧)
        for (i, input) in definition.inputs.iter().enumerate() {
            let y = port_y_start + (i as f32 + 0.5) * Self::PORT_HEIGHT * viewport.zoom;
            let port_pos = Pos2::new(pos.x, y);
            let value = block.input_values.get(&input.id);
            Self::draw_port(painter, port_pos, &input.name, value, &input.data_type, true, viewport.zoom);
        }

        // 输出端口(右侧)
        for (i, output) in definition.outputs.iter().enumerate() {
            let y = port_y_start + (i as f32 + 0.5) * Self::PORT_HEIGHT * viewport.zoom;
            let port_pos = Pos2::new(pos.x + size.x, y);
            let value = block.output_values.get(&output.id);
            Self::draw_port(painter, port_pos, &output.name, value, &output.data_type, false, viewport.zoom);
        }
    }

    /// 绘制端口
    fn draw_port(
        painter: &Painter,
        pos: Pos2,
        name: &str,
        value: Option<&Value>,
        data_type: &DataType,
        is_input: bool,
        zoom: f32,
    ) {
        let radius = Self::PORT_RADIUS * zoom;
        let port_color = Self::get_type_color(data_type);

        // 绘制端口圆圈
        painter.circle_filled(pos, radius, port_color);
        painter.circle_stroke(pos, radius, Stroke::new(1.0, Color32::from_gray(100)));

        // 端口名称
        let text_offset = if is_input { 10.0 } else { -10.0 } * zoom;
        let align = if is_input {
            egui::Align2::LEFT_CENTER
        } else {
            egui::Align2::RIGHT_CENTER
        };

        painter.text(
            Pos2::new(pos.x + text_offset, pos.y),
            align,
            name,
            FontId::proportional(10.0 * zoom),
            Color32::from_gray(220),
        );

        // 显示端口值(如果有且缩放足够大)
        if zoom > 0.6 {
            if let Some(val) = value {
                let val_str = Self::format_value(val);
                if !val_str.is_empty() {
                    let val_offset = if is_input { 10.0 } else { -10.0 } * zoom;
                    let val_y = pos.y + 10.0 * zoom;
                    painter.text(
                        Pos2::new(pos.x + val_offset, val_y),
                        align,
                        val_str,
                        FontId::proportional(8.0 * zoom),
                        Color32::from_gray(140),
                    );
                }
            }
        }
    }

    /// 获取数据类型对应的颜色
    fn get_type_color(data_type: &DataType) -> Color32 {
        match data_type {
            DataType::Number => Color32::from_rgb(100, 180, 255),  // 蓝色
            DataType::String => Color32::from_rgb(255, 200, 100),  // 橙色
            DataType::Boolean => Color32::from_rgb(255, 100, 100), // 红色
            DataType::Event => Color32::from_rgb(255, 255, 100),   // 黄色
            DataType::Array => Color32::from_rgb(180, 100, 255),   // 紫色
            DataType::Any => Color32::from_gray(200),              // 灰色
        }
    }

    /// 格式化值为字符串显示
    fn format_value(value: &Value) -> String {
        match value {
            Value::Nil => String::new(),
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{:.2}", n)
                }
            }
            Value::String(s) => {
                if s.len() > 8 {
                    format!("\"{}...\"", &s[..6])
                } else {
                    format!("\"{}\"", s)
                }
            }
            Value::Boolean(b) => if *b { "true" } else { "false" }.to_string(),
            Value::Array(arr) => format!("[{}]", arr.len()),
            Value::Object(map) => format!("{{{}}}", map.len()),
        }
    }

    /// 获取Block的屏幕位置
    fn block_screen_pos(block: &Block, viewport: &Viewport, canvas_offset: Pos2) -> Pos2 {
        let screen = viewport.canvas_to_screen(block.position);
        Pos2::new(screen.x + canvas_offset.x, screen.y + canvas_offset.y)
    }

    /// 解析颜色字符串
    fn parse_color(color_str: &str) -> Color32 {
        if color_str.starts_with('#') && color_str.len() == 7 {
            let r = u8::from_str_radix(&color_str[1..3], 16).unwrap_or(100);
            let g = u8::from_str_radix(&color_str[3..5], 16).unwrap_or(100);
            let b = u8::from_str_radix(&color_str[5..7], 16).unwrap_or(100);
            Color32::from_rgb(r, g, b)
        } else {
            Color32::from_rgb(100, 100, 100)
        }
    }

    /// 获取端口的屏幕位置
    pub fn get_port_screen_pos(
        block: &Block,
        port_index: usize,
        is_output: bool,
        viewport: &Viewport,
        canvas_offset: Pos2,
    ) -> Pos2 {
        let block_pos = Self::block_screen_pos(block, viewport, canvas_offset);
        let y = block_pos.y + Self::HEADER_HEIGHT * viewport.zoom 
            + (port_index as f32 + 0.5) * Self::PORT_HEIGHT * viewport.zoom;
        let x = if is_output {
            block_pos.x + block.size.x * viewport.zoom
        } else {
            block_pos.x
        };
        Pos2::new(x, y)
    }
}

