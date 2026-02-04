use serde::{ Deserialize, Serialize };
use super::timer::Timer;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub description: String,
    pub status: TodoStatus,
    pub timer: Option<Timer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

impl std::fmt::Display for TodoStatus {
    fn fmt(&self, __f__: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoStatus::Pending => write!(__f__, "Pending"),
            TodoStatus::InProgress => write!(__f__, "In Progress"),
            TodoStatus::Completed => write!(__f__, "Completed"),
        }
    }
}

impl Todo {
    pub fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            status: TodoStatus::Pending,
            timer: None,
        }
    }

    pub fn save_all(todos: &Vec<Todo>) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(todos)?;
        fs::write("todos.json", json)?;
        Ok(())
    }

    pub fn load_all() -> Vec<Todo> {
        let path = Path::new("todos.json");

        if !path.exists() {
            return Vec::new();
        }

        let mut todos: Vec<Todo> = fs
            ::read_to_string(path)
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_else(Vec::new);

        for todo in &mut todos {
            if let Some(timer) = &mut todo.timer {
                timer.restore_instant();
            }
        }

        todos
    }

    pub fn next_id(todos: &Vec<Todo>) -> u32 {
        todos
            .iter()
            .map(|t| t.id)
            .max()
            .unwrap_or(0) + 1
    }
}
