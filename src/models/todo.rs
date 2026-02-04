use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub description: String,
    pub status: TodoStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

impl std::fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoStatus::Pending => write!(f, "Pending"),
            TodoStatus::InProgress => write!(f, "In Progress"),
            TodoStatus::Completed => write!(f, "Completed"),
        }
    }
}

impl Todo {
    pub fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            status: TodoStatus::Pending,
        }
    }
}
