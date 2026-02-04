mod commands;
mod display;
mod models;

use crossterm::{
    event::{ self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
};
use models::todo::{ Todo, TodoStatus };
use ratatui::{ backend::{ Backend, CrosstermBackend }, Terminal };
use std::{ error::Error, io };

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut todos = get_todos();
    let mut input_buffer = String::new();
    let res = run_app(&mut terminal, &mut todos, &mut input_buffer);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
use std::time::Duration;
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    todos: &mut Vec<Todo>,
    input_buffer: &mut String
) -> io::Result<()> {
    let mut output_buffer = String::new();
    loop {
        terminal.draw(|f| display::ui(f, todos, input_buffer, &output_buffer))?;

        if event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                output_buffer.clear();
                let mut should_save = false; // Track if we need to save

                match key.code {
                    KeyCode::Enter => {
                        let input = input_buffer.trim();
                        if !input.is_empty() {
                            let parts: Vec<&str> = input.splitn(2, ' ').collect();
                            let command = parts[0].to_lowercase();

                            match command.as_str() {
                                "add" => {
                                    if parts.len() > 1 && !parts[1].is_empty() {
                                        let description = parts[1].to_string();
                                        match commands::add::add_todo(todos, description) {
                                            Ok(msg) => {
                                                output_buffer = msg;
                                                should_save = true;
                                            }
                                            Err(e) => {
                                                output_buffer = e;
                                            }
                                        }
                                    } else {
                                        output_buffer = "Usage: add <description>".to_string();
                                    }
                                }
                                "remove" | "rm" => {
                                    if parts.len() > 1 {
                                        if let Ok(id) = parts[1].parse::<u32>() {
                                            match commands::remove::remove_todo(todos, id) {
                                                Ok(msg) => {
                                                    output_buffer = msg;
                                                    should_save = true;
                                                }
                                                Err(e) => {
                                                    output_buffer = e;
                                                }
                                            }
                                        } else {
                                            output_buffer = "Invalid ID".to_string();
                                        }
                                    } else {
                                        output_buffer = "Usage: remove <id>".to_string();
                                    }
                                }
                                "start" => {
                                    if parts.len() > 1 {
                                        if let Ok(id) = parts[1].parse::<u32>() {
                                            match commands::start::start_todo(todos, id) {
                                                Ok(msg) => {
                                                    output_buffer = msg;
                                                    should_save = true;
                                                }
                                                Err(e) => {
                                                    output_buffer = e;
                                                }
                                            }
                                        } else {
                                            output_buffer = "Invalid ID".to_string();
                                        }
                                    } else {
                                        output_buffer = "Usage: start <id>".to_string();
                                    }
                                }
                                "stop" => {
                                    if parts.len() > 1 {
                                        if let Ok(id) = parts[1].parse::<u32>() {
                                            match commands::stop::stop_todo(todos, id) {
                                                Ok(msg) => {
                                                    output_buffer = msg;
                                                    should_save = true;
                                                }
                                                Err(e) => {
                                                    output_buffer = e;
                                                }
                                            }
                                        } else {
                                            output_buffer = "Invalid ID".to_string();
                                        }
                                    } else {
                                        output_buffer = "Usage: stop <id>".to_string();
                                    }
                                }
                                "complete" => {
                                    if parts.len() > 1 {
                                        if let Ok(id) = parts[1].parse::<u32>() {
                                            match commands::complete::complete_todo(todos, id) {
                                                Ok(msg) => {
                                                    output_buffer = msg;
                                                    should_save = true;
                                                }
                                                Err(e) => {
                                                    output_buffer = e;
                                                }
                                            }
                                        } else {
                                            output_buffer = "Invalid ID".to_string();
                                        }
                                    } else {
                                        output_buffer = "Usage: complete <id>".to_string();
                                    }
                                }
                                "quit" => {
                                    return Ok(());
                                }
                                _ => {
                                    output_buffer = "Invalid command".to_string();
                                }
                            }
                        }
                        input_buffer.clear();

                        // Save to disk if any command modified the todos
                        if should_save {
                            Todo::save_all(todos).ok();
                        }
                    }
                    KeyCode::Char(c) => {
                        input_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        input_buffer.pop();
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
}

// Placeholder for loading todos
// fn get_todos() -> Vec<Todo> {
//     vec![
//         Todo::new(1, "Write a blog post".to_string()),
//         Todo::new(2, "Learn Rust".to_string()),
//         Todo::new(3, "Go for a run".to_string())
//     ]
// }

fn get_todos() -> Vec<Todo> {
    Todo::load_all()
}
