//! 连线渲染组件

use egui::{Color32, Painter, Pos2, Stroke};

/// 连线模式
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ConnectionMode {
    /// 贝塞尔曲线
    Bezier,
    /// 折线（直角）
    #[default]
    Orthogonal,
}

/// 连线渲染器
pub struct ConnectionWidget;

/// 全局连线模式（可通过UI切换）
static mut CONNECTION_MODE: ConnectionMode = ConnectionMode::Orthogonal;

impl ConnectionWidget {
    /// 设置连线模式
    pub fn set_mode(mode: ConnectionMode) {
        unsafe { CONNECTION_MODE = mode; }
    }

    /// 获取当前模式
    pub fn mode() -> ConnectionMode {
        unsafe { CONNECTION_MODE }
    }

    /// 绘制连接
    pub fn draw(painter: &Painter, from: Pos2, to: Pos2, dragging: bool) {
        let color = if dragging {
            Color32::from_rgb(80, 200, 255)
        } else {
            Color32::from_rgb(120, 180, 255)
        };
        Self::draw_line(painter, from, to, color, 2.5, 0.0);
    }

    /// 绘制连线（支持选中状态和流动效果）
    pub fn draw_with_selection(painter: &Painter, from: Pos2, to: Pos2, dragging: bool, selected: bool) {
        let color = if selected {
            Color32::from_rgb(255, 180, 50)
        } else if dragging {
            Color32::from_rgb(80, 200, 255)
        } else {
            Color32::from_rgb(100, 160, 230)
        };
        let width = if selected { 3.5 } else { 2.5 };

        Self::draw_line(painter, from, to, color, width, 0.0);
    }

    /// 绘制带流动效果的连线（支持激活强度）
    pub fn draw_with_flow(painter: &Painter, from: Pos2, to: Pos2, selected: bool, activation: f32) {
        let base_color = if selected {
            Color32::from_rgb(255, 180, 50)
        } else if activation > 0.01 {
            // 激活状态：根据强度显示高亮颜色
            let glow = (activation * 255.0) as u8;
            Color32::from_rgb(50 + glow / 2, 180, 100 + glow / 2)
        } else {
            Color32::from_rgb(100, 160, 230)
        };
        let width = if selected { 3.5 } else if activation > 0.01 { 3.0 } else { 2.5 };

        // 只有激活时才显示流动效果
        let flow_phase = if activation > 0.01 { activation } else { 0.0 };
        Self::draw_line(painter, from, to, base_color, width, flow_phase);
    }

    /// 绘制连线（根据模式）
    fn draw_line(painter: &Painter, from: Pos2, to: Pos2, color: Color32, width: f32, flow_phase: f32) {
        let mode = Self::mode();
        let points = match mode {
            ConnectionMode::Bezier => Self::bezier_points(from, to),
            ConnectionMode::Orthogonal => Self::orthogonal_points(from, to),
        };

        // 绘制阴影（更明显）
        let shadow_color = Color32::from_rgba_unmultiplied(0, 0, 0, 60);
        for i in 0..points.len() - 1 {
            painter.line_segment(
                [Pos2::new(points[i].x + 2.0, points[i].y + 2.0),
                 Pos2::new(points[i + 1].x + 2.0, points[i + 1].y + 2.0)],
                Stroke::new(width + 1.0, shadow_color)
            );
        }

        // 绘制主线
        for i in 0..points.len() - 1 {
            painter.line_segment([points[i], points[i + 1]], Stroke::new(width, color));
        }

        // 流动效果
        if flow_phase > 0.0 {
            Self::draw_flow_dots(painter, &points, flow_phase, width);
        }

        // 绘制箭头
        if points.len() >= 2 {
            Self::draw_arrow(painter, points[points.len() - 2], *points.last().unwrap(), color, width);
        }
    }

    /// 生成折线点（直角连接）
    fn orthogonal_points(from: Pos2, to: Pos2) -> Vec<Pos2> {
        let mid_x = (from.x + to.x) / 2.0;

        // 如果目标在左边，需要绕行
        if to.x < from.x + 40.0 {
            let offset = 30.0;
            vec![
                from,
                Pos2::new(from.x + offset, from.y),
                Pos2::new(from.x + offset, (from.y + to.y) / 2.0),
                Pos2::new(to.x - offset, (from.y + to.y) / 2.0),
                Pos2::new(to.x - offset, to.y),
                to,
            ]
        } else {
            vec![
                from,
                Pos2::new(mid_x, from.y),
                Pos2::new(mid_x, to.y),
                to,
            ]
        }
    }

    /// 生成贝塞尔曲线点
    fn bezier_points(from: Pos2, to: Pos2) -> Vec<Pos2> {
        let dx = (to.x - from.x).abs();
        let control_offset = (dx * 0.5).max(50.0);
        let control1 = Pos2::new(from.x + control_offset, from.y);
        let control2 = Pos2::new(to.x - control_offset, to.y);

        let segments = 24;
        (0..=segments)
            .map(|i| {
                let t = i as f32 / segments as f32;
                Self::cubic_bezier(from, control1, control2, to, t)
            })
            .collect()
    }

    /// 绘制流动点
    fn draw_flow_dots(painter: &Painter, points: &[Pos2], phase: f32, _width: f32) {
        let total_len = Self::path_length(points);
        if total_len < 1.0 { return; }

        let dot_spacing = 20.0;
        let dot_count = (total_len / dot_spacing) as i32;

        for i in 0..dot_count {
            let t = ((i as f32 * dot_spacing / total_len) + phase) % 1.0;
            let pos = Self::point_at_t(points, t);
            let alpha = ((1.0 - t) * 200.0) as u8;
            painter.circle_filled(pos, 3.0, Color32::from_rgba_unmultiplied(150, 220, 255, alpha));
        }
    }

    /// 计算路径长度
    fn path_length(points: &[Pos2]) -> f32 {
        points.windows(2)
            .map(|w| ((w[1].x - w[0].x).powi(2) + (w[1].y - w[0].y).powi(2)).sqrt())
            .sum()
    }

    /// 获取路径上t位置的点
    fn point_at_t(points: &[Pos2], t: f32) -> Pos2 {
        let total = Self::path_length(points);
        let target = t * total;
        let mut acc = 0.0;

        for w in points.windows(2) {
            let seg_len = ((w[1].x - w[0].x).powi(2) + (w[1].y - w[0].y).powi(2)).sqrt();
            if acc + seg_len >= target {
                let local_t = (target - acc) / seg_len;
                return Pos2::new(
                    w[0].x + (w[1].x - w[0].x) * local_t,
                    w[0].y + (w[1].y - w[0].y) * local_t,
                );
            }
            acc += seg_len;
        }
        *points.last().unwrap_or(&Pos2::ZERO)
    }

    /// 三次贝塞尔曲线插值
    fn cubic_bezier(p0: Pos2, p1: Pos2, p2: Pos2, p3: Pos2, t: f32) -> Pos2 {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;

        Pos2::new(
            mt3 * p0.x + 3.0 * mt2 * t * p1.x + 3.0 * mt * t2 * p2.x + t3 * p3.x,
            mt3 * p0.y + 3.0 * mt2 * t * p1.y + 3.0 * mt * t2 * p2.y + t3 * p3.y,
        )
    }

    /// 绘制箭头
    fn draw_arrow(painter: &Painter, from: Pos2, to: Pos2, color: Color32, width: f32) {
        let arrow_size = 10.0;
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let len = (dx * dx + dy * dy).sqrt();

        if len < 0.001 { return; }

        let nx = dx / len;
        let ny = dy / len;

        let arrow_p1 = Pos2::new(
            to.x - arrow_size * nx + arrow_size * 0.5 * ny,
            to.y - arrow_size * ny - arrow_size * 0.5 * nx,
        );
        let arrow_p2 = Pos2::new(
            to.x - arrow_size * nx - arrow_size * 0.5 * ny,
            to.y - arrow_size * ny + arrow_size * 0.5 * nx,
        );

        painter.line_segment([arrow_p1, to], Stroke::new(width, color));
        painter.line_segment([arrow_p2, to], Stroke::new(width, color));
    }

    // ============ 碰撞检测 ============

    /// 检测点是否在连线附近（用于点击选中）
    pub fn hit_test(from: Pos2, to: Pos2, point: Pos2, threshold: f32) -> bool {
        let points = match Self::mode() {
            ConnectionMode::Bezier => Self::bezier_points(from, to),
            ConnectionMode::Orthogonal => Self::orthogonal_points(from, to),
        };
        Self::point_to_polyline_distance(&points, point) < threshold
    }

    /// 检测连线是否与矩形相交（用于框选）
    pub fn intersects_rect(from: Pos2, to: Pos2, rect_min: Pos2, rect_max: Pos2) -> bool {
        let points = match Self::mode() {
            ConnectionMode::Bezier => Self::bezier_points(from, to),
            ConnectionMode::Orthogonal => Self::orthogonal_points(from, to),
        };

        // 检查每条线段是否与矩形相交
        for w in points.windows(2) {
            if Self::line_intersects_rect(w[0], w[1], rect_min, rect_max) {
                return true;
            }
        }
        false
    }

    /// 点到折线的最短距离
    fn point_to_polyline_distance(points: &[Pos2], point: Pos2) -> f32 {
        let mut min_dist = f32::MAX;
        for w in points.windows(2) {
            let dist = Self::point_to_segment_distance(point, w[0], w[1]);
            if dist < min_dist {
                min_dist = dist;
            }
        }
        min_dist
    }

    /// 点到线段的距离
    fn point_to_segment_distance(p: Pos2, a: Pos2, b: Pos2) -> f32 {
        let ab = Pos2::new(b.x - a.x, b.y - a.y);
        let ap = Pos2::new(p.x - a.x, p.y - a.y);
        let ab_len_sq = ab.x * ab.x + ab.y * ab.y;

        if ab_len_sq < 0.0001 {
            return ((p.x - a.x).powi(2) + (p.y - a.y).powi(2)).sqrt();
        }

        let t = ((ap.x * ab.x + ap.y * ab.y) / ab_len_sq).clamp(0.0, 1.0);
        let closest = Pos2::new(a.x + t * ab.x, a.y + t * ab.y);
        ((p.x - closest.x).powi(2) + (p.y - closest.y).powi(2)).sqrt()
    }

    /// 线段是否与矩形相交
    fn line_intersects_rect(a: Pos2, b: Pos2, rect_min: Pos2, rect_max: Pos2) -> bool {
        // 快速检查：点在矩形内
        if Self::point_in_rect(a, rect_min, rect_max) || Self::point_in_rect(b, rect_min, rect_max) {
            return true;
        }

        // 检查线段与矩形四边的交点
        let edges = [
            (Pos2::new(rect_min.x, rect_min.y), Pos2::new(rect_max.x, rect_min.y)), // top
            (Pos2::new(rect_max.x, rect_min.y), Pos2::new(rect_max.x, rect_max.y)), // right
            (Pos2::new(rect_max.x, rect_max.y), Pos2::new(rect_min.x, rect_max.y)), // bottom
            (Pos2::new(rect_min.x, rect_max.y), Pos2::new(rect_min.x, rect_min.y)), // left
        ];

        for (e1, e2) in edges {
            if Self::segments_intersect(a, b, e1, e2) {
                return true;
            }
        }
        false
    }

    fn point_in_rect(p: Pos2, rect_min: Pos2, rect_max: Pos2) -> bool {
        p.x >= rect_min.x && p.x <= rect_max.x && p.y >= rect_min.y && p.y <= rect_max.y
    }

    /// 两线段是否相交
    fn segments_intersect(a1: Pos2, a2: Pos2, b1: Pos2, b2: Pos2) -> bool {
        let d1 = Self::cross_product(b2, b1, a1);
        let d2 = Self::cross_product(b2, b1, a2);
        let d3 = Self::cross_product(a2, a1, b1);
        let d4 = Self::cross_product(a2, a1, b2);

        if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0)) &&
           ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0)) {
            return true;
        }

        // 共线情况（可以忽略）
        false
    }

    fn cross_product(o: Pos2, a: Pos2, b: Pos2) -> f32 {
        (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
    }
}

