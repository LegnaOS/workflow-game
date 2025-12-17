//! Block实例 - 画布上的节点

use crate::script::{BlockDefinition, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 2D向量
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Block实例
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: Uuid,
    pub script_id: String,
    pub position: Vec2,
    pub size: Vec2,

    // 运行时值
    pub input_values: HashMap<String, Value>,
    pub output_values: HashMap<String, Value>,
    pub properties: HashMap<String, Value>,

    // 持久化状态（跨执行周期保留）
    pub state: HashMap<String, Value>,

    // UI状态
    pub selected: bool,
    pub collapsed: bool,
    pub group_id: Option<Uuid>,
}

impl Block {
    /// 从Block定义创建新实例
    pub fn new(definition: &BlockDefinition, position: Vec2) -> Self {
        let mut properties = HashMap::new();
        for prop in &definition.properties {
            properties.insert(prop.id.clone(), prop.default.clone());
        }

        let mut input_values = HashMap::new();
        for input in &definition.inputs {
            input_values.insert(input.id.clone(), input.default.clone());
        }

        let mut output_values = HashMap::new();
        for output in &definition.outputs {
            output_values.insert(output.id.clone(), output.default.clone());
        }

        let size = Vec2::new(definition.calculate_width(), definition.calculate_height());

        Self {
            id: Uuid::new_v4(),
            script_id: definition.meta.id.clone(),
            position,
            size,
            input_values,
            output_values,
            properties,
            state: HashMap::new(),
            selected: false,
            collapsed: false,
            group_id: None,
        }
    }

    /// 获取输入端口的屏幕位置
    pub fn input_port_position(&self, index: usize, port_count: usize) -> Vec2 {
        let port_height = 24.0;
        let header_height = 32.0;
        let y = self.position.y + header_height + (index as f32 * port_height) + port_height / 2.0;
        Vec2::new(self.position.x, y)
    }

    /// 获取输出端口的屏幕位置
    pub fn output_port_position(&self, index: usize, port_count: usize) -> Vec2 {
        let port_height = 24.0;
        let header_height = 32.0;
        let y = self.position.y + header_height + (index as f32 * port_height) + port_height / 2.0;
        Vec2::new(self.position.x + self.size.x, y)
    }

    /// 获取Block的边界矩形
    pub fn bounds(&self) -> (Vec2, Vec2) {
        (
            self.position,
            Vec2::new(
                self.position.x + self.size.x,
                self.position.y + self.size.y,
            ),
        )
    }

    /// 检查点是否在Block内
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.position.x
            && point.x <= self.position.x + self.size.x
            && point.y >= self.position.y
            && point.y <= self.position.y + self.size.y
    }

    /// 设置输入值
    pub fn set_input(&mut self, port_id: &str, value: Value) {
        self.input_values.insert(port_id.to_string(), value);
    }

    /// 设置输出值
    pub fn set_output(&mut self, port_id: &str, value: Value) {
        self.output_values.insert(port_id.to_string(), value);
    }

    /// 获取输出值
    pub fn get_output(&self, port_id: &str) -> Option<&Value> {
        self.output_values.get(port_id)
    }

    /// 吸附到网格
    pub fn snap_to_grid(&mut self, grid_size: f32) {
        self.position.x = (self.position.x / grid_size).round() * grid_size;
        self.position.y = (self.position.y / grid_size).round() * grid_size;
    }
}

