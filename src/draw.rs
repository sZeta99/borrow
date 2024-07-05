use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListDirection, ListItem, ListState},
    Frame,
};

use crate::config::Window;

pub fn draw_window(f: &mut Frame, window: &Window) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    let items: Vec<ListItem> = window
        .commands
        .iter()
        .map(|cmd| ListItem::new(cmd.as_str()))
        .collect();
    let mut state = ListState::default().with_selected(Some(window.selected));

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

pub fn draw_menu(f: &mut Frame, windows: &[Window], selected: usize) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    let items: Vec<ListItem> = windows
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
