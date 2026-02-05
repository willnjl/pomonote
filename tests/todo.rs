use pomonote::models::timer::Timer;
use pomonote::models::todo::{ Todo, TodoStatus };

#[cfg(test)]
mod todo_tests {
    use super::*;

    #[test]
    fn test_todo_new() {
        let todo = Todo::new(1, "Test task".to_string());
        assert_eq!(todo.id, 1);
        assert_eq!(todo.description, "Test task");
        assert!(matches!(todo.status, TodoStatus::Pending));
        assert!(todo.timer.is_none());
    }

    #[test]
    fn test_todo_status_display() {
        assert_eq!(TodoStatus::Pending.to_string(), "Pending");
        assert_eq!(TodoStatus::InProgress.to_string(), "In Progress");
        assert_eq!(TodoStatus::Completed.to_string(), "Completed");
    }

    #[test]
    fn test_next_id_empty_list() {
        let todos: Vec<Todo> = Vec::new();
        assert_eq!(Todo::next_id(&todos), 1);
    }

    #[test]
    fn test_next_id_with_todos() {
        let todos = vec![
            Todo::new(1, "First".to_string()),
            Todo::new(2, "Second".to_string()),
            Todo::new(5, "Third".to_string())
        ];
        assert_eq!(Todo::next_id(&todos), 6);
    }

    #[test]
    fn test_todo_with_timer() {
        let mut todo = Todo::new(1, "Test".to_string());
        let mut timer = Timer::new();
        timer.start();
        todo.timer = Some(timer);

        assert!(todo.timer.is_some());
        assert!(todo.timer.as_ref().unwrap().remaining_seconds() <= 1500);
    }

    #[test]
    fn test_todo_serialization() {
        let todo = Todo::new(1, "Test task".to_string());
        let json = serde_json::to_string(&todo).unwrap();
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"description\":\"Test task\""));
        assert!(json.contains("\"status\":\"Pending\""));
    }

    #[test]
    fn test_todo_deserialization() {
        let json = r#"{"id":1,"description":"Test","status":"InProgress","timer":null}"#;
        let todo: Todo = serde_json::from_str(json).unwrap();
        assert_eq!(todo.id, 1);
        assert_eq!(todo.description, "Test");
        assert!(matches!(todo.status, TodoStatus::InProgress));
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_todo_description() {
        let todo = Todo::new(1, "".to_string());
        assert_eq!(todo.description, "");
    }

    #[test]
    fn test_todo_with_special_characters() {
        let desc = "Test with 特殊字符 and émojis 🚀";
        let todo = Todo::new(1, desc.to_string());
        assert_eq!(todo.description, desc);
    }

    #[test]
    fn test_todo_with_multiline_description() {
        let desc = "Line 1\nLine 2\nLine 3";
        let todo = Todo::new(1, desc.to_string());
        assert_eq!(todo.description, desc);
    }

    #[test]
    fn test_very_long_todo_description() {
        let desc = "a".repeat(1000);
        let todo = Todo::new(1, desc.clone());
        assert_eq!(todo.description.len(), 1000);
    }

    #[test]
    fn test_large_todo_id() {
        let todo = Todo::new(u32::MAX, "Test".to_string());
        assert_eq!(todo.id, u32::MAX);
    }

    #[test]
    fn test_timer_immediately_after_start() {
        let mut timer = Timer::new();
        timer.start();
        // Check immediately - should still be close to 1500
        let remaining = timer.remaining_seconds();
        assert!(remaining >= 1499 && remaining <= 1500);
    }
}
