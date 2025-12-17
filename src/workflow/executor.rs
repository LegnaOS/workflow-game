//! 工作流执行引擎 - 实时执行Lua脚本

use crate::script::{ScriptRegistry, Value};
use crate::workflow::Workflow;
use anyhow::{anyhow, Result};
use mlua::{Lua, Table, Value as LuaValue};
use std::collections::HashMap;
use uuid::Uuid;

/// 将mlua错误转换为anyhow错误
fn lua_err(e: mlua::Error) -> anyhow::Error {
    anyhow!("Lua执行错误: {}", e)
}

/// 工作流执行引擎
pub struct WorkflowExecutor {
    lua: Lua,
}

impl WorkflowExecutor {
    pub fn new() -> Result<Self> {
        let lua = Lua::new();
        Ok(Self { lua })
    }

    /// 执行整个工作流(按拓扑顺序)
    pub fn execute_all(
        &self,
        workflow: &mut Workflow,
        registry: &ScriptRegistry,
    ) -> Result<()> {
        let order = workflow.execution_order.clone();
        
        for block_id in order {
            self.execute_block(workflow, registry, block_id)?;
        }

        workflow.dirty_blocks.clear();
        Ok(())
    }

    /// 只执行脏Block
    pub fn execute_dirty(
        &self,
        workflow: &mut Workflow,
        registry: &ScriptRegistry,
    ) -> Result<()> {
        // 按拓扑顺序执行所有Block
        let order = workflow.execution_order.clone();

        // Debug: 打印执行顺序
        if !order.is_empty() {
            let names: Vec<String> = order.iter()
                .filter_map(|id| workflow.blocks.get(id))
                .map(|b| b.script_id.clone())
                .collect();
            log::debug!("执行顺序: {:?}", names);
        }

        for block_id in order {
            // 激活Block（用于流动动画）
            workflow.activate_block(block_id);
            self.execute_block(workflow, registry, block_id)?;
        }

        workflow.dirty_blocks.clear();
        Ok(())
    }

    /// 执行单个Block
    fn execute_block(
        &self,
        workflow: &mut Workflow,
        registry: &ScriptRegistry,
        block_id: Uuid,
    ) -> Result<()> {
        let block = match workflow.blocks.get(&block_id) {
            Some(b) => b,
            None => return Ok(()),
        };

        let definition = match registry.get(&block.script_id) {
            Some(d) => d,
            None => {
                log::warn!("找不到Block定义: {}", block.script_id);
                return Ok(());
            }
        };

        // 收集输入值(从连接获取)
        let mut inputs: HashMap<String, Value> = block.input_values.clone();

        // 收集需要更新的连接值
        let input_connections: Vec<(String, Value)> = workflow.get_input_connections(block_id)
            .iter()
            .filter_map(|conn| {
                workflow.blocks.get(&conn.from_block)
                    .and_then(|source_block| {
                        source_block.output_values.get(&conn.from_port)
                            .map(|v| (conn.to_port.clone(), v.clone()))
                    })
            })
            .collect();

        for (port_id, value) in &input_connections {
            inputs.insert(port_id.clone(), value.clone());
        }

        // 更新Block的input_values以便UI显示
        if let Some(block) = workflow.blocks.get_mut(&block_id) {
            for (port_id, value) in input_connections {
                block.input_values.insert(port_id, value);
            }
        }

        // 重新获取block引用
        let block = match workflow.blocks.get(&block_id) {
            Some(b) => b,
            None => return Ok(()),
        };

        // 加载并执行Lua脚本
        let script_content = crate::script::ScriptLoader::load(&definition.script_path)?;
        let script_table: Table = self.lua.load(&script_content).eval().map_err(lua_err)?;

        // 获取block的state用于传递给Lua
        let block_state = block.state.clone();

        // 构建self表(包含properties和state)
        let self_table = self.lua.create_table().map_err(lua_err)?;

        let props_table = self.lua.create_table().map_err(lua_err)?;
        for (key, value) in &block.properties {
            props_table.set(key.as_str(), self.value_to_lua(value)?).map_err(lua_err)?;
        }
        self_table.set("properties", props_table).map_err(lua_err)?;

        // 添加state表
        let state_table = self.lua.create_table().map_err(lua_err)?;
        for (key, value) in &block_state {
            state_table.set(key.as_str(), self.value_to_lua(value)?).map_err(lua_err)?;
        }
        self_table.set("state", state_table).map_err(lua_err)?;

        // 构建inputs表
        let inputs_table = self.lua.create_table().map_err(lua_err)?;
        for (key, value) in &inputs {
            inputs_table.set(key.as_str(), self.value_to_lua(value)?).map_err(lua_err)?;
        }

        // 调用execute函数
        if let Ok(execute_fn) = script_table.get::<mlua::Function>("execute") {
            // Debug: 打印输入
            log::debug!("[{}] inputs: {:?}", block.script_id, inputs);

            let result: Table = execute_fn.call((self_table.clone(), inputs_table)).map_err(lua_err)?;

            // Debug: 打印输出
            let mut outputs: HashMap<String, Value> = HashMap::new();
            for pair in result.clone().pairs::<String, LuaValue>() {
                if let Ok((k, v)) = pair {
                    if let Ok(val) = self.lua_to_value(v) {
                        outputs.insert(k, val);
                    }
                }
            }
            log::debug!("[{}] outputs: {:?}", block.script_id, outputs);

            // 更新输出值和state
            if let Some(block) = workflow.blocks.get_mut(&block_id) {
                for pair in result.pairs::<String, LuaValue>() {
                    let (key, lua_val) = pair.map_err(lua_err)?;
                    let value = self.lua_to_value(lua_val)?;
                    block.output_values.insert(key, value);
                }

                // 从self.state中获取更新后的state
                if let Ok(updated_state) = self_table.get::<Table>("state") {
                    for pair in updated_state.pairs::<String, LuaValue>() {
                        if let Ok((key, lua_val)) = pair {
                            if let Ok(value) = self.lua_to_value(lua_val) {
                                block.state.insert(key, value);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn value_to_lua(&self, value: &Value) -> Result<LuaValue> {
        Ok(match value {
            Value::Nil => LuaValue::Nil,
            Value::Number(n) => LuaValue::Number(*n),
            Value::String(s) => LuaValue::String(self.lua.create_string(s).map_err(lua_err)?),
            Value::Boolean(b) => LuaValue::Boolean(*b),
            Value::Array(arr) => {
                let table = self.lua.create_table().map_err(lua_err)?;
                for (i, v) in arr.iter().enumerate() {
                    table.set(i + 1, self.value_to_lua(v)?).map_err(lua_err)?;
                }
                LuaValue::Table(table)
            }
            Value::Object(map) => {
                let table = self.lua.create_table().map_err(lua_err)?;
                for (k, v) in map {
                    table.set(k.as_str(), self.value_to_lua(v)?).map_err(lua_err)?;
                }
                LuaValue::Table(table)
            }
        })
    }

    fn lua_to_value(&self, lua_val: LuaValue) -> Result<Value> {
        Ok(match lua_val {
            LuaValue::Nil => Value::Nil,
            LuaValue::Boolean(b) => Value::Boolean(b),
            LuaValue::Integer(i) => Value::Number(i as f64),
            LuaValue::Number(n) => Value::Number(n),
            LuaValue::String(s) => Value::String(s.to_str().map_err(lua_err)?.to_string()),
            LuaValue::Table(t) => {
                let mut map = HashMap::new();
                for pair in t.pairs::<String, LuaValue>() {
                    let (k, v) = pair.map_err(lua_err)?;
                    map.insert(k, self.lua_to_value(v)?);
                }
                Value::Object(map)
            }
            _ => Value::Nil,
        })
    }
}

impl Default for WorkflowExecutor {
    fn default() -> Self {
        Self::new().expect("创建执行引擎失败")
    }
}

