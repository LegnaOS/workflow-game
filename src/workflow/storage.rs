//! 蓝图存储模块
//! 支持 .L (Legna明文) 和 .LZ (Legna加密) 格式
//! 文件命名：
//!   - xxx.L / xxx.LZ - 可编辑版本
//!   - xxx.dist.L / xxx.dist.LZ - 可分发版本（只读）

use super::Workflow;
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

/// 蓝图文件格式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlueprintFormat {
    /// .L 格式 - 明文JSON
    Legna,
    /// .LZ 格式 - 加密JSON
    LegnaEncrypted,
}

impl BlueprintFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "l" => Some(Self::Legna),
            "lz" => Some(Self::LegnaEncrypted),
            _ => None,
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Self::Legna => "L",
            Self::LegnaEncrypted => "LZ",
        }
    }
}

/// 蓝图存储
pub struct BlueprintStorage;

impl BlueprintStorage {
    /// 保存蓝图到文件（单份）
    pub fn save(workflow: &Workflow, path: &Path, password: Option<&str>) -> Result<()> {
        Self::save_internal(workflow, path, password)
    }

    /// 保存蓝图双份（可编辑 + 可分发）
    /// 返回 (可编辑路径, 可分发路径)
    pub fn save_dual(workflow: &Workflow, base_name: &str, encrypted: bool, password: Option<&str>) -> Result<(std::path::PathBuf, std::path::PathBuf)> {
        let ext = if encrypted { "LZ" } else { "L" };

        // 可编辑版本
        let editable_path = std::path::PathBuf::from(format!("{}.{}", base_name, ext));
        let mut editable = workflow.clone();
        editable.readonly = false;
        Self::save_internal(&editable, &editable_path, password)?;

        // 可分发版本（只读）
        let dist_path = std::path::PathBuf::from(format!("{}.dist.{}", base_name, ext));
        let distributable = workflow.to_distributable();
        Self::save_internal(&distributable, &dist_path, password)?;

        Ok((editable_path, dist_path))
    }

    /// 内部保存函数
    fn save_internal(workflow: &Workflow, path: &Path, password: Option<&str>) -> Result<()> {
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("L");

        let format = BlueprintFormat::from_extension(ext)
            .unwrap_or(BlueprintFormat::Legna);

        let json = serde_json::to_string_pretty(workflow)?;

        match format {
            BlueprintFormat::Legna => {
                fs::write(path, json)?;
            }
            BlueprintFormat::LegnaEncrypted => {
                let password = password.ok_or_else(|| anyhow!("加密文件需要密码"))?;
                if password.len() > 32 {
                    return Err(anyhow!("密码长度不能超过32位"));
                }
                let encrypted = Self::encrypt(&json, password)?;
                fs::write(path, encrypted)?;
            }
        }

        Ok(())
    }

    /// 从文件加载蓝图
    pub fn load(path: &Path, password: Option<&str>) -> Result<Workflow> {
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("L");

        let format = BlueprintFormat::from_extension(ext)
            .unwrap_or(BlueprintFormat::Legna);

        let content = fs::read(path)?;

        let json = match format {
            BlueprintFormat::Legna => {
                String::from_utf8(content)?
            }
            BlueprintFormat::LegnaEncrypted => {
                let password = password.ok_or_else(|| anyhow!("加密文件需要密码"))?;
                Self::decrypt(&content, password)?
            }
        };

        let workflow: Workflow = serde_json::from_str(&json)?;
        Ok(workflow)
    }

    /// 简单的XOR加密 (生产环境应使用AES)
    fn encrypt(data: &str, password: &str) -> Result<Vec<u8>> {
        let key = Self::derive_key(password);
        let mut encrypted = Vec::with_capacity(data.len() + 8);

        // 添加魔数标识
        encrypted.extend_from_slice(b"LEGNA_LZ");

        // XOR加密
        for (i, byte) in data.as_bytes().iter().enumerate() {
            encrypted.push(byte ^ key[i % key.len()]);
        }

        Ok(encrypted)
    }

    /// 解密
    fn decrypt(data: &[u8], password: &str) -> Result<String> {
        // 检查魔数
        if data.len() < 8 || &data[0..8] != b"LEGNA_LZ" {
            return Err(anyhow!("不是有效的LZ加密文件"));
        }

        let key = Self::derive_key(password);
        let encrypted = &data[8..];

        // XOR解密
        let decrypted: Vec<u8> = encrypted.iter()
            .enumerate()
            .map(|(i, byte)| byte ^ key[i % key.len()])
            .collect();

        String::from_utf8(decrypted)
            .map_err(|_| anyhow!("密码错误或文件损坏"))
    }

    /// 从密码派生密钥
    fn derive_key(password: &str) -> Vec<u8> {
        let mut key = vec![0u8; 32];
        for (i, byte) in password.as_bytes().iter().enumerate() {
            key[i % 32] ^= byte;
            key[(i + 1) % 32] = key[(i + 1) % 32].wrapping_add(*byte);
        }
        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let data = "Hello, World!";
        let password = "secret123";

        let encrypted = BlueprintStorage::encrypt(data, password).unwrap();
        let decrypted = BlueprintStorage::decrypt(&encrypted, password).unwrap();

        assert_eq!(data, decrypted);
    }
}

