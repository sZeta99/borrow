use anyhow::Result;
use crossterm::{
    event::{self, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use log::info;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

use crate::config::Config;

pub fn main_menu(config: &Config) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            let items: Vec<ListItem> = config
                .windows
                .iter()
                .map(|i| ListItem::new(i.as_str()))
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Peek Menu"))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");

            f.render_widget(list, chunks[0]);
        })?;

        if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(())
}
