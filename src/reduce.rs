use std::process::Output;

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
    RemoveTodos(Vec<u32>),
    CompleteTodo(u32),
    StartTodo(u32),
    StopTodo(u32),
    UpdateInput(String),
    ClearInput,
    SetOutput(String),
    ToggleStatus(Vec<u32>),
    NoOp,
}

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
        Action::RemoveTodos(ids) => {
            state.output_buffer.clear();
            for id in ids {
                if let Some(pos) = state.todos.iter().position(|t| t.id == id) {
                    state.todos.remove(pos);
                    state.output_buffer = format!(
                        "{} ✅ Todo {} removed successfully! \n",
                        state.output_buffer,
                        id
                    );
                } else {
                    state.output_buffer = format!("❌ Todo with ID {} not found.", id);
                }
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
        Action::ToggleStatus(ids) => {
            let mut tmp_output_buffer = String::new();
            ids.iter().for_each(|&id| {
                if let Some(todo) = state.todos.iter_mut().find(|t| t.id == id) {
                    match todo.status {
                        TodoStatus::Pending => {
                            todo.status = TodoStatus::InProgress;
                            let mut timer = Timer::new();
                            timer.start();
                            let output = format!("⏱️  Timer started: {}\n", timer.output());
                            tmp_output_buffer.push_str(
                                &format!("{}🍅  Focus for 25 minutes!\n", &output)
                            );
                            todo.timer = Some(timer);
                        }
                        TodoStatus::Completed => {
                            todo.status = TodoStatus::Pending;
                            tmp_output_buffer.push_str("✅ Todo moved to pending \n");
                        }
                        TodoStatus::InProgress => {
                            todo.timer = None;
                            todo.status = TodoStatus::Completed;
                            let output = format!("✅ Todo {} completed! Great work!\n", id);
                            tmp_output_buffer.push_str(
                                &format!("{}🍅  Focus for 25 minutes! \n", &output)
                            );
                        }
                    };
                }

                state.output_buffer = tmp_output_buffer.clone();
            });
        }
    }
    state
}

pub fn parse_command(input: &str) -> Action {
    // Split input into command, integer arguments, and string arguments
    let mut tokens = input.split_whitespace();
    let mut int_args = Vec::new();
    let mut str_args = Vec::new();

    for token in tokens {
        if let Ok(num) = token.parse::<u32>() {
            int_args.push(num);
        } else {
            str_args.push(token);
        }
    }

    let command = if !str_args.is_empty() { str_args.remove(0).to_string() } else { String::new() };

    if command.is_empty() && !int_args.is_empty() {
        return Action::ToggleStatus(int_args);
    }

    match command.as_str() {
        "quit" | "exit" | "q" => Action::Quit,
        "add" => {
            if !str_args.is_empty() || !int_args.is_empty() {
                // Join all remaining tokens (ints and strings) as the todo text
                let mut all_args = Vec::new();
                for i in int_args.iter() {
                    all_args.push(i.to_string());
                }
                for s in str_args.iter() {
                    all_args.push(s.to_string());
                }
                Action::AddTodo(all_args.join(" "))
            } else {
                Action::SetOutput("Usage: add <todo text>".to_string())
            }
        }
        "remove" | "rm" => {
            if int_args.is_empty() {
                Action::SetOutput("Usage: remove <id> [<id> ...]".to_string())
            } else {
                if int_args.len() == 1 {
                    Action::RemoveTodo(int_args[0])
                } else {
                    Action::RemoveTodos(int_args)
                }
            }
        }
        "complete" | "done" | "start" | "stop" => {
            if let Some(id) = int_args.get(0) {
                match command.as_str() {
                    "complete" | "done" => Action::CompleteTodo(*id),
                    "start" => Action::StartTodo(*id),
                    "stop" => Action::StopTodo(*id),
                    _ => unreachable!(), // Should not happen due to outer match
                }
            } else {
                Action::SetOutput(format!("Usage: {} <id>", command))
            }
        }
        _ => Action::SetOutput("Invalid command".to_string()),
    }
}
