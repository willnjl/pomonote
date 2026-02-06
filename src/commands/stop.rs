use crate::reduce::AppState;
use crate::models::todo::TodoStatus;

pub fn run(state: &mut AppState, id: u32) -> () {
    if let Some(todo) = state.todos.iter_mut().find(|t| t.id == id) {
        todo.status = TodoStatus::Pending;
        todo.timer = None;
        state.output_buffer = format!("⏸️  Todo {} stopped. Timer removed.", id);
    } else {
        state.output_buffer = format!("❌ Todo with ID {} not found.", id);
    }
}
