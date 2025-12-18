//! 脚本解析器 - 从Lua脚本解析Block定义

use crate::script::types::*;
use anyhow::{anyhow, Result};
use mlua::{Lua, Table, Value as LuaValue};
use std::path::Path;

/// 将mlua错误转换为anyhow错误
fn lua_err(e: mlua::Error) -> anyhow::Error {
    anyhow!("Lua错误: {}", e)
}

/// 脚本解析器
pub struct ScriptParser {
    lua: Lua,
}

impl ScriptParser {
    pub fn new() -> Result<Self> {
        let lua = Lua::new();
        Ok(Self { lua })
    }

    /// 解析Lua脚本，返回Block定义
    pub fn parse(&self, content: &str, script_path: &Path) -> Result<BlockDefinition> {
        // 执行脚本获取返回的table
        let table: Table = self
            .lua
            .load(content)
            .eval()
            .map_err(|e| anyhow!("执行脚本失败 {}: {}", script_path.display(), e))?;

        // 解析meta
        let meta = self.parse_meta(&table, script_path)?;

        // 解析inputs
        let inputs = self.parse_ports(&table, "inputs")?;

        // 解析outputs
        let outputs = self.parse_ports(&table, "outputs")?;

        // 解析properties
        let properties = self.parse_properties(&table)?;

        Ok(BlockDefinition {
            meta,
            inputs,
            outputs,
            properties,
            script_path: script_path.to_string_lossy().to_string(),
        })
    }

    fn parse_meta(&self, table: &Table, script_path: &Path) -> Result<BlockMeta> {
        use crate::script::WidgetType;

        let meta: Table = table
            .get("meta")
            .map_err(|_| anyhow!("脚本缺少meta字段: {}", script_path.display()))?;

        let id: String = meta
            .get("id")
            .map_err(|_| anyhow!("meta缺少id字段"))?;
        let name: String = meta
            .get("name")
            .map_err(|_| anyhow!("meta缺少name字段"))?;
        let version: String = meta.get("version").unwrap_or_else(|_| "1.0.0".to_string());

        // 从文件路径推断分类
        let default_category = script_path
            .parent()
            .and_then(|p| p.file_name())
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "未分类".to_string());

        let category: String = meta.get("category").unwrap_or(default_category);
        let description: String = meta.get("description").unwrap_or_default();
        let icon: String = meta.get("icon").unwrap_or_default();
        let color: String = meta.get("color").unwrap_or_else(|_| "#607D8B".to_string());

        // 解析widget类型
        let widget_str: String = meta.get("widget").unwrap_or_default();
        let widget = match widget_str.to_lowercase().as_str() {
            "textinput" | "text_input" | "input" => WidgetType::TextInput,
            "password" => WidgetType::Password,
            "textarea" | "text_area" => WidgetType::TextArea,
            "slider" => WidgetType::Slider,
            "checkbox" => WidgetType::Checkbox,
            "dropdown" | "select" => WidgetType::Dropdown,
            "button" => WidgetType::Button,
            _ => WidgetType::None,
        };

        let placeholder: String = meta.get("placeholder").unwrap_or_default();

        // 解析下拉选项
        let options: Vec<String> = if let Ok(opts_table) = meta.get::<Table>("options") {
            let mut opts = Vec::new();
            for pair in opts_table.pairs::<i64, String>() {
                if let Ok((_, v)) = pair {
                    opts.push(v);
                }
            }
            opts
        } else {
            Vec::new()
        };

        // 解析 hideable 属性
        let hideable: bool = meta.get("hideable").unwrap_or(false);

        Ok(BlockMeta {
            id,
            name,
            version,
            category,
            description,
            icon,
            color,
            widget,
            placeholder,
            options,
            hideable,
        })
    }

    fn parse_ports(&self, table: &Table, key: &str) -> Result<Vec<PortDefinition>> {
        let mut ports = Vec::new();

        if let Ok(ports_table) = table.get::<Table>(key) {
            for pair in ports_table.pairs::<i64, Table>() {
                let (_, port_table) = pair.map_err(lua_err)?;
                let port = self.parse_port(&port_table)?;
                ports.push(port);
            }
        }

        Ok(ports)
    }

    fn parse_data_type(s: &str) -> DataType {
        match s.to_lowercase().as_str() {
            "number" => DataType::Number,
            "string" => DataType::String,
            "boolean" | "bool" => DataType::Boolean,
            "event" => DataType::Event,
            "array" => DataType::Array,
            _ => DataType::Any,
        }
    }

    fn parse_port(&self, table: &Table) -> Result<PortDefinition> {
        let id: String = table.get("id").map_err(|_| anyhow!("端口缺少id"))?;
        let name: String = table.get("name").map_err(|_| anyhow!("端口缺少name"))?;
        let type_str: String = table.get("type").unwrap_or_else(|_| "any".to_string());

        Ok(PortDefinition {
            id,
            name,
            data_type: Self::parse_data_type(&type_str),
            default: self.lua_to_value(table.get("default").ok())?,
            description: table.get("description").unwrap_or_default(),
            required: table.get("required").unwrap_or(false),
            multiple: table.get("multiple").unwrap_or(false),
            element_type: table
                .get::<String>("element_type")
                .ok()
                .map(|s| Self::parse_data_type(&s)),
            min: table.get("min").ok(),
            max: table.get("max").ok(),
        })
    }

    fn parse_properties(&self, table: &Table) -> Result<Vec<PropertyDefinition>> {
        let mut props = Vec::new();

        if let Ok(props_table) = table.get::<Table>("properties") {
            for pair in props_table.pairs::<i64, Table>() {
                let (_, prop_table) = pair.map_err(lua_err)?;
                let prop = self.parse_property(&prop_table)?;
                props.push(prop);
            }
        }

        Ok(props)
    }

    fn parse_property(&self, table: &Table) -> Result<PropertyDefinition> {
        let id: String = table.get("id").map_err(|_| anyhow!("属性缺少id"))?;
        let name: String = table.get("name").map_err(|_| anyhow!("属性缺少name"))?;
        let type_str: String = table.get("type").unwrap_or_else(|_| "any".to_string());

        Ok(PropertyDefinition {
            id,
            name,
            data_type: Self::parse_data_type(&type_str),
            default: self.lua_to_value(table.get("default").ok())?,
            description: table.get("description").unwrap_or_default(),
            min: table.get("min").ok(),
            max: table.get("max").ok(),
        })
    }

    fn lua_to_value(&self, lua_val: Option<LuaValue>) -> Result<Value> {
        match lua_val {
            None => Ok(Value::Nil),
            Some(LuaValue::Nil) => Ok(Value::Nil),
            Some(LuaValue::Boolean(b)) => Ok(Value::Boolean(b)),
            Some(LuaValue::Integer(i)) => Ok(Value::Number(i as f64)),
            Some(LuaValue::Number(n)) => Ok(Value::Number(n)),
            Some(LuaValue::String(s)) => Ok(Value::String(s.to_str().map_err(lua_err)?.to_string())),
            Some(LuaValue::Table(t)) => {
                // 检查是数组还是对象
                let mut is_array = true;
                let mut max_index = 0i64;

                for pair in t.clone().pairs::<LuaValue, LuaValue>() {
                    let (k, _) = pair.map_err(lua_err)?;
                    if let LuaValue::Integer(i) = k {
                        if i > max_index {
                            max_index = i;
                        }
                    } else {
                        is_array = false;
                        break;
                    }
                }

                if is_array && max_index > 0 {
                    let mut arr = Vec::new();
                    for i in 1..=max_index {
                        let v = t.get::<LuaValue>(i).map_err(lua_err)?;
                        arr.push(self.lua_to_value(Some(v))?);
                    }
                    Ok(Value::Array(arr))
                } else {
                    let mut map = std::collections::HashMap::new();
                    for pair in t.pairs::<String, LuaValue>() {
                        let (k, v) = pair.map_err(lua_err)?;
                        map.insert(k, self.lua_to_value(Some(v))?);
                    }
                    Ok(Value::Object(map))
                }
            }
            _ => Ok(Value::Nil),
        }
    }
}

impl Default for ScriptParser {
    fn default() -> Self {
        Self::new().expect("创建Lua运行时失败")
    }
}
