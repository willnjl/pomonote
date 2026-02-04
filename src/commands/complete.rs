use crate::models::todo::{ Todo, TodoStatus };

/// Complete a todo task (stop timer and mark as completed)
pub fn complete_todo(todos: &mut Vec<Todo>, id: u32) -> Result<String, String> {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.status = TodoStatus::Completed;
        todo.timer = None;

        Ok(format!("✅ Todo {} completed! Great work!", id))
    } else {
        Err(format!("❌ Todo with ID {} not found.", id))
    }
}
