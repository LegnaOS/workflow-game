//! Block实例 - 画布上的节点

use crate::script::{BlockDefinition, DataType, PortDefinition, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Block 显示模式
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockDisplayMode {
    /// Mini 模式：只显示名称和图标，不显示端口
    Mini,
    /// Full 模式：显示完整端口列表
    #[default]
    Full,
    /// Hidden 模式：完全隐藏（预览模式下的子块）
    Hidden,
}

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

    /// 自定义名称（覆盖定义名称）
    #[serde(default)]
    pub custom_name: Option<String>,

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

    // 动画状态（运行时，不序列化）
    /// 当前动画偏移量
    #[serde(skip)]
    pub animation_offset: Vec2,
    /// 目标动画偏移量（由Lua设置）
    #[serde(skip)]
    pub animation_target: Vec2,
    /// 动画速度（每秒移动的像素距离，0表示瞬移）
    #[serde(skip)]
    pub animation_speed: f32,

    /// 交互控件状态（用于输入框等）
    #[serde(default)]
    pub widget_text: String,
    /// 控件是否正在编辑
    #[serde(skip)]
    pub widget_editing: bool,
    /// 下拉选择的索引
    #[serde(default)]
    pub widget_selected_index: usize,
    /// 复选框/按钮状态
    #[serde(default)]
    pub widget_checked: bool,
    /// 滑块值
    #[serde(default)]
    pub widget_slider_value: f32,

    /// 动态输出端口（运行时生成）
    #[serde(default)]
    pub dynamic_outputs: Vec<PortDefinition>,
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
            custom_name: None,
            input_values,
            output_values,
            properties,
            state: HashMap::new(),
            selected: false,
            collapsed: false,
            group_id: None,
            animation_offset: Vec2::new(0.0, 0.0),
            animation_target: Vec2::new(0.0, 0.0),
            animation_speed: 200.0, // 默认速度：200像素/秒
            widget_text: String::new(),
            widget_editing: false,
            widget_selected_index: 0,
            widget_checked: false,
            widget_slider_value: 0.0,
            dynamic_outputs: Vec::new(),
        }
    }

    /// 获取所有输出端口（静态 + 动态）
    pub fn all_outputs<'a>(&'a self, definition: &'a BlockDefinition) -> Vec<&'a PortDefinition> {
        let mut outputs: Vec<&PortDefinition> = definition.outputs.iter().collect();
        outputs.extend(self.dynamic_outputs.iter());
        outputs
    }

    /// 更新动态输出端口（根据执行结果）
    pub fn update_dynamic_outputs(&mut self, definition: &BlockDefinition) {
        // 收集所有静态输出端口 ID
        let static_ids: std::collections::HashSet<_> =
            definition.outputs.iter().map(|p| p.id.as_str()).collect();

        // 找出 output_values 中不在静态端口中的 key
        let mut new_dynamic: Vec<PortDefinition> = Vec::new();
        for (key, value) in &self.output_values {
            if !static_ids.contains(key.as_str()) {
                // 推断数据类型
                let data_type = match value {
                    Value::Number(_) => DataType::Number,
                    Value::String(_) => DataType::String,
                    Value::Boolean(_) => DataType::Boolean,
                    Value::Array(_) => DataType::Array,
                    _ => DataType::Any,
                };
                new_dynamic.push(PortDefinition {
                    id: key.clone(),
                    name: key.clone(),
                    data_type,
                    default: Value::Nil,
                    description: String::new(),
                    required: false,
                    multiple: false,
                    element_type: None,
                    min: None,
                    max: None,
                });
            }
        }

        // 按 id 排序保持稳定顺序
        new_dynamic.sort_by(|a, b| a.id.cmp(&b.id));
        self.dynamic_outputs = new_dynamic;
    }

    /// 重新计算 Block 尺寸（考虑动态端口）
    pub fn recalculate_size(&mut self, definition: &BlockDefinition) {
        let total_outputs = definition.outputs.len() + self.dynamic_outputs.len();
        let port_count = definition.inputs.len().max(total_outputs);
        let header_height = 28.0;
        let port_height = 22.0;
        let min_height = 60.0;
        let calculated_height = header_height + (port_count as f32 * port_height);
        self.size.y = calculated_height.max(min_height);

        // 计算宽度（考虑动态端口名称）
        let base_width: f32 = 140.0;
        let name_width = definition.meta.name.chars().count() as f32 * 10.0 + 20.0;
        let max_input_len = definition
            .inputs
            .iter()
            .map(|p| p.name.chars().count())
            .max()
            .unwrap_or(0) as f32;
        let max_static_output_len = definition
            .outputs
            .iter()
            .map(|p| p.name.chars().count())
            .max()
            .unwrap_or(0) as f32;
        let max_dynamic_output_len = self
            .dynamic_outputs
            .iter()
            .map(|p| p.name.chars().count())
            .max()
            .unwrap_or(0) as f32;
        let max_output_len = max_static_output_len.max(max_dynamic_output_len);
        let port_width = (max_input_len + max_output_len) * 7.0 + 60.0;

        self.size.x = base_width.max(name_width).max(port_width);
    }

    /// 更新动画（每帧调用）
    /// 返回true表示动画仍在进行
    pub fn update_animation(&mut self, delta_time: f32) -> bool {
        let dx = self.animation_target.x - self.animation_offset.x;
        let dy = self.animation_target.y - self.animation_offset.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < 0.5 {
            // 足够接近，直接到达
            self.animation_offset = self.animation_target;
            return false;
        }

        if self.animation_speed <= 0.0 {
            // 瞬移
            self.animation_offset = self.animation_target;
            return false;
        }

        // 按速度移动
        let move_distance = self.animation_speed * delta_time;
        if move_distance >= distance {
            self.animation_offset = self.animation_target;
            return false;
        }

        let ratio = move_distance / distance;
        self.animation_offset.x += dx * ratio;
        self.animation_offset.y += dy * ratio;
        true
    }

    /// 设置动画目标（由Lua调用）
    pub fn set_animation_target(&mut self, x: f32, y: f32, speed: Option<f32>) {
        self.animation_target = Vec2::new(x, y);
        if let Some(s) = speed {
            self.animation_speed = s;
        }
    }

    /// 获取渲染位置（包含动画偏移）
    pub fn render_position(&self) -> Vec2 {
        Vec2::new(
            self.position.x + self.animation_offset.x,
            self.position.y + self.animation_offset.y,
        )
    }

    /// 获取显示名称（自定义名称优先）
    pub fn display_name<'a>(&'a self, definition: &'a BlockDefinition) -> &'a str {
        self.custom_name.as_deref().unwrap_or(&definition.meta.name)
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

    /// Mini 模式的尺寸
    pub fn mini_size() -> Vec2 {
        Vec2::new(80.0, 36.0)
    }

    /// 获取当前显示尺寸（根据显示模式）
    pub fn display_size(&self, mode: BlockDisplayMode) -> Vec2 {
        match mode {
            BlockDisplayMode::Mini => Self::mini_size(),
            BlockDisplayMode::Full => self.size,
            BlockDisplayMode::Hidden => Vec2::new(0.0, 0.0),
        }
    }

    /// 检查点是否在Block内（考虑显示模式）
    pub fn contains_with_mode(&self, point: Vec2, mode: BlockDisplayMode) -> bool {
        let size = self.display_size(mode);
        point.x >= self.position.x
            && point.x <= self.position.x + size.x
            && point.y >= self.position.y
            && point.y <= self.position.y + size.y
    }
}

