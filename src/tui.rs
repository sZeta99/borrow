use anyhow::{anyhow, Context, Error, Result};
use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::{self, stdout, Stdout},
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    config::Config,
    draw::{draw_menu, draw_window},
    window::Windows,
};

pub struct App {
    pub config: Config,
    pub windows: Windows,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub selected: usize,
}
impl App {
    pub fn main_menu(&mut self) -> Result<Option<String>> {
        loop {
            self.terminal
                .draw(|f| draw_menu(f, &self.windows.windows, self.selected))?;

            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('v') => {
                        // TODO: open vim for selected window
                        disable_raw_mode()?;
                        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;

                        Self::run_vim(&self.windows.windows[self.selected].path)?;

                        execute!(self.terminal.backend_mut(), EnterAlternateScreen)?;
                        enable_raw_mode()?;
                    }
                    KeyCode::Char('V') => {
                        disable_raw_mode()?;
                        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
                        self.terminal.show_cursor()?;
                        Self::run_vim(&self.windows.windows[self.selected].path)?;
                        enable_raw_mode()?;
                        execute!(self.terminal.backend_mut(), EnterAlternateScreen)?;

                        break;
                    }

                    KeyCode::Down => {
                        if self.selected < self.windows.windows.len() - 1 {
                            self.selected += 1;
                        } else {
                            self.selected = 0;
                        }
                    }
                    KeyCode::Up => {
                        if self.selected > 0 {
                            self.selected -= 1;
                        } else {
                            self.selected = self.windows.windows.len() - 1;
                        }
                    }
                    KeyCode::Enter => {
                        let _ = self.window(self.selected);
                    }
                    _ => {}
                }
            }
        }

        Ok(None)
    }

    fn window(&mut self, window: usize) -> Result<Option<String>> {
        let window = &mut self.windows.windows[window];

        loop {
            self.terminal.draw(|f| draw_window(f, window))?;

            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('v') => {
                        // TODO: open vim for this window
                        Self::run_vim(&window.path)?;
                    }
                    KeyCode::Char('V') => {
                        disable_raw_mode()?;
                        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
                        self.terminal.show_cursor()?;
                        Self::run_vim(&window.path)?;
                        enable_raw_mode()?;
                        execute!(self.terminal.backend_mut(), EnterAlternateScreen)?;

                        break;
                    }

                    KeyCode::Down => {
                        if window.selected < window.commands.len() - 1 {
                            window.selected += 1;
                        } else {
                            window.selected = 0;
                        }
                    }
                    KeyCode::Up => {
                        if window.selected > 0 {
                            window.selected -= 1;
                        } else {
                            window.selected = window.commands.len() - 1;
                        }
                    }
                    KeyCode::Enter => {
                        //disable_raw_mode()?;
                        //execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
                        //self.terminal.show_cursor()?;
                        //Self::run_command(window.commands[window.selected].clone(), None)?;
                        return Ok(Some(window.commands[window.selected].clone()));
                    }

                    _ => {}
                }
            }
        }

        Ok(None)
    }

    fn run_vim(file: &Path) -> Result<(), Error> {
        let file = file
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("File path is not valid"))?;

        // Err(anyhow::anyhow!(Command::new("vim").arg(file).exec()))
        let status = Command::new("vim")
            .arg(file)
            .status()
            .with_context(|| "Failed to execute vim")?;

        if !status.success() {
            return Err(anyhow::anyhow!("vim did not exit successfully"));
        }
        Ok(())
    }
    fn run_command(command: String, _scope: Option<String>) -> Result<(), Error> {
        Err(anyhow::anyhow!(Command::new(command).exec()))
    }
}
