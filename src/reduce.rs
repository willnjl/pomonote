use std::io;
use crossterm::event::{ self, Event, KeyCode };
use ratatui::{ backend::Backend, Terminal };

use crate::display;
use crate::models::todo::{ Todo, TodoStatus };
use crate::models::timer::Timer;

// Centralized application state
pub struct AppState {
    pub todos: Vec<Todo>,
    pub input_buffer: String,
    pub output_buffer: String,
    pub should_quit: bool,
}

// All possible actions
enum Action {
    Quit,
    AddTodo(String),
    RemoveTodo(u32),
    CompleteTodo(u32),
    StartTodo(u32),
    StopTodo(u32),
    UpdateInput(String),
    ClearInput,
    SetOutput(String),
    NoOp,
}

// Reducer: pure function that takes state + action, returns new state
fn reduce(mut state: AppState, action: Action) -> AppState {
    match action {
        Action::Quit => {
            state.should_quit = true;
        }
        Action::AddTodo(text) => {
            let next_id =
                state.todos
                    .iter()
                    .map(|t| t.id)
                    .max()
                    .unwrap_or(0) + 1;
            let new_todo = Todo::new(next_id, text.clone());
            state.todos.push(new_todo);
            state.output_buffer = format!("✅ Todo added successfully! (ID: {})", next_id);
        }
        Action::RemoveTodo(id) => {
            if let Some(pos) = state.todos.iter().position(|t| t.id == id) {
                state.todos.remove(pos);
                state.output_buffer = format!("✅ Todo {} removed successfully!", id);
            } else {
                state.output_buffer = format!("❌ Todo with ID {} not found.", id);
            }
        }
        Action::CompleteTodo(id) => {
            if let Some(todo) = state.todos.iter_mut().find(|t| t.id == id) {
                todo.status = TodoStatus::Completed;
                todo.timer = None;
                state.output_buffer = format!("✅ Todo {} completed! Great work!", id);
            } else {
                state.output_buffer = format!("❌ Todo with ID {} not found.", id);
            }
        }
        Action::StartTodo(id) => {
            if let Some(todo) = state.todos.iter_mut().find(|t| t.id == id) {
                todo.status = TodoStatus::InProgress;
                let mut timer = Timer::new();
                timer.start();
                let output = format!("⏱️  Timer started: {}\n", timer.output());
                state.output_buffer = format!("{}🍅  Focus for 25 minutes!", output);
                todo.timer = Some(timer);
            } else {
                state.output_buffer = format!("❌ Todo with ID {} not found.", id);
            }
        }
        Action::StopTodo(id) => {
            if let Some(todo) = state.todos.iter_mut().find(|t| t.id == id) {
                todo.status = TodoStatus::Pending;
                todo.timer = None;
                state.output_buffer = format!("⏸️  Todo {} stopped. Timer removed.", id);
            } else {
                state.output_buffer = format!("❌ Todo with ID {} not found.", id);
            }
        }
        Action::UpdateInput(s) => {
            state.input_buffer = s;
        }
        Action::ClearInput => {
            state.input_buffer.clear();
        }
        Action::SetOutput(s) => {
            state.output_buffer = s;
        }
        Action::NoOp => {}
    }
    state
}

// Parse command string into an Action
fn parse_command(input: &str) -> Action {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let command = parts[0].to_lowercase();

    match command.as_str() {
        "quit" | "exit" | "q" => Action::Quit,
        "add" => {
            if let Some(text) = parts.get(1) {
                Action::AddTodo(text.to_string())
            } else {
                Action::SetOutput("Usage: add <todo text>".to_string())
            }
        }
        "remove" | "rm" => {
            if let Some(id_str) = parts.get(1) {
                if let Ok(id) = id_str.parse::<u32>() {
                    Action::RemoveTodo(id)
                } else {
                    Action::SetOutput("Invalid ID".to_string())
                }
            } else {
                Action::SetOutput("Usage: remove <id>".to_string())
            }
        }
        "complete" | "done" => {
            if let Some(id_str) = parts.get(1) {
                if let Ok(id) = id_str.parse::<u32>() {
                    Action::CompleteTodo(id)
                } else {
                    Action::SetOutput("Invalid ID".to_string())
                }
            } else {
                Action::SetOutput("Usage: complete <id>".to_string())
            }
        }
        "start" => {
            if let Some(id_str) = parts.get(1) {
                if let Ok(id) = id_str.parse::<u32>() {
                    Action::StartTodo(id)
                } else {
                    Action::SetOutput("Invalid ID".to_string())
                }
            } else {
                Action::SetOutput("Usage: start <id>".to_string())
            }
        }
        "stop" => {
            if let Some(id_str) = parts.get(1) {
                if let Ok(id) = id_str.parse::<u32>() {
                    Action::StopTodo(id)
                } else {
                    Action::SetOutput("Invalid ID".to_string())
                }
            } else {
                Action::SetOutput("Usage: stop <id>".to_string())
            }
        }
        _ => Action::SetOutput("Invalid command".to_string()),
    }
}

// Handle keyboard events and return an Action
fn handle_key_event(key: KeyCode, current_input: &str) -> Action {
    match key {
        KeyCode::Esc => Action::Quit,
        KeyCode::Enter => {
            let input = current_input.trim();
            if !input.is_empty() {
                parse_command(input)
            } else {
                Action::NoOp
            }
        }
        KeyCode::Char(c) => {
            let mut new_input = current_input.to_string();
            new_input.push(c);
            Action::UpdateInput(new_input)
        }
        KeyCode::Backspace => {
            let mut new_input = current_input.to_string();
            new_input.pop();
            Action::UpdateInput(new_input)
        }
        _ => Action::NoOp,
    }
}

// Clean event loop
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, initial_todos: Vec<Todo>) -> io::Result<()> {
    let mut state = AppState {
        todos: initial_todos,
        input_buffer: String::new(),
        output_buffer: String::new(),
        should_quit: false,
    };

    while !state.should_quit {
        // Render
        terminal.draw(|f| {
            display::ui(f, &state.todos, &state.input_buffer, &state.output_buffer)
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let action = handle_key_event(key.code, &state.input_buffer);
                state = reduce(state, action);

                // Clear input after Enter
                if matches!(key.code, KeyCode::Enter) {
                    state.input_buffer.clear();
                }
            }
        }
    }
    // save on quit
    if let Err(e) = Todo::save_all(&state.todos) {
        eprintln!("Failed to save todos: {}", e);
    }

    Ok(())
}
