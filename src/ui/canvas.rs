//! 无限画布 - 平移、缩放、网格

use crate::workflow::{Vec2, Viewport};
use egui::{Color32, Painter, Pos2, Rect, Stroke};

/// 画布渲染器
pub struct Canvas;

impl Canvas {
    /// 绘制网格背景
    pub fn draw_grid(painter: &Painter, viewport: &Viewport, rect: Rect) {
        // 60%灰色网格（0.6 * 255 ≈ 153，但因为是暗色主题，用较暗的灰）
        // 60%灰 = 40%黑 = 255 * 0.4 = 102
        let grid_color = Color32::from_gray(100);         // 小网格 60%灰
        let major_grid_color = Color32::from_gray(120);   // 大网格稍亮

        // 根据缩放级别选择合适的网格大小，避免网格太密或太稀
        let base_grid = 20.0;
        let screen_grid = base_grid * viewport.zoom;

        // 网格太小时放大步长，太大时缩小步长
        let (grid_step, major_step) = if screen_grid < 8.0 {
            // 缩得太小，用5倍网格
            (base_grid * 5.0, 5)
        } else if screen_grid < 15.0 {
            // 稍小，用2倍网格
            (base_grid * 2.0, 5)
        } else if screen_grid > 80.0 {
            // 放得太大，用0.5倍网格
            (base_grid * 0.5, 10)
        } else {
            (base_grid, 5)
        };

        let screen_step = grid_step * viewport.zoom;

        // 计算可见范围内的网格线数量
        let offset_x = viewport.offset.x % screen_step;
        let offset_y = viewport.offset.y % screen_step;

        let count_x = (rect.width() / screen_step).ceil() as i32 + 1;
        let count_y = (rect.height() / screen_step).ceil() as i32 + 1;

        // 计算起始网格索引（用于判断是否是主网格线）
        let start_idx_x = ((-viewport.offset.x) / screen_step).floor() as i32;
        let start_idx_y = ((-viewport.offset.y) / screen_step).floor() as i32;

        // 绘制垂直线
        for i in 0..count_x {
            let x = rect.min.x + offset_x + i as f32 * screen_step;
            if x < rect.min.x || x > rect.max.x {
                continue;
            }
            let idx = start_idx_x + i;
            let color = if idx % major_step == 0 { major_grid_color } else { grid_color };
            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                Stroke::new(1.0, color),
            );
        }

        // 绘制水平线
        for i in 0..count_y {
            let y = rect.min.y + offset_y + i as f32 * screen_step;
            if y < rect.min.y || y > rect.max.y {
                continue;
            }
            let idx = start_idx_y + i;
            let color = if idx % major_step == 0 { major_grid_color } else { grid_color };
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

