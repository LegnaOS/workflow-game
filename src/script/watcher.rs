//! 脚本文件监听器 - 热重载

use anyhow::Result;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

/// 脚本文件监听器
pub struct ScriptWatcher {
    _watcher: RecommendedWatcher,
    receiver: Receiver<Result<Event, notify::Error>>,
}

impl ScriptWatcher {
    pub fn new<P: AsRef<Path>>(script_dir: P) -> Result<Self> {
        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default().with_poll_interval(Duration::from_secs(1)),
        )?;

        watcher.watch(script_dir.as_ref(), RecursiveMode::Recursive)?;

        Ok(Self {
            _watcher: watcher,
            receiver: rx,
        })
    }

    /// 获取变化的文件列表
    pub fn poll_changes(&self) -> Vec<PathBuf> {
        let mut changed = Vec::new();

        while let Ok(result) = self.receiver.try_recv() {
            if let Ok(event) = result {
                for path in event.paths {
                    if path.extension().map(|e| e == "lua").unwrap_or(false) {
                        if !changed.contains(&path) {
                            changed.push(path);
                        }
                    }
                }
            }
        }

        changed
    }
}

