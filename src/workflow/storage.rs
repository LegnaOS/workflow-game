//! 蓝图存储模块
//! 支持 .L (Legna明文) 和 .LZ (Legna加密) 格式
//!
//! 两种保护机制：
//! 1. **打开密码**：存储在Workflow.password_hash中，打开时需要验证
//! 2. **加密**：.LZ文件使用"Legna"固定密钥AES加密，保护文件内容不被查看
//!
//! 文件命名：
//!   - xxx.L - 明文版本（可设置打开密码）
//!   - xxx.LZ - 加密版本（固定密钥加密 + 可设置打开密码）
//!   - xxx.dist.L / xxx.dist.LZ - 可分发版本（只读）

use super::Workflow;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

/// 固定加密密钥
const ENCRYPTION_KEY: &str = "Legna";

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

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

    /// 内部保存函数（加密使用固定密钥，不需要用户密码）
    fn save_internal(workflow: &Workflow, path: &Path, _password: Option<&str>) -> Result<()> {
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
                // 使用固定密钥加密
                let encrypted = Self::encrypt(&json)?;
                fs::write(path, encrypted)?;
            }
        }

        Ok(())
    }

    /// 从文件加载蓝图（加密文件自动解密，打开密码需要调用者验证）
    pub fn load(path: &Path, _password: Option<&str>) -> Result<Workflow> {
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
                // 使用固定密钥解密
                Self::decrypt(&content)?
            }
        };

        let workflow: Workflow = serde_json::from_str(&json)?;
        Ok(workflow)
    }

    /// AES-128-CBC加密（使用固定密钥"Legna"）
    fn encrypt(data: &str) -> Result<Vec<u8>> {
        let (key, iv) = Self::derive_key_iv();

        let data_bytes = data.as_bytes();
        // 计算需要的缓冲区大小（PKCS7填充）
        let block_size = 16;
        let padded_len = (data_bytes.len() / block_size + 1) * block_size;
        let mut buf = vec![0u8; padded_len];
        buf[..data_bytes.len()].copy_from_slice(data_bytes);

        let ct = Aes128CbcEnc::new(&key.into(), &iv.into())
            .encrypt_padded_mut::<Pkcs7>(&mut buf, data_bytes.len())
            .map_err(|_| anyhow!("加密失败"))?;

        // 添加魔数 + 加密数据
        let mut result = Vec::with_capacity(ct.len() + 8);
        result.extend_from_slice(b"LEGNA_LZ");
        result.extend_from_slice(ct);

        Ok(result)
    }

    /// AES-128-CBC解密（使用固定密钥"Legna"）
    fn decrypt(data: &[u8]) -> Result<String> {
        // 检查魔数
        if data.len() < 8 || &data[0..8] != b"LEGNA_LZ" {
            return Err(anyhow!("不是有效的LZ加密文件"));
        }

        let (key, iv) = Self::derive_key_iv();
        let encrypted = &data[8..];

        let mut buf = encrypted.to_vec();
        let pt = Aes128CbcDec::new(&key.into(), &iv.into())
            .decrypt_padded_mut::<Pkcs7>(&mut buf)
            .map_err(|_| anyhow!("解密失败：文件损坏"))?;

        String::from_utf8(pt.to_vec())
            .map_err(|_| anyhow!("解密失败：数据格式错误"))
    }

    /// 从固定密钥"Legna"派生AES-128密钥和IV
    fn derive_key_iv() -> ([u8; 16], [u8; 16]) {
        // 简单的密钥派生：重复密钥填充到16字节
        let key_bytes = ENCRYPTION_KEY.as_bytes();
        let mut key = [0u8; 16];
        let mut iv = [0u8; 16];

        for i in 0..16 {
            key[i] = key_bytes[i % key_bytes.len()];
            // IV使用密钥的反转+偏移
            iv[i] = key_bytes[key_bytes.len() - 1 - i % key_bytes.len()] ^ (i as u8);
        }

        (key, iv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let data = "Hello, World!";

        let encrypted = BlueprintStorage::encrypt(data).unwrap();
        let decrypted = BlueprintStorage::decrypt(&encrypted).unwrap();

        assert_eq!(data, decrypted);
    }
}

