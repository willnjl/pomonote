use ratatui::{
    layout::{ Constraint, Direction, Layout },
    style::{ Color, Style },
    widgets::{ Block, Borders, Cell, Paragraph, Row, Table },
    Frame,
};

use super::table::table_str;

use crate::models::todo::{ Todo, TodoStatus };

pub fn ui(f: &mut Frame, todos: &[Todo], input_buffer: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(10)].as_ref())
        .split(f.size());

    let table_str = table_str(todos);
    let table_widget = Paragraph::new(table_str).block(Block::default().borders(Borders::ALL));

    f.render_widget(table_widget, chunks[0]);

    let input = Paragraph::new(input_buffer)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
}
