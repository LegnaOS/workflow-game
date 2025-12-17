//! 脚本注册表 - 管理所有Block定义

use crate::script::{BlockDefinition, ScriptLoader, ScriptParser};
use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// 脚本注册表
pub struct ScriptRegistry {
    /// 所有Block定义 (key = meta.id)
    definitions: HashMap<String, BlockDefinition>,
    /// 分类索引 (category -> [meta.id])
    categories: HashMap<String, Vec<String>>,
    /// 脚本目录
    script_dir: PathBuf,
    /// 解析器
    parser: ScriptParser,
}

impl ScriptRegistry {
    pub fn new<P: AsRef<Path>>(script_dir: P) -> Result<Self> {
        let script_dir = script_dir.as_ref().to_path_buf();
        let parser = ScriptParser::new()?;

        let mut registry = Self {
            definitions: HashMap::new(),
            categories: HashMap::new(),
            script_dir,
            parser,
        };

        registry.reload_all()?;
        Ok(registry)
    }

    /// 重新加载所有脚本
    pub fn reload_all(&mut self) -> Result<()> {
        self.definitions.clear();
        self.categories.clear();

        let scripts = ScriptLoader::scan_scripts(&self.script_dir)?;

        for script_path in scripts {
            if let Err(e) = self.load_script(&script_path) {
                log::warn!("加载脚本失败 {}: {}", script_path.display(), e);
            }
        }

        log::info!(
            "已加载 {} 个Block定义, {} 个分类",
            self.definitions.len(),
            self.categories.len()
        );

        Ok(())
    }

    /// 加载单个脚本
    pub fn load_script(&mut self, path: &Path) -> Result<String> {
        let content = ScriptLoader::load(path)?;
        let definition = self.parser.parse(&content, path)?;

        let id = definition.meta.id.clone();
        let category = definition.meta.category.clone();

        // 更新分类索引
        self.categories
            .entry(category)
            .or_default()
            .push(id.clone());

        // 存储定义
        self.definitions.insert(id.clone(), definition);

        Ok(id)
    }

    /// 重新加载单个脚本(热重载)
    pub fn reload_script(&mut self, path: &Path) -> Result<()> {
        // 先移除旧的定义
        if let Some(old_def) = self.find_by_path(path) {
            let old_id = old_def.meta.id.clone();
            let old_category = old_def.meta.category.clone();

            self.definitions.remove(&old_id);
            if let Some(ids) = self.categories.get_mut(&old_category) {
                ids.retain(|id| id != &old_id);
            }
        }

        // 加载新定义
        self.load_script(path)?;
        Ok(())
    }

    /// 根据路径查找定义
    fn find_by_path(&self, path: &Path) -> Option<&BlockDefinition> {
        let path_str = path.to_string_lossy();
        self.definitions
            .values()
            .find(|def| def.script_path == path_str)
    }

    /// 获取Block定义
    pub fn get(&self, id: &str) -> Option<&BlockDefinition> {
        self.definitions.get(id)
    }

    /// 获取所有定义
    pub fn all(&self) -> impl Iterator<Item = &BlockDefinition> {
        self.definitions.values()
    }

    /// 获取所有分类
    pub fn categories(&self) -> impl Iterator<Item = (&String, &Vec<String>)> {
        self.categories.iter()
    }

    /// 获取分类下的所有Block
    pub fn get_by_category(&self, category: &str) -> Vec<&BlockDefinition> {
        self.categories
            .get(category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.definitions.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 获取脚本目录
    pub fn script_dir(&self) -> &Path {
        &self.script_dir
    }
}

