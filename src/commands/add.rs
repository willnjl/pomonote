use crate::models::todo::Todo;

/// Add a new todo task
pub fn add_todo(todos: &mut Vec<Todo>, description: String) -> Result<(), Box<dyn std::error::Error>> {
    // Find the next available ID
    let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    
    let new_todo = Todo::new(next_id, description.clone());
    todos.push(new_todo);
    
    println!("✅ Todo added successfully! (ID: {})", next_id);
    
    Ok(())
}
