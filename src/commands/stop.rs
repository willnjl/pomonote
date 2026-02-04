use crate::models::todo::{Todo, TodoStatus};

/// Stop a todo task (remove timer and set to pending)
pub fn stop_todo(todos: &mut Vec<Todo>, id: u32) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.status = TodoStatus::Pending;
        todo.timer = None;
        
        println!("⏸️  Todo {} stopped. Timer removed.", id);
    } else {
        println!("❌ Todo with ID {} not found.", id);
    }
    
    Ok(())
}
