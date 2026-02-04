use crate::models::todo::{ Todo, TodoStatus };

/// Stop a todo task (remove timer and set to pending)
pub fn stop_todo(todos: &mut Vec<Todo>, id: u32) -> Result<String, String> {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.status = TodoStatus::Pending;
        todo.timer = None;

        Ok(format!("⏸️  Todo {} stopped. Timer removed.", id))
    } else {
        Err(format!("❌ Todo with ID {} not found.", id))
    }
}
