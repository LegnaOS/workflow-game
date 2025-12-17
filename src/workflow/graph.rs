//! 工作流图

use super::{Block, BlockGroup, Connection, Vec2};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
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

    /// 最近执行的Block和连线（用于流动动画）
    #[serde(skip)]
    pub active_blocks: HashMap<Uuid, f32>,  // block_id -> 激活强度 (0.0-1.0)
    #[serde(skip)]
    pub active_connections: HashMap<Uuid, f32>,  // connection_id -> 激活强度 (0.0-1.0)
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
            active_blocks: HashMap::new(),
            active_connections: HashMap::new(),
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

    /// 激活Block（执行时调用）
    pub fn activate_block(&mut self, block_id: Uuid) {
        self.active_blocks.insert(block_id, 1.0);
        // 激活该Block的所有输出连线
        let conn_ids: Vec<Uuid> = self.connections.iter()
            .filter(|(_, c)| c.from_block == block_id)
            .map(|(id, _)| *id)
            .collect();
        for conn_id in conn_ids {
            self.active_connections.insert(conn_id, 1.0);
        }
    }

    /// 衰减激活状态（每帧调用）
    pub fn decay_activation(&mut self, decay_rate: f32) {
        // 衰减Block激活
        self.active_blocks.retain(|_, strength| {
            *strength -= decay_rate;
            *strength > 0.01
        });
        // 衰减连线激活
        self.active_connections.retain(|_, strength| {
            *strength -= decay_rate;
            *strength > 0.01
        });
    }

    /// 获取Block的激活强度
    pub fn get_block_activation(&self, block_id: Uuid) -> f32 {
        self.active_blocks.get(&block_id).copied().unwrap_or(0.0)
    }

    /// 获取连线的激活强度
    pub fn get_connection_activation(&self, conn_id: Uuid) -> f32 {
        self.active_connections.get(&conn_id).copied().unwrap_or(0.0)
    }

    /// 更新执行顺序(拓扑排序)
    pub fn update_execution_order(&mut self) {
        self.execution_order = self.topological_sort();
    }

    /// Kahn算法拓扑排序（稳定版：按位置排序保证确定性）
    fn topological_sort(&self) -> Vec<Uuid> {
        if self.blocks.is_empty() {
            return Vec::new();
        }

        let mut in_degree: HashMap<Uuid, usize> = HashMap::new();
        let mut adjacency: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

        // 初始化
        for block_id in self.blocks.keys() {
            in_degree.insert(*block_id, 0);
            adjacency.insert(*block_id, Vec::new());
        }

        // 构建图
        for conn in self.connections.values() {
            // 只处理有效连接（两端 block 都存在）
            if self.blocks.contains_key(&conn.from_block) && self.blocks.contains_key(&conn.to_block) {
                if let Some(degree) = in_degree.get_mut(&conn.to_block) {
                    *degree += 1;
                }
                if let Some(adj) = adjacency.get_mut(&conn.from_block) {
                    if !adj.contains(&conn.to_block) {
                        adj.push(conn.to_block);
                    }
                }
            }
        }

        // 使用 BTreeMap 按位置排序，确保相同入度的节点有确定顺序
        // 排序键：(位置Y, 位置X, UUID) 保证从上到下、从左到右
        let get_sort_key = |id: &Uuid| -> (i32, i32, Uuid) {
            if let Some(block) = self.blocks.get(id) {
                ((block.position.y * 100.0) as i32, (block.position.x * 100.0) as i32, *id)
            } else {
                (i32::MAX, i32::MAX, *id)
            }
        };

        // 收集入度为0的节点，按位置排序
        let mut queue: VecDeque<Uuid> = {
            let mut zero_degree: Vec<_> = in_degree
                .iter()
                .filter(|(_, &degree)| degree == 0)
                .map(|(&id, _)| id)
                .collect();
            zero_degree.sort_by_key(get_sort_key);
            zero_degree.into_iter().collect()
        };

        let mut result = Vec::with_capacity(self.blocks.len());

        while let Some(node) = queue.pop_front() {
            result.push(node);

            if let Some(neighbors) = adjacency.get(&node) {
                // 收集新的零入度节点
                let mut new_zero_degree = Vec::new();
                for &neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(&neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            new_zero_degree.push(neighbor);
                        }
                    }
                }
                // 按位置排序后加入队列
                new_zero_degree.sort_by_key(get_sort_key);
                for id in new_zero_degree {
                    queue.push_back(id);
                }
            }
        }

        // 检测环：如果结果数量少于节点数量，说明有环
        if result.len() < self.blocks.len() {
            log::warn!("工作流存在循环依赖，部分节点未执行");
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
        let h_spacing = 60.0;  // 水平间距
        let v_spacing = 40.0;  // 垂直间距
        let start_x = 100.0;
        let start_y = 100.0;

        // 计算每层的最大宽度（用于X坐标）
        let mut layer_max_widths: Vec<f32> = Vec::with_capacity(layers.len());
        for layer in &layers {
            let max_width = layer.iter()
                .filter_map(|id| self.blocks.get(id))
                .map(|b| b.size.x)
                .fold(0.0f32, |a, b| a.max(b));
            layer_max_widths.push(max_width.max(160.0)); // 最小160
        }

        // 按层级放置节点
        let mut current_x = start_x;
        for (level, layer) in layers.iter().enumerate() {
            // 计算该层所有Block的实际高度，用于垂直居中或排列
            let mut current_y = start_y;

            // 按Block的Y位置排序（保持原有的上下顺序）
            let mut sorted_layer: Vec<Uuid> = layer.clone();
            sorted_layer.sort_by(|a, b| {
                let ay = self.blocks.get(a).map(|b| b.position.y).unwrap_or(0.0);
                let by = self.blocks.get(b).map(|b| b.position.y).unwrap_or(0.0);
                ay.partial_cmp(&by).unwrap_or(std::cmp::Ordering::Equal)
            });

            for block_id in sorted_layer {
                if let Some(block) = self.blocks.get_mut(&block_id) {
                    block.position = Vec2::new(current_x, current_y);
                    current_y += block.size.y + v_spacing;
                }
            }

            // 下一层的X位置 = 当前X + 当前层最大宽度 + 间距
            current_x += layer_max_widths[level] + h_spacing;
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
