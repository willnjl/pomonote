use comfy_table::{Table, presets::UTF8_FULL, Cell, Color, Attribute};
use crate::models::todo::{Todo, TodoStatus};

/// Display todos in a formatted table
pub fn table_str(todos: &[Todo]) -> String {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    
    // Add header
    table.set_header(vec![
        Cell::new("ID").add_attribute(Attribute::Bold),
        Cell::new("Description").add_attribute(Attribute::Bold),
        Cell::new("Status").add_attribute(Attribute::Bold),
        Cell::new("Timer").add_attribute(Attribute::Bold)
    ]);

    // Add todos
    for todo in todos {
        let status_cell = match todo.status {
            TodoStatus::Pending => Cell::new(&todo.status).fg(Color::Yellow),
            TodoStatus::InProgress => Cell::new(&todo.status).fg(Color::Cyan),
            TodoStatus::Completed => Cell::new(&todo.status).fg(Color::Green),
        };

        let timer_display = if let Some(ref timer) = todo.timer {
            timer.output()
        } else {
            "--:--".to_string()
        };

        table.add_row(vec![
            Cell::new(todo.id),
            Cell::new(&todo.description),
            status_cell,
            Cell::new(timer_display)
        ]);
    }

    table.to_string()
}
