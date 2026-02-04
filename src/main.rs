mod commands;
mod display;
mod models;
mod table;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use models::todo::{Todo, TodoStatus};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut todos = get_todos();
    let mut input_buffer = String::new();
    let res = run_app(&mut terminal, &mut todos, &mut input_buffer);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    todos: &mut Vec<Todo>,
    input_buffer: &mut String,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| display::ui(f, todos, input_buffer))?;

        if let Event::Key(key) = event::read()? {
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
                                    if let Err(e) = commands::add::add_todo(todos, description) {
                                        // TODO: display error in UI
                                    }
                                }
                            }
                            "remove" | "rm" => {
                                if parts.len() > 1 {
                                    if let Ok(id) = parts[1].parse::<u32>() {
                                        if let Err(e) = commands::remove::remove_todo(todos, id) {
                                            // TODO: display error in UI
                                        }
                                    }
                                }
                            }
                            "start" => {
                                if parts.len() > 1 {
                                    if let Ok(id) = parts[1].parse::<u32>() {
                                        if let Err(e) = commands::start::start_todo(todos, id) {
                                            // TODO: display error in UI
                                        }
                                    }
                                }
                            }
                            "stop" => {
                                if parts.len() > 1 {
                                    if let Ok(id) = parts[1].parse::<u32>() {
                                        if let Err(e) = commands::stop::stop_todo(todos, id) {
                                            // TODO: display error in UI
                                        }
                                    }
                                }
                            }
                            "complete" => {
                                if parts.len() > 1 {
                                    if let Ok(id) = parts[1].parse::<u32>() {
                                        if let Err(e) = commands::complete::complete_todo(todos, id)
                                        {
                                            // TODO: display error in UI
                                        }
                                    }
                                }
                            }
                            "quit" => return Ok(()),
                            _ => {
                                // TODO: display invalid command in UI
                            }
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
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}

// Placeholder for loading todos
fn get_todos() -> Vec<Todo> {
    vec![
        Todo::new(1, "Write a blog post".to_string()),
        Todo::new(2, "Learn Rust".to_string()),
        Todo::new(3, "Go for a run".to_string()),
    ]
}
