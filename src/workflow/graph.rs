//! 工作流图

use super::{Block, BlockGroup, Connection, Vec2};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// 画布视口
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viewport {
    pub offset: Vec2,
    pub zoom: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            offset: Vec2::new(0.0, 0.0),
            zoom: 1.0,
        }
    }
}

impl Viewport {
    /// 屏幕坐标转画布坐标
    pub fn screen_to_canvas(&self, screen_pos: Vec2) -> Vec2 {
        Vec2::new(
            (screen_pos.x - self.offset.x) / self.zoom,
            (screen_pos.y - self.offset.y) / self.zoom,
        )
    }

    /// 画布坐标转屏幕坐标
    pub fn canvas_to_screen(&self, canvas_pos: Vec2) -> Vec2 {
        Vec2::new(
            canvas_pos.x * self.zoom + self.offset.x,
            canvas_pos.y * self.zoom + self.offset.y,
        )
    }

    /// 限制缩放范围
    pub fn clamp_zoom(&mut self) {
        self.zoom = self.zoom.clamp(0.1, 10.0);
    }
}

/// 完整的工作流
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub blocks: HashMap<Uuid, Block>,
    pub connections: HashMap<Uuid, Connection>,
    pub groups: HashMap<Uuid, BlockGroup>,
    pub viewport: Viewport,

    /// 只读模式（可分发版本）
    #[serde(default)]
    pub readonly: bool,

    // 执行相关(不序列化)
    #[serde(skip)]
    pub execution_order: Vec<Uuid>,
    #[serde(skip)]
    pub dirty_blocks: HashSet<Uuid>,
}

impl Default for Workflow {
    fn default() -> Self {
        Self {
            name: "未命名".to_string(),
            blocks: HashMap::new(),
            connections: HashMap::new(),
            groups: HashMap::new(),
            viewport: Viewport::default(),
            readonly: false,
            execution_order: Vec::new(),
            dirty_blocks: HashSet::new(),
        }
    }
}

impl Workflow {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// 创建只读副本（可分发版本）
    pub fn to_distributable(&self) -> Self {
        let mut dist = self.clone();
        dist.readonly = true;
        dist
    }

    /// 检查是否可编辑
    pub fn is_editable(&self) -> bool {
        !self.readonly
    }

    /// 根据注册表更新所有Block的尺寸（加载后调用）
    pub fn update_block_sizes(&mut self, registry: &crate::script::ScriptRegistry) {
        for block in self.blocks.values_mut() {
            if let Some(def) = registry.get(&block.script_id) {
                block.size = Vec2::new(def.calculate_width(), def.calculate_height());
            }
        }
    }

    /// 添加Block
    pub fn add_block(&mut self, block: Block) -> Uuid {
        let id = block.id;
        self.blocks.insert(id, block);
        self.mark_dirty(id);
        self.update_execution_order();
        id
    }

    /// 删除Block
    pub fn remove_block(&mut self, id: Uuid) {
        self.blocks.remove(&id);
        // 删除相关连接
        self.connections
            .retain(|_, conn| conn.from_block != id && conn.to_block != id);
        // 从分组中移除
        for group in self.groups.values_mut() {
            group.blocks.remove(&id);
        }
        self.update_execution_order();
    }

    /// 添加连接
    pub fn add_connection(&mut self, connection: Connection) -> Uuid {
        let id = connection.id;
        self.mark_dirty(connection.to_block);
        self.connections.insert(id, connection);
        self.update_execution_order();
        id
    }

    /// 删除连接
    pub fn remove_connection(&mut self, id: Uuid) {
        if let Some(conn) = self.connections.remove(&id) {
            self.mark_dirty(conn.to_block);
        }
        self.update_execution_order();
    }

    /// 获取Block的所有输入连接
    pub fn get_input_connections(&self, block_id: Uuid) -> Vec<&Connection> {
        self.connections
            .values()
            .filter(|conn| conn.to_block == block_id)
            .collect()
    }

    /// 获取Block的所有输出连接
    pub fn get_output_connections(&self, block_id: Uuid) -> Vec<&Connection> {
        self.connections
            .values()
            .filter(|conn| conn.from_block == block_id)
            .collect()
    }

    /// 标记Block为脏(需要重新执行)
    pub fn mark_dirty(&mut self, block_id: Uuid) {
        self.dirty_blocks.insert(block_id);
        // 标记所有下游Block
        let downstream: Vec<Uuid> = self
            .connections
            .values()
            .filter(|c| c.from_block == block_id)
            .map(|c| c.to_block)
            .collect();
        for id in downstream {
            if !self.dirty_blocks.contains(&id) {
                self.mark_dirty(id);
            }
        }
    }

    /// 更新执行顺序(拓扑排序)
    pub fn update_execution_order(&mut self) {
        self.execution_order = self.topological_sort();
    }

    /// Kahn算法拓扑排序
    fn topological_sort(&self) -> Vec<Uuid> {
        let mut in_degree: HashMap<Uuid, usize> = HashMap::new();
        let mut adjacency: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

        // 初始化
        for block_id in self.blocks.keys() {
            in_degree.insert(*block_id, 0);
            adjacency.insert(*block_id, Vec::new());
        }

        // 构建图
        for conn in self.connections.values() {
            if let Some(degree) = in_degree.get_mut(&conn.to_block) {
                *degree += 1;
            }
            if let Some(adj) = adjacency.get_mut(&conn.from_block) {
                adj.push(conn.to_block);
            }
        }

        // Kahn算法
        let mut queue: Vec<Uuid> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(&id, _)| id)
            .collect();

        let mut result = Vec::new();

        while let Some(node) = queue.pop() {
            result.push(node);

            if let Some(neighbors) = adjacency.get(&node) {
                for &neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(&neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(neighbor);
                        }
                    }
                }
            }
        }

        result
    }

    /// 获取选中的Block
    pub fn selected_blocks(&self) -> Vec<Uuid> {
        self.blocks
            .iter()
            .filter(|(_, b)| b.selected)
            .map(|(id, _)| *id)
            .collect()
    }

    /// 清除所有选中状态
    pub fn clear_selection(&mut self) {
        for block in self.blocks.values_mut() {
            block.selected = false;
        }
    }

    /// 框选Block
    pub fn select_in_rect(&mut self, min: Vec2, max: Vec2) {
        for block in self.blocks.values_mut() {
            let (bmin, bmax) = block.bounds();
            // 检查是否相交
            let intersects = bmin.x < max.x && bmax.x > min.x && bmin.y < max.y && bmax.y > min.y;
            block.selected = intersects;
        }
    }

    /// 创建分组
    pub fn create_group(&mut self, name: String) -> Option<Uuid> {
        let selected: HashSet<Uuid> = self.selected_blocks().into_iter().collect();
        if selected.is_empty() {
            return None;
        }

        let mut group = BlockGroup::new(name, selected.clone());

        // 更新Block的group_id
        for block_id in &selected {
            if let Some(block) = self.blocks.get_mut(block_id) {
                block.group_id = Some(group.id);
            }
        }

        // 计算分组边界
        let positions: Vec<_> = self
            .blocks
            .iter()
            .map(|(id, b)| (*id, b.position, b.size))
            .collect();
        group.update_bounds(&positions);

        let id = group.id;
        self.groups.insert(id, group);
        Some(id)
    }

    /// 解散分组
    pub fn ungroup(&mut self, group_id: Uuid) {
        if let Some(group) = self.groups.remove(&group_id) {
            for block_id in group.blocks {
                if let Some(block) = self.blocks.get_mut(&block_id) {
                    block.group_id = None;
                }
            }
        }
    }

    /// 自动布局（基于拓扑排序的分层布局）
    pub fn auto_layout(&mut self) {
        if self.blocks.is_empty() {
            return;
        }

        // 计算每个节点的层级（基于拓扑排序）
        let mut levels: HashMap<Uuid, usize> = HashMap::new();
        let order = self.topological_sort();

        // 计算入度为0的起始节点
        let mut in_degree: HashMap<Uuid, usize> = HashMap::new();
        for block_id in self.blocks.keys() {
            in_degree.insert(*block_id, 0);
        }
        for conn in self.connections.values() {
            *in_degree.entry(conn.to_block).or_insert(0) += 1;
        }

        // 根据依赖关系分配层级
        for &block_id in &order {
            let max_parent_level = self.connections.values()
                .filter(|c| c.to_block == block_id)
                .filter_map(|c| levels.get(&c.from_block))
                .max()
                .copied()
                .unwrap_or(0);

            let level = if in_degree.get(&block_id) == Some(&0) {
                0
            } else {
                max_parent_level + 1
            };
            levels.insert(block_id, level);
        }

        // 按层级分组
        let max_level = levels.values().max().copied().unwrap_or(0);
        let mut layers: Vec<Vec<Uuid>> = vec![Vec::new(); max_level + 1];
        for (block_id, level) in &levels {
            layers[*level].push(*block_id);
        }

        // 布局参数
        let block_width = 160.0;
        let block_height = 120.0;
        let h_spacing = 80.0;
        let v_spacing = 60.0;
        let start_x = 100.0;
        let start_y = 100.0;

        // 按层级放置节点
        for (level, layer) in layers.iter().enumerate() {
            let x = start_x + level as f32 * (block_width + h_spacing);
            for (index, &block_id) in layer.iter().enumerate() {
                let y = start_y + index as f32 * (block_height + v_spacing);
                if let Some(block) = self.blocks.get_mut(&block_id) {
                    block.position = Vec2::new(x, y);
                }
            }
        }

        // 更新分组边界
        let positions: Vec<_> = self.blocks.iter()
            .map(|(id, b)| (*id, b.position, b.size))
            .collect();
        for group in self.groups.values_mut() {
            group.update_bounds(&positions);
        }
    }
}
