mod cli;
mod commands;
mod display;
mod models;

use clap::Parser;
use cli::{Cli, Commands};
use display::display_todos;
use models::todo::{Todo, TodoStatus};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load existing todos (placeholder - replace with actual persistence later)
    let todos = get_todos();

    // If no command provided, display the todo table
    match cli.command {
        None => {
            display_todos(&todos);
        }
        Some(Commands::Add { description }) => {
            commands::add::add_todo(description)?;
        }
        Some(Commands::Remove { id }) => {
            commands::remove::remove_todo(id)?;
        }
        Some(Commands::Start { id }) => {
            commands::start::start_todo(id)?;
        }
    }

    Ok(())
}

// Placeholder function to get todos - replace with actual persistence later
fn get_todos() -> Vec<Todo> {
    vec![
        Todo {
            id: 1,
            description: "Complete Rust CLI project".to_string(),
            status: TodoStatus::InProgress,
        },
        Todo {
            id: 2,
            description: "Add data persistence".to_string(),
            status: TodoStatus::Pending,
        },
        Todo {
            id: 3,
            description: "Write tests".to_string(),
            status: TodoStatus::Pending,
        },
    ]
}
