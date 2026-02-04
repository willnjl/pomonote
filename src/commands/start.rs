use crate::models::todo::{ Todo, TodoStatus };
use crate::models::timer::Timer;

/// Start a todo task (mark as in progress) and start a 25-minute timer
pub fn start_todo(todos: &mut Vec<Todo>, id: u32) -> Result<String, String> {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.status = TodoStatus::InProgress;

        // Create and start a new timer
        let mut timer = Timer::new();
        timer.start();

        let output = format!("{}⏱️  Timer started: {}\n", output, timer.output());
        let output = format!("{}🍅  Focus for 25 minutes!", output);

        todo.timer = Some(timer);
        Ok(output)
    } else {
        Err(format!("❌ Todo with ID {} not found.", id))
    }
}
