use crate::reduce::AppState;
use crate::models::todo::TodoStatus;
use crate::models::timer::Timer;

pub fn run(state: &mut AppState, id: u32) -> () {
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
