mod commands;
mod display;
mod models;

use display::display_todos;
use models::todo::{Todo, TodoStatus};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Pomonote - Your CLI Pomodoro Todo App!");
    println!("Commands: add <desc> | start <id> | stop <id> | complete <id> | remove <id> | list | quit");
    println!("{}", "=".repeat(70));

    // Load existing todos (placeholder - replace with actual persistence later)
    let mut todos = get_todos();
    let mut input_buffer = String::new();

    // Initial display
    print!("\x1B[2J\x1B[1;1H");
    println!("Welcome to Pomonote - Your CLI Pomodoro Todo App!");
    println!("Commands: add <desc> | start <id> | stop <id> | complete <id> | remove <id> | list | quit");
    println!("{}", "=".repeat(70));
    display_todos(&todos);

    loop {
        // Only redraw if a timer is active
        let needs_redraw = todos.iter().any(|t| matches!(t.status, TodoStatus::InProgress));

        // Show prompt with current input
        if needs_redraw {
            // Clear screen and display todos
            print!("\x1B[2J\x1B[1;1H");
            println!("Welcome to Pomonote - Your CLI Pomodoro Todo App!");
            println!("Commands: add <desc> | start <id> | stop <id> | complete <id> | remove <id> | list | quit");
            println!("{}", "=".repeat(70));
            display_todos(&todos);
            print!("\n> {}", input_buffer);
            io::stdout().flush()?;
        }

        // Poll for input with 1 second timeout
        if event::poll(Duration::from_secs(1))? {
            if let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Enter => {
                        println!(); // Move to new line after Enter
                        
                        // Process command
                        let input = input_buffer.trim();
                        
                        if !input.is_empty() {
                            let parts: Vec<&str> = input.splitn(2, ' ').collect();
                            let command = parts[0].to_lowercase();

                            let mut command_processed = true;
                            match command.as_str() {
                                "add" => {
                                    if parts.len() < 2 || parts[1].is_empty() {
                                        println!("❌ Usage: add <description>");
                                        command_processed = false;
                                    } else {
                                        let description = parts[1].to_string();
                                        commands::add::add_todo(&mut todos, description)?;
                                    }
                                }
                                "remove" | "rm" => {
                                    if parts.len() < 2 {
                                        println!("❌ Usage: remove <id>");
                                        command_processed = false;
                                    } else {
                                        match parts[1].parse::<u32>() {
                                            Ok(id) => commands::remove::remove_todo(&mut todos, id)?,
                                            Err(_) => {
                                                println!("❌ Invalid ID. Please provide a number.");
                                                command_processed = false;
                                            }
                                        }
                                    }
                                }
                                "start" => {
                                    if parts.len() < 2 {
                                        println!("❌ Usage: start <id>");
                                        command_processed = false;
                                    } else {
                                        match parts[1].parse::<u32>() {
                                            Ok(id) => commands::start::start_todo(&mut todos, id)?,
                                            Err(_) => {
                                                println!("❌ Invalid ID. Please provide a number.");
                                                command_processed = false;
                                            }
                                        }
                                    }
                                }
                                "stop" => {
                                    if parts.len() < 2 {
                                        println!("❌ Usage: stop <id>");
                                        command_processed = false;
                                    } else {
                                        match parts[1].parse::<u32>() {
                                            Ok(id) => commands::stop::stop_todo(&mut todos, id)?,
                                            Err(_) => {
                                                println!("❌ Invalid ID. Please provide a number.");
                                                command_processed = false;
                                            }
                                        }
                                    }
                                }
                                "complete" | "done" => {
                                    if parts.len() < 2 {
                                        println!("❌ Usage: complete <id>");
                                        command_processed = false;
                                    } else {
                                        match parts[1].parse::<u32>() {
                                            Ok(id) => commands::complete::complete_todo(&mut todos, id)?,
                                            Err(_) => {
                                                println!("❌ Invalid ID. Please provide a number.");
                                                command_processed = false;
                                            }
                                        }
                                    }
                                }
                                "list" | "ls" => {
                                    // Just refresh display
                                }
                                "quit" | "exit" | "q" => {
                                    println!("\n👋 Goodbye! Stay productive!");
                                    return Ok(());
                                }
                                "help" | "h" => {
                                    println!("\nAvailable commands:");
                                    println!("  add <description>  - Add a new todo");
                                    println!("  start <id>         - Start a todo (mark as in progress)");
                                    println!("  stop <id>          - Stop a todo (remove timer, set to pending)");
                                    println!("  complete <id>      - Complete a todo (stop timer, mark done)");
                                    println!("  remove <id>        - Remove a todo by ID");
                                    println!("  list               - Show all todos");
                                    println!("  quit               - Exit the app");
                                    command_processed = false;
                                }
                                _ => {
                                    println!("❌ Unknown command: '{}'. Type 'help' for available commands.", command);
                                    command_processed = false;
                                }
                            }
                            
                            if command_processed {
                                // Redraw after a command is processed
                                print!("\x1B[2J\x1B[1;1H");
                                println!("Welcome to Pomonote - Your CLI Pomodoro Todo App!");
                                println!("Commands: add <desc> | start <id> | stop <id> | complete <id> | remove <id> | list | quit");
                                println!("{}", "=".repeat(70));
                                display_todos(&todos);
                            } else {
                                thread::sleep(Duration::from_millis(1000)); // Brief pause to show message
                            }
                        }
                        
                        input_buffer.clear();
                    }
                    KeyCode::Char(c) => {
                        input_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        input_buffer.pop();
                    }
                    KeyCode::Esc => {
                        input_buffer.clear();
                    }
                    _ => {}
                }
            }
        } else {
            // If no input, and timers are active, update display
            if needs_redraw {
                // This part is tricky without more granular control.
                // For now, the full redraw inside the loop handles it.
            }
        }
    }
}

// Placeholder function to get todos - replace with actual persistence later
fn get_todos() -> Vec<Todo> {
    vec![
        Todo {
            id: 1,
            description: "Complete Rust CLI project".to_string(),
            status: TodoStatus::Pending,
            timer: None,
        },
        Todo {
            id: 2,
            description: "Add data persistence".to_string(),
            status: TodoStatus::Pending,
            timer: None,
        },
        Todo {
            id: 3,
            description: "Write tests".to_string(),
            status: TodoStatus::Pending,
            timer: None,
        },
    ]
}
