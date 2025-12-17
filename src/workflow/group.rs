//! Block分组

use super::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

/// Block分组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockGroup {
    pub id: Uuid,
    pub name: String,
    pub color: [u8; 3],
    pub blocks: HashSet<Uuid>,
    pub position: Vec2,
    pub size: Vec2,
}

impl BlockGroup {
    pub fn new(name: String, blocks: HashSet<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            color: [100, 149, 237], // Cornflower blue
            blocks,
            position: Vec2::default(),
            size: Vec2::default(),
        }
    }

    /// 更新分组边界
    pub fn update_bounds(&mut self, block_positions: &[(Uuid, Vec2, Vec2)]) {
        let group_blocks: Vec<_> = block_positions
            .iter()
            .filter(|(id, _, _)| self.blocks.contains(id))
            .collect();

        if group_blocks.is_empty() {
            return;
        }

        let padding = 20.0;
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for (_, pos, size) in &group_blocks {
            min_x = min_x.min(pos.x);
            min_y = min_y.min(pos.y);
            max_x = max_x.max(pos.x + size.x);
            max_y = max_y.max(pos.y + size.y);
        }

        self.position = Vec2::new(min_x - padding, min_y - padding - 24.0);
        self.size = Vec2::new(max_x - min_x + padding * 2.0, max_y - min_y + padding * 2.0 + 24.0);
    }
}

