use crate::models::todo::{Todo, TodoStatus};

/// Start a todo task (mark as in progress)
pub fn start_todo(todos: &mut Vec<Todo>, id: u32) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.status = TodoStatus::InProgress;
        println!("✅ Todo {} started successfully!", id);
    } else {
        println!("❌ Todo with ID {} not found.", id);
    }
    
    Ok(())
}
