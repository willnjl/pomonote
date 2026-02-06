use crate::reduce::AppState;

pub fn run(state: &mut AppState, id: u32) -> () {
    if let Some(pos) = state.todos.iter().position(|t| t.id == id) {
        state.todos.remove(pos);
        state.output_buffer = format!("✅ Todo {} removed successfully!", id);
    } else {
        state.output_buffer = format!("❌ Todo with ID {} not found.", id);
    }
}
