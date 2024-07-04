use anyhow::{Context, Result};
use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::info;
use ratatui::{
    backend::{self, CrosstermBackend, TestBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{block::Title, Block, Borders, List, ListDirection, ListItem, ListState},
    Frame, Terminal,
};
use std::{
    fs,
    io::{self, stdout, Stdout},
};

use crate::config::{Command, Config, Window};

fn draw_menu(f: &mut Frame, config: &Config, selected: usize) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    let items: Vec<ListItem> = config
        .windows
        .iter()
        .map(|i| ListItem::new(i.name.as_str()))
        .collect();
    let mut state = ListState::default().with_selected(Some(selected));
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Peek Menu"))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
        .direction(ListDirection::TopToBottom);

    f.render_stateful_widget(list, chunks[0], &mut state);
}

pub fn main_menu(config: &Config) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut selected = 0;

    loop {
        terminal.draw(|f| draw_menu(f, config, selected))?;

        if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down => {
                    if selected < config.windows.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Enter => {
                    // Implement window opening logic here
                    // For now, just print the selected window
                    println!("Selected window: {}", config.windows[selected].name);
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
pub fn read_bash_history() -> Result<Vec<Command>> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Failed to determine home directory"))?;
    let history_file = home_dir.join(".bash_history");
    let history_content = fs::read_to_string(&history_file)
        .with_context(|| format!("Failed to read bash history file: {:?}", history_file))?;
    let recent_commands: Vec<Command> = history_content
        .lines()
        .map(|line| Command {
            command: line.trim().to_string(),
        })
        .collect();
    Ok(recent_commands)
}
