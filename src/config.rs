use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::tui::read_bash_history;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub command_history_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            command_history_path: "../Recent_command.txt".to_owned(),
        }
    }
}

pub fn load_config(file_path: &str) -> Result<Config> {
    if !Path::new(file_path).exists() {
        let default_config = Config::default();
        save_config(file_path, &default_config)
            .with_context(|| format!("Failed to create default config file: {}", file_path))?;
        return Ok(default_config);
    }

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
