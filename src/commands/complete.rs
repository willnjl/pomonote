use crate::models::todo::{Todo, TodoStatus};

/// Complete a todo task (stop timer and mark as completed)
pub fn complete_todo(todos: &mut Vec<Todo>, id: u32) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.status = TodoStatus::Completed;
        todo.timer = None;
        
        println!("✅ Todo {} completed! Great work!", id);
    } else {
        println!("❌ Todo with ID {} not found.", id);
    }
    
    Ok(())
}
