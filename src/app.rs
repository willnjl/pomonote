use std::io;
use crossterm::event::{ self, Event, KeyCode };
use ratatui::{ backend::Backend, Terminal };

use crate::display;
use crate::models::todo::{ Todo };
use crate::reduce::{ AppState, reduce, Action, parse_command };

// Clean event loop
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, initial_todos: Vec<Todo>) -> io::Result<()> {
    let mut state = AppState {
        todos: initial_todos,
        input_buffer: String::new(),
        output_buffer: String::new(),
        should_quit: false,
    };

    while !state.should_quit {
        terminal.draw(|f| {
            display::ui(f, &state.todos, &state.input_buffer, &state.output_buffer)
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let action = handle_key_event(key.code, &state.input_buffer);
                state = reduce(state, action);

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

pub fn handle_key_event(key: KeyCode, current_input: &str) -> Action {
    match key {
        KeyCode::Esc => Action::Quit,
        KeyCode::Enter => {
            let input = current_input.trim();
            if !input.is_empty() {
                let action = parse_command(input);
                action
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
