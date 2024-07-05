use std::io;

use anyhow::Context;
use config::{load_config, save_config};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::debug;
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::App;

extern crate core;
extern crate crossterm;
extern crate dirs;
extern crate serde;
extern crate serde_yaml;

pub mod command;
pub mod config;
pub mod draw;
pub mod tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let config_path = "config.yaml";
    debug!("Loading configuration from {}", config_path);

    let config = load_config(config_path).with_context(|| "Failed to load configuration")?;
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let mut app = App {
        config,
        terminal,
        selected: 0,
    };
    let command = app.main_menu()?;
    disable_raw_mode()?;
    execute!(app.terminal.backend_mut(), LeaveAlternateScreen)?;
    app.terminal.show_cursor()?;
    save_config(config_path, &app.config).with_context(|| "Failed to save configuration")?;

    if let Some(command) = command {
        println!("{}", command)
    }
    Ok(())
}
