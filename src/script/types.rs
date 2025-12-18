//! 脚本类型定义 - Lua脚本解析后的数据结构

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 数据类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Number,
    String,
    Boolean,
    Event,
    Array,
    Any,
}

impl Default for DataType {
    fn default() -> Self {
        Self::Any
    }
}

/// 运行时值
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Nil,
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Default for Value {
    fn default() -> Self {
        Self::Nil
    }
}

impl Value {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

/// Block交互控件类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum WidgetType {
    #[default]
    None,           // 普通Block，无交互控件
    TextInput,      // 文本输入框
    Password,       // 密码输入框
    TextArea,       // 多行文本
    Slider,         // 滑块
    Checkbox,       // 复选框
    Dropdown,       // 下拉选择
    Button,         // 按钮
}

/// Block元信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMeta {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default = "default_color")]
    pub color: String,
    /// 交互控件类型
    #[serde(default)]
    pub widget: WidgetType,
    /// 控件占位符文本
    #[serde(default)]
    pub placeholder: String,
    /// 下拉选项（用于Dropdown类型）
    #[serde(default)]
    pub options: Vec<String>,
    /// 预览模式下可隐藏（有连线时隐藏，孤立时显示）
    #[serde(default)]
    pub hideable: bool,
}

fn default_color() -> String {
    "#607D8B".to_string()
}

/// 端口定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortDefinition {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: DataType,
    #[serde(default)]
    pub default: Value,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub multiple: bool,
    #[serde(default)]
    pub element_type: Option<DataType>,
    #[serde(default)]
    pub min: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
}

/// 属性定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDefinition {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: DataType,
    #[serde(default)]
    pub default: Value,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub min: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
}

/// Block定义 - 从Lua解析出的完整定义
#[derive(Debug, Clone)]
pub struct BlockDefinition {
    pub meta: BlockMeta,
    pub inputs: Vec<PortDefinition>,
    pub outputs: Vec<PortDefinition>,
    pub properties: Vec<PropertyDefinition>,
    pub script_path: String,
}

impl BlockDefinition {
    /// 计算Block显示所需的高度
    pub fn calculate_height(&self) -> f32 {
        let port_count = self.inputs.len().max(self.outputs.len());
        let header_height = 28.0;
        let port_height = 22.0;
        let property_height = if self.properties.is_empty() { 0.0 } else { 8.0 };
        let min_height = 60.0;
        let calculated = header_height + (port_count as f32 * port_height) + property_height;
        calculated.max(min_height)
    }

    /// 计算Block显示所需的宽度
    pub fn calculate_width(&self) -> f32 {
        let base_width: f32 = 140.0;

        // 根据名称长度调整
        let name_width = self.meta.name.chars().count() as f32 * 10.0 + 20.0;

        // 根据端口名称长度调整
        let max_input_len = self.inputs.iter()
            .map(|p| p.name.chars().count())
            .max()
            .unwrap_or(0) as f32;
        let max_output_len = self.outputs.iter()
            .map(|p| p.name.chars().count())
            .max()
            .unwrap_or(0) as f32;
        let port_width = (max_input_len + max_output_len) * 7.0 + 60.0;

        base_width.max(name_width).max(port_width)
    }
}

