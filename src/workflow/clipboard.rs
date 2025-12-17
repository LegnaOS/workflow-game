//! 剪贴板 - 复制粘贴Block

use super::{Block, Connection, Vec2};
use std::collections::HashMap;
use uuid::Uuid;

/// 剪贴板
#[derive(Debug, Clone, Default)]
pub struct Clipboard {
    blocks: Vec<Block>,
    connections: Vec<Connection>,
}

impl Clipboard {
    pub fn new() -> Self {
        Self::default()
    }

    /// 复制选中的Block和它们之间的连接
    pub fn copy(&mut self, blocks: &[&Block], connections: &[&Connection]) {
        self.blocks = blocks.iter().map(|b| (*b).clone()).collect();
        
        // 只复制被选中Block之间的连接
        let block_ids: std::collections::HashSet<Uuid> = 
            blocks.iter().map(|b| b.id).collect();
        
        self.connections = connections
            .iter()
            .filter(|c| block_ids.contains(&c.from_block) && block_ids.contains(&c.to_block))
            .map(|c| (*c).clone())
            .collect();
    }

    /// 粘贴,返回新的Block和Connection
    pub fn paste(&self, offset: Vec2) -> (Vec<Block>, Vec<Connection>) {
        if self.blocks.is_empty() {
            return (Vec::new(), Vec::new());
        }

        // 创建ID映射(旧ID -> 新ID)
        let mut id_map: HashMap<Uuid, Uuid> = HashMap::new();
        
        // 复制Block并生成新ID
        let new_blocks: Vec<Block> = self
            .blocks
            .iter()
            .map(|b| {
                let new_id = Uuid::new_v4();
                id_map.insert(b.id, new_id);
                
                Block {
                    id: new_id,
                    position: Vec2::new(b.position.x + offset.x, b.position.y + offset.y),
                    selected: true,
                    group_id: None,
                    ..b.clone()
                }
            })
            .collect();

        // 复制连接并更新ID
        let new_connections: Vec<Connection> = self
            .connections
            .iter()
            .filter_map(|c| {
                let from = id_map.get(&c.from_block)?;
                let to = id_map.get(&c.to_block)?;
                Some(Connection {
                    id: Uuid::new_v4(),
                    from_block: *from,
                    from_port: c.from_port.clone(),
                    to_block: *to,
                    to_port: c.to_port.clone(),
                })
            })
            .collect();

        (new_blocks, new_connections)
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
}

