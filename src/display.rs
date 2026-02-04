use ratatui::{
    layout::{ Constraint, Direction, Layout },
    style::{ Color, Style },
    widgets::{ Block, Borders, Cell, Paragraph, Row, Table },
    Frame,
};

use super::table::table_str;

use crate::models::todo::{ Todo, TodoStatus };

pub fn ui(f: &mut Frame, todos: &[Todo], input_buffer: &str, output_buffer: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(65),
                Constraint::Percentage(20),
                Constraint::Percentage(15),
            ].as_ref()
        )
        .split(f.size());

    let table_str = table_str(todos);
    let table_widget = Paragraph::new(table_str).block(Block::default().borders(Borders::ALL));

    let output = Paragraph::new(output_buffer)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Output"));

    let input = Paragraph::new(input_buffer)
        .style(Style::default().fg(Color::LightBlue))
        .block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(table_widget, chunks[0]);
    f.render_widget(output, chunks[1]);
    f.render_widget(input, chunks[2]);
}
