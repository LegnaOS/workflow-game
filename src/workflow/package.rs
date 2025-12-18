//! 游戏数据包模块
//!
//! .lpak (Legna Package) 格式：
//! - 将工作流 + 所有使用的脚本打包成单一加密文件
//! - 播放器从内存加载，无法提取原始脚本
//!
//! 文件结构:
//! [8字节魔数: "LEGNAPAK"]
//! [4字节版本: 0x0001]
//! [N字节: AES加密的JSON数据]

use super::Workflow;
use crate::script::{ScriptLoader, ScriptRegistry};
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// 魔数
const MAGIC: &[u8; 8] = b"LEGNAPAK";
/// 版本
const VERSION: u32 = 1;
/// 加密密钥（与 LZ 格式不同，更长更安全）
const PACKAGE_KEY: &str = "LegnaGamePackage2024";

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

/// 游戏数据包内容（序列化到JSON）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamePackage {
    /// 游戏名称
    pub name: String,
    /// 版本信息
    pub version: String,
    /// 工作流数据
    pub workflow: Workflow,
    /// 脚本源码（key = script_id, value = Lua源码）
    pub scripts: HashMap<String, String>,
}

impl GamePackage {
    /// 从工作流和注册表创建游戏包
    /// 只收集工作流中实际使用的脚本
    pub fn from_workflow(
        workflow: &Workflow,
        registry: &ScriptRegistry,
        name: &str,
        version: &str,
    ) -> Result<Self> {
        let mut scripts = HashMap::new();

        // 收集所有使用到的 script_id
        let used_scripts: std::collections::HashSet<_> = workflow
            .blocks
            .values()
            .map(|b| b.script_id.clone())
            .collect();

        // 加载每个脚本的源码
        for script_id in used_scripts {
            if let Some(def) = registry.get(&script_id) {
                let source = ScriptLoader::load(&def.script_path)?;
                scripts.insert(script_id, source);
            }
        }

        Ok(Self {
            name: name.to_string(),
            version: version.to_string(),
            workflow: workflow.to_distributable(), // 只读版本
            scripts,
        })
    }

    /// 保存到文件（加密）
    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string(self)?;
        let encrypted = Self::encrypt(&json)?;

        // 构建文件
        let mut file_data = Vec::with_capacity(12 + encrypted.len());
        file_data.extend_from_slice(MAGIC);
        file_data.extend_from_slice(&VERSION.to_le_bytes());
        file_data.extend_from_slice(&encrypted);

        fs::write(path, file_data)?;
        Ok(())
    }

    /// 从文件加载（解密）
    pub fn load(path: &Path) -> Result<Self> {
        let data = fs::read(path)?;

        // 检查最小长度
        if data.len() < 12 {
            return Err(anyhow!("文件太小，不是有效的游戏数据包"));
        }

        // 检查魔数
        if &data[0..8] != MAGIC {
            return Err(anyhow!("不是有效的游戏数据包文件"));
        }

        // 检查版本
        let version = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
        if version > VERSION {
            return Err(anyhow!("数据包版本过新，请更新播放器"));
        }

        // 解密
        let json = Self::decrypt(&data[12..])?;
        let package: GamePackage = serde_json::from_str(&json)?;

        Ok(package)
    }

    /// 从内存中的数据加载（用于嵌入式资源）
    pub fn load_from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < 12 || &data[0..8] != MAGIC {
            return Err(anyhow!("无效的游戏数据包"));
        }

        let json = Self::decrypt(&data[12..])?;
        let package: GamePackage = serde_json::from_str(&json)?;
        Ok(package)
    }

    /// AES-128-CBC 加密
    fn encrypt(data: &str) -> Result<Vec<u8>> {
        let (key, iv) = Self::derive_key_iv();
        let data_bytes = data.as_bytes();

        let block_size = 16;
        let padded_len = (data_bytes.len() / block_size + 1) * block_size;
        let mut buf = vec![0u8; padded_len];
        buf[..data_bytes.len()].copy_from_slice(data_bytes);

        let ct = Aes128CbcEnc::new(&key.into(), &iv.into())
            .encrypt_padded_mut::<Pkcs7>(&mut buf, data_bytes.len())
            .map_err(|_| anyhow!("加密失败"))?;

        Ok(ct.to_vec())
    }

    /// AES-128-CBC 解密
    fn decrypt(data: &[u8]) -> Result<String> {
        let (key, iv) = Self::derive_key_iv();
        let mut buf = data.to_vec();

        let pt = Aes128CbcDec::new(&key.into(), &iv.into())
            .decrypt_padded_mut::<Pkcs7>(&mut buf)
            .map_err(|_| anyhow!("解密失败：数据包损坏或版本不兼容"))?;

        String::from_utf8(pt.to_vec()).map_err(|_| anyhow!("解密失败：数据格式错误"))
    }

    /// 派生密钥和IV
    fn derive_key_iv() -> ([u8; 16], [u8; 16]) {
        let key_bytes = PACKAGE_KEY.as_bytes();
        let mut key = [0u8; 16];
        let mut iv = [0u8; 16];

        for i in 0..16 {
            key[i] = key_bytes[i % key_bytes.len()];
            iv[i] = key_bytes[(i + 7) % key_bytes.len()] ^ (i as u8 * 17);
        }

        (key, iv)
    }
}

