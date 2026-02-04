use crate::models::todo::{Todo, TodoStatus};
use crate::models::timer::Timer;

/// Start a todo task (mark as in progress) and start a 25-minute timer
pub fn start_todo(todos: &mut Vec<Todo>, id: u32) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.status = TodoStatus::InProgress;
        
        // Create and start a new timer
        let mut timer = Timer::new();
        timer.start();
        
        println!("✅ Todo {} started successfully!", id);
        println!("⏱️  Timer started: {}", timer.output());
        println!("   Focus for 25 minutes!");
        
        todo.timer = Some(timer);
    } else {
        println!("❌ Todo with ID {} not found.", id);
    }
    
    Ok(())
}
