use crate::models::todo::Todo;

/// Remove a todo task by ID
pub fn remove_todo(todos: &mut Vec<Todo>, id: u32) -> Result<String, String> {
    if let Some(pos) = todos.iter().position(|t| t.id == id) {
        todos.remove(pos);
        Ok(format!("✅ Todo {} removed successfully!", id))
    } else {
        Err(format!("❌ Todo with ID {} not found.", id))
    }
}
