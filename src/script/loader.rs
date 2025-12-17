//! 脚本加载器 - 处理GBK/UTF-8编码

use anyhow::{Context, Result};
use encoding_rs::{GBK, UTF_8};
use std::fs;
use std::path::Path;

/// 脚本加载器
pub struct ScriptLoader;

impl ScriptLoader {
    /// 加载脚本文件，自动检测编码
    pub fn load<P: AsRef<Path>>(path: P) -> Result<String> {
        let path = path.as_ref();
        let bytes = fs::read(path)
            .with_context(|| format!("无法读取脚本文件: {}", path.display()))?;

        // 尝试检测编码
        let content = Self::decode_content(&bytes);
        Ok(content)
    }

    /// 解码内容，优先尝试UTF-8，失败则使用GBK
    fn decode_content(bytes: &[u8]) -> String {
        // 首先尝试UTF-8
        if let Ok(s) = std::str::from_utf8(bytes) {
            return s.to_string();
        }

        // 检查BOM
        if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
            // UTF-8 BOM
            if let Ok(s) = std::str::from_utf8(&bytes[3..]) {
                return s.to_string();
            }
        }

        // 尝试GBK解码
        let (decoded, _, had_errors) = GBK.decode(bytes);
        if !had_errors {
            return decoded.into_owned();
        }

        // 最后尝试有损UTF-8解码
        let (decoded, _, _) = UTF_8.decode(bytes);
        decoded.into_owned()
    }

    /// 扫描目录下的所有Lua脚本
    pub fn scan_scripts<P: AsRef<Path>>(dir: P) -> Result<Vec<std::path::PathBuf>> {
        let dir = dir.as_ref();
        let mut scripts = Vec::new();

        if !dir.exists() {
            fs::create_dir_all(dir)
                .with_context(|| format!("无法创建脚本目录: {}", dir.display()))?;
            return Ok(scripts);
        }

        Self::scan_recursive(dir, &mut scripts)?;
        Ok(scripts)
    }

    fn scan_recursive(dir: &Path, scripts: &mut Vec<std::path::PathBuf>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                Self::scan_recursive(&path, scripts)?;
            } else if let Some(ext) = path.extension() {
                if ext == "lua" {
                    scripts.push(path);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_decode() {
        let content = "-- 中文注释\nreturn {}";
        let decoded = ScriptLoader::decode_content(content.as_bytes());
        assert_eq!(decoded, content);
    }
}

