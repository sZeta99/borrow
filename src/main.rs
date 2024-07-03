use anyhow::{Context, Error};
use config::{load_config, save_config};
use log::debug;
use tui::main_menu;

extern crate core;
extern crate crossterm;
extern crate serde;
extern crate serde_yaml;

pub mod command;
pub mod config;
pub mod tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let config_path = "config.yaml";
    debug!("Loading configuration from {}", config_path);

    let config = load_config(config_path).with_context(|| "Failed to load configuration")?;

    main_menu(&config)?;

    save_config(config_path, &config).with_context(|| "Failed to save configuration")?;

    Ok(())
}
