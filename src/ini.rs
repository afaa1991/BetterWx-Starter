use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
pub struct AppConfig {
    pub programs: Vec<String>,
    pub auto_login: bool,
    pub close_first: bool,
}

impl AppConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| anyhow!("读取配置文件失败: {}",e))?;
        let mut config = AppConfig::default();
        for line in content.lines() {
            let line = line.trim();
            // 跳过注释和空行
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            // 解析键值对
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                if key.starts_with("program") {
                    config.programs.push(value.to_string());
                } else if key == "auto_login" {
                    config.auto_login = value.to_lowercase() == "true";
                } else if key == "close_first" {
                    config.close_first = value.to_lowercase() == "true";
                }
            }
        }
        Ok(config)
    }
}