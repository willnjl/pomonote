use std::io;
use crossterm::event::{ self, Event, KeyCode };
use ratatui::{ backend::Backend, Terminal };

use crate::display;
use crate::models::todo::{ Todo };
use crate::reduce::{ AppState, reduce, Action, parse_command };
use crate::utils::OneOrMany;

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
                let actions = handle_key_event(key.code, &state.input_buffer);

                match actions {
                    OneOrMany::Many(actions) => {
                        actions.into_iter().for_each(|a| reduce(&mut state, a));
                    }
                    OneOrMany::One(a) => {
                        reduce(&mut state, a);
                    }
                }

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

pub fn handle_key_event(key: KeyCode, current_input: &str) -> OneOrMany<Action> {
    match key {
        KeyCode::Esc => OneOrMany::One(Action::Quit),
        KeyCode::Enter => {
            let input = current_input.trim();
            if !input.is_empty() {
                let actions = parse_command(input);
                actions
            } else {
                OneOrMany::One(Action::NoOp)
            }
        }
        KeyCode::Char(c) => {
            let mut new_input = current_input.to_string();
            new_input.push(c);
            OneOrMany::One(Action::UpdateInput(new_input))
        }
        KeyCode::Backspace => {
            let mut new_input = current_input.to_string();
            new_input.pop();
            OneOrMany::One(Action::UpdateInput(new_input))
        }
        _ => OneOrMany::One(Action::NoOp),
    }
}
