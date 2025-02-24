use anyhow::{Context, Result};
use crossterm::event::{self, KeyCode};

use log::info;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    fs,
    io::{self, Stdout},
};

use crate::{
    config::{Config, Window},
    draw::{draw_menu, draw_window},
};

pub struct App {
    pub config: Config,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub selected: usize,
}
impl App {
    pub fn main_menu(&mut self) -> Result<Option<String>> {
        loop {
            self.terminal
                .draw(|f| draw_menu(f, &self.config.windows, self.selected))?;

            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => {
                        if self.selected < self.config.windows.len() - 1 {
                            self.selected += 1;
                        }
                    }
                    KeyCode::Up => {
                        if self.selected > 0 {
                            self.selected -= 1;
                        }
                    }
                    KeyCode::Enter => {
                        // Enter the selected window
                        return self.window(self.selected);
                    }
                    _ => {}
                }
            }
        }

        Ok(None)
    }

    fn window(&mut self, window: usize) -> Result<Option<String>> {
        let window = &mut self.config.windows[window];

        loop {
            self.terminal.draw(|f| draw_window(f, window))?;

            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => {
                        if window.selected < window.commands.len() - 1 {
                            window.selected += &1;
                        }
                    }
                    KeyCode::Up => {
                        if window.selected > 0 {
                            window.selected -= 1;
                        }
                    }
                    KeyCode::Enter => return Ok(Some(window.commands[window.selected].clone())),
                    _ => {}
                }
            }
        }

        Ok(None)
    }
}
pub fn read_bash_history(start: usize, end: usize) -> Result<Vec<String>> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Failed to determine home directory"))?;
    let history_file = home_dir.join(".bash_history");
    let history_content = fs::read_to_string(&history_file)
        .with_context(|| format!("Failed to read bash history file: {:?}", history_file))?;

    let recent_commands: Vec<String> = history_content
        .lines()
        .skip(start)
        .take(end - start + 1)
        .map(|line| line.to_string())
        .collect();

    Ok(recent_commands)
}
