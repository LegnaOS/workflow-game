//! 图层 - 画布区域快捷跳转

use super::Vec2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 图层（画布区域书签）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: Uuid,
    pub name: String,
    /// 图层区域的中心点（画布坐标）
    pub center: Vec2,
    /// 图层区域的尺寸（画布坐标）
    pub size: Vec2,
    /// 缩放级别
    pub zoom: f32,
    /// 显示顺序
    pub order: usize,
    /// 图层颜色（用于UI显示）
    pub color: [u8; 3],
}

impl Layer {
    pub fn new(name: impl Into<String>, center: Vec2, size: Vec2) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            center,
            size,
            zoom: 1.0,
            order: 0,
            color: [100, 150, 200],
        }
    }

    /// 创建默认图层
    pub fn default_layer() -> Self {
        Self::new("主场景", Vec2::new(400.0, 300.0), Vec2::new(800.0, 600.0))
    }

    /// 获取图层的边界矩形
    pub fn bounds(&self) -> (Vec2, Vec2) {
        let half_w = self.size.x / 2.0;
        let half_h = self.size.y / 2.0;
        (
            Vec2::new(self.center.x - half_w, self.center.y - half_h),
            Vec2::new(self.center.x + half_w, self.center.y + half_h),
        )
    }

    /// 检查点是否在图层区域内
    pub fn contains(&self, point: Vec2) -> bool {
        let (min, max) = self.bounds();
        point.x >= min.x && point.x <= max.x && point.y >= min.y && point.y <= max.y
    }
}

