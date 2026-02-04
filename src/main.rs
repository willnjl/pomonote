mod commands;
mod display;
mod models;

use display::display_todos;
use models::todo::{Todo, TodoStatus};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Pomonote - Your CLI Pomodoro Todo App!");
    println!("Commands: add <description> | remove <id> | start <id> | list | quit");
    println!("{}", "=".repeat(70));

    // Load existing todos (placeholder - replace with actual persistence later)
    let mut todos = get_todos();

    loop {
        // Display todos
        display_todos(&todos);

        // Prompt for command
        print!("\n> ");
        io::stdout().flush()?;

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        // Parse and execute command
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0].to_lowercase();

        match command.as_str() {
            "add" => {
                if parts.len() < 2 || parts[1].is_empty() {
                    println!("❌ Usage: add <description>");
                    continue;
                }
                let description = parts[1].to_string();
                commands::add::add_todo(&mut todos, description)?;
            }
            "remove" | "rm" => {
                if parts.len() < 2 {
                    println!("❌ Usage: remove <id>");
                    continue;
                }
                match parts[1].parse::<u32>() {
                    Ok(id) => commands::remove::remove_todo(&mut todos, id)?,
                    Err(_) => println!("❌ Invalid ID. Please provide a number."),
                }
            }
            "start" => {
                if parts.len() < 2 {
                    println!("❌ Usage: start <id>");
                    continue;
                }
                match parts[1].parse::<u32>() {
                    Ok(id) => commands::start::start_todo(&mut todos, id)?,
                    Err(_) => println!("❌ Invalid ID. Please provide a number."),
                }
            }
            "list" | "ls" => {
                // Just continue loop to redisplay
            }
            "quit" | "exit" | "q" => {
                println!("\n👋 Goodbye! Stay productive!");
                break;
            }
            "help" | "h" => {
                println!("\nAvailable commands:");
                println!("  add <description>  - Add a new todo");
                println!("  remove <id>        - Remove a todo by ID");
                println!("  start <id>         - Start a todo (mark as in progress)");
                println!("  list               - Show all todos");
                println!("  quit               - Exit the app");
            }
            _ => {
                println!("❌ Unknown command: '{}'. Type 'help' for available commands.", command);
            }
        }

        println!(); // Add spacing
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
