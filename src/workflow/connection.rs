//! 连接 - 两个端口之间的连线

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 连接
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub id: Uuid,
    pub from_block: Uuid,
    pub from_port: String,
    pub to_block: Uuid,
    pub to_port: String,
}

impl Connection {
    pub fn new(from_block: Uuid, from_port: String, to_block: Uuid, to_port: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            from_block,
            from_port,
            to_block,
            to_port,
        }
    }
}

/// 拖拽中的临时连接
#[derive(Debug, Clone)]
pub struct DraggingConnection {
    pub from_block: Uuid,
    pub from_port: String,
    pub is_output: bool,
    pub mouse_pos: (f32, f32),
}

