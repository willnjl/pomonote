use crate::models::todo::Todo;

/// Remove a todo task by ID
pub fn remove_todo(todos: &mut Vec<Todo>, id: u32) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(pos) = todos.iter().position(|t| t.id == id) {
        todos.remove(pos);
        println!("✅ Todo {} removed successfully!", id);
    } else {
        println!("❌ Todo with ID {} not found.", id);
    }
    
    Ok(())
}
