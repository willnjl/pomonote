use crate::models::todo::{ self, Todo, TodoStatus };
use crate::commands;
use crate::utils::OneOrMany;

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
    ToggleStatus(u32),
    UpdateInput(String),
    ClearInput,
    SetOutput(String),
    NoOp,
}

pub fn reduce(state: &mut AppState, action: Action) -> () {
    match action {
        Action::Quit => {
            state.should_quit = true;
        }
        Action::AddTodo(text) => commands::add::run(state, &text),
        Action::RemoveTodo(id) => commands::remove::run(state, id),
        Action::CompleteTodo(id) => commands::complete::run(state, id),
        Action::StartTodo(id) => commands::start::run(state, id),
        Action::StopTodo(id) => commands::stop::run(state, id),
        Action::ToggleStatus(id) => commands::toggleStatus::run(state, id),
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
}

pub fn parse_command(input: &str) -> OneOrMany<Action> {
    let tokens = input.split_whitespace();
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
        return OneOrMany::Many(
            int_args
                .iter()
                .map(|&int| Action::ToggleStatus(int))
                .collect()
        );
    }

    let actions = match command.as_str() {
        "quit" | "exit" | "q" => OneOrMany::One(Action::Quit),
        "add" => {
            if !str_args.is_empty() || !int_args.is_empty() {
                let mut all_args = Vec::new();
                for i in int_args.iter() {
                    all_args.push(i.to_string());
                }
                for s in str_args.iter() {
                    all_args.push(s.to_string());
                }
                OneOrMany::One(Action::AddTodo(all_args.join(" ")))
            } else {
                OneOrMany::One(Action::SetOutput("Usage: add <todo text>".to_string()))
            }
        }
        "remove" | "rm" => {
            if int_args.is_empty() {
                OneOrMany::One(Action::SetOutput("Usage: remove <id> [<id> ...]".to_string()))
            } else {
                OneOrMany::Many(int_args.into_iter().map(Action::RemoveTodo).collect())
            }
        }
        "complete" | "done" => {
            if int_args.is_empty() {
                OneOrMany::One(Action::SetOutput("Usage: complete <id> [<id> ...]".to_string()))
            } else {
                OneOrMany::Many(int_args.into_iter().map(Action::CompleteTodo).collect())
            }
        }
        "start" => {
            if int_args.is_empty() {
                OneOrMany::One(Action::SetOutput("Usage: start <id> [<id> ...]".to_string()))
            } else {
                OneOrMany::Many(int_args.into_iter().map(Action::StartTodo).collect())
            }
        }
        "stop" => {
            if int_args.is_empty() {
                OneOrMany::One(Action::SetOutput("Usage: stop <id> [<id> ...]".to_string()))
            } else {
                OneOrMany::Many(int_args.into_iter().map(Action::StopTodo).collect())
            }
        }
        _ => OneOrMany::One(Action::SetOutput("Invalid command".to_string())),
    };

    actions
}
