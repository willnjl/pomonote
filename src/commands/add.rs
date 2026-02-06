use crate::reduce::AppState;
use crate::models::todo::Todo;

pub fn run(state: &mut AppState, text: &str) -> () {
    let next_id =
        state.todos
            .iter()
            .map(|t| t.id)
            .max()
            .unwrap_or(0) + 1;
    let new_todo = Todo::new(next_id, text.to_string());
    state.todos.push(new_todo);
    state.output_buffer = format!("✅ Todo added successfully! (ID: {})", next_id);
}
