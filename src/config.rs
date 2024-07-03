use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub command: String,
    pub usage_count: u32,
    pub last_used: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub commands: Vec<Command>,
    pub windows: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            commands: Vec::new(),
            windows: vec!["Most Recent".to_string(), "Most Used".to_string()],
        }
    }
}

pub fn load_config(file_path: &str) -> Result<Config> {
    let config_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read config file: {}", file_path))?;
    let config: Config =
        serde_yaml::from_str(&config_content).with_context(|| "Failed to parse config file")?;
    Ok(config)
}

pub fn save_config(file_path: &str, config: &Config) -> Result<()> {
    let config_content =
        serde_yaml::to_string(config).with_context(|| "Failed to serialize config")?;
    fs::write(file_path, config_content)
        .with_context(|| format!("Failed to write config file: {}", file_path))?;
    Ok(())
}
