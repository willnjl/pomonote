use comfy_table::{Table, presets::UTF8_FULL, Cell, Color, Attribute};
use crate::models::todo::{Todo, TodoStatus};

/// Display todos in a formatted table
pub fn display_todos(todos: &[Todo]) {
    if todos.is_empty() {
        println!("\nNo todos yet! Add one with: pomonote add \"Your task description\"");
        return;
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    
    // Add header
    table.set_header(vec![
        Cell::new("ID").add_attribute(Attribute::Bold),
        Cell::new("Description").add_attribute(Attribute::Bold),
        Cell::new("Status").add_attribute(Attribute::Bold),
    ]);

    // Add todos
    for todo in todos {
        let status_cell = match todo.status {
            TodoStatus::Pending => Cell::new(&todo.status).fg(Color::Yellow),
            TodoStatus::InProgress => Cell::new(&todo.status).fg(Color::Cyan),
            TodoStatus::Completed => Cell::new(&todo.status).fg(Color::Green),
        };

        table.add_row(vec![
            Cell::new(todo.id),
            Cell::new(&todo.description),
            status_cell,
        ]);
    }

    println!("\n{}", table);
}
