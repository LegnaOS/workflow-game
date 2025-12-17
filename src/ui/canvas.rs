//! 无限画布 - 平移、缩放、网格

use crate::workflow::{Vec2, Viewport};
use egui::{Color32, Painter, Pos2, Rect, Stroke};

/// 画布渲染器
pub struct Canvas;

impl Canvas {
    /// 绘制网格背景
    pub fn draw_grid(painter: &Painter, viewport: &Viewport, rect: Rect) {
        let grid_size = 20.0 * viewport.zoom;
        let grid_color = Color32::from_gray(40);
        let major_grid_color = Color32::from_gray(50);

        // 计算可见范围
        let start_x = ((-viewport.offset.x / grid_size).floor() * grid_size) as i32;
        let start_y = ((-viewport.offset.y / grid_size).floor() * grid_size) as i32;
        let end_x = ((rect.width() - viewport.offset.x) / grid_size).ceil() as i32;
        let end_y = ((rect.height() - viewport.offset.y) / grid_size).ceil() as i32;

        // 绘制垂直线
        for i in start_x..=end_x {
            let x = i as f32 * grid_size + viewport.offset.x + rect.min.x;
            let color = if i % 5 == 0 { major_grid_color } else { grid_color };
            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                Stroke::new(1.0, color),
            );
        }

        // 绘制水平线
        for i in start_y..=end_y {
            let y = i as f32 * grid_size + viewport.offset.y + rect.min.y;
            let color = if i % 5 == 0 { major_grid_color } else { grid_color };
            painter.line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                Stroke::new(1.0, color),
            );
        }
    }

    /// 绘制框选矩形
    pub fn draw_selection_rect(painter: &Painter, min: Pos2, max: Pos2) {
        let rect = Rect::from_min_max(min, max);
        painter.rect_filled(rect, 0.0, Color32::from_rgba_unmultiplied(100, 149, 237, 30));
        painter.rect_stroke(rect, 0.0, Stroke::new(1.0, Color32::from_rgb(100, 149, 237)));
    }

    /// 转换Vec2到egui的Pos2
    pub fn vec2_to_pos2(v: Vec2, viewport: &Viewport, canvas_offset: Pos2) -> Pos2 {
        let screen = viewport.canvas_to_screen(v);
        Pos2::new(screen.x + canvas_offset.x, screen.y + canvas_offset.y)
    }

    /// 转换egui的Pos2到Vec2
    pub fn pos2_to_vec2(p: Pos2, viewport: &Viewport, canvas_offset: Pos2) -> Vec2 {
        let screen = Vec2::new(p.x - canvas_offset.x, p.y - canvas_offset.y);
        viewport.screen_to_canvas(screen)
    }
}

