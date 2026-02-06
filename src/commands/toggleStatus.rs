use crate::reduce::AppState;
use crate::models::todo::TodoStatus;
use crate::models::timer::Timer;

pub fn run(state: &mut AppState, id: u32) -> () {
    if let Some(todo) = state.todos.iter_mut().find(|t| t.id == id) {
        match todo.status {
            TodoStatus::Pending => {
                todo.status = TodoStatus::InProgress;
                let mut timer = Timer::new();
                timer.start();
                let output = format!("⏱️  Timer started: {}\n", timer.output());
                state.output_buffer.push_str(&format!("{}🍅  Focus for 25 minutes!\n", &output));
                todo.timer = Some(timer);
            }
            TodoStatus::Completed => {
                todo.status = TodoStatus::Pending;
                state.output_buffer.push_str("✅ Todo moved to pending \n");
            }
            TodoStatus::InProgress => {
                todo.timer = None;
                todo.status = TodoStatus::Completed;
                let output = format!("✅ Todo {} completed! Great work!\n", id);
                state.output_buffer.push_str(&format!("{}🍅  Focus for 25 minutes! \n", &output));
            }
        };
    }
}
