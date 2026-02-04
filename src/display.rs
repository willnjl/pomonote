use ratatui::{
    layout::{ Constraint, Direction, Layout },
    style::{ Color, Style },
    widgets::{ Block, Borders, Cell, Paragraph, Row, Table },
    Frame,
};

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

    let header_cells = ["ID", "Description", "Status", "Timer"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = todos.iter().map(|item| {
        let height = item.description
            .chars()
            .filter(|c| *c == '\n')
            .count()
            .max(1) as u16;
        let status_style = match item.status {
            TodoStatus::Pending => Style::default().fg(Color::Yellow),
            TodoStatus::InProgress => Style::default().fg(Color::Cyan),
            TodoStatus::Completed => Style::default().fg(Color::Green),
        };
        let cells = vec![
            Cell::from(item.id.to_string()),
            Cell::from(item.description.clone()),
            Cell::from(item.status.to_string()).style(status_style),
            Cell::from(item.timer.as_ref().map_or("--:--".to_string(), |t| t.output()))
        ];
        Row::new(cells).height(height)
    });

    let table = Table::new(rows, [
        Constraint::Percentage(10),
        Constraint::Percentage(50),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ])
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Todos"));

    let output = Paragraph::new(output_buffer)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Output"));

    let input = Paragraph::new(input_buffer)
        .style(Style::default().fg(Color::LightBlue))
        .block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(table, chunks[0]);
    f.render_widget(output, chunks[1]);
    f.render_widget(input, chunks[2]);
}
