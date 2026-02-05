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
pub enum Action {
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
pub fn reduce(mut state: AppState, action: Action) -> AppState {
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

pub fn parse_command(input: &str) -> Action {
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
