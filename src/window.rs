use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Window {
    pub name: String,
    pub commands: Vec<String>,
    pub selected: usize,
    // TODO: path is use to open with vim
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Windows {
    pub windows: Vec<Window>,
}

impl Windows {
    pub fn new() -> Self {
        let mut s = Self {
            windows: Vec::new(),
        };
        s.load_from_path().expect("Failed to parse");
        s.windows.push(Window::new(
            "history".to_owned(),
            Self::read_bash_history(0, 100).unwrap(),
            100,
            PathBuf::new(),
        ));
        s
    }

    pub fn load_from_path(&mut self) -> Result<()> {
        let mut current_dir = std::env::current_dir()?;
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Failed to determine home directory"))?;

        loop {
            self.search_and_load_windows(&current_dir)?;

            if current_dir == home_dir {
                break;
            }

            if !current_dir.pop() {
                break;
            }
        }

        Ok(())
    }

    fn search_and_load_windows(&mut self, dir: &Path) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|name| name.to_str()) {
                    if filename.starts_with('.') && filename.ends_with(".borrow") {
                        let window_name = filename
                            .trim_start_matches('.')
                            .trim_end_matches(".borrow")
                            .to_string();
                        let commands = self.read_commands_from_file(&path)?;

                        self.windows
                            .push(Window::new(window_name, commands, 0, path));
                    }
                }
            }
        }
        Ok(())
    }

    fn read_commands_from_file(&self, path: &Path) -> Result<Vec<String>> {
        let file = fs::File::open(path)
            .with_context(|| format!("Failed to open window file: {:?}", path))?;
        let reader = io::BufReader::new(file);

        let commands: Vec<String> = reader
            .lines()
            .collect::<Result<_, _>>()
            .with_context(|| format!("Failed to read window file: {:?}", path))?;

        Ok(commands)
    }
    fn read_bash_history(start: usize, end: usize) -> Result<Vec<String>> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Failed to determine home directory"))?;
        let history_file = home_dir.join(".bash_history");
        let history_content = fs::read_to_string(&history_file)
            .with_context(|| format!("Failed to read bash history file: {:?}", history_file))?;

        let all_commands: Vec<String> = history_content
            .lines()
            .map(|line| line.to_string())
            .collect();
        let total_commands = all_commands.len();

        if start >= total_commands || end >= total_commands || start > end {
            return Err(anyhow::anyhow!("Invalid start or end indices"));
        }

        let rev_start = total_commands - end - 1;
        let rev_end = total_commands - start;

        let recent_commands = all_commands[rev_start..rev_end].to_vec();

        Ok(recent_commands)
    }
}

impl Default for Windows {
    fn default() -> Self {
        Self::new()
    }
}

impl Window {
    pub fn new(name: String, commands: Vec<String>, selected: usize, path: PathBuf) -> Self {
        Self {
            name,
            commands,
            selected,
            path,
        }
    }
}
