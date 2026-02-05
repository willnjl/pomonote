use pomonote::models::timer::Timer;
use pomonote::models::todo::{ Todo, TodoStatus };
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_todo_lifecycle() {
        // Create a new todo
        let mut todo = Todo::new(1, "Write tests".to_string());
        assert!(matches!(todo.status, TodoStatus::Pending));

        // Start working on it
        todo.status = TodoStatus::InProgress;
        let mut timer = Timer::new();
        timer.start();
        todo.timer = Some(timer);
        assert!(matches!(todo.status, TodoStatus::InProgress));
        assert!(todo.timer.is_some());

        // Complete it
        todo.status = TodoStatus::Completed;
        todo.timer = None;
        assert!(matches!(todo.status, TodoStatus::Completed));
        assert!(todo.timer.is_none());
    }

    #[test]
    fn test_multiple_todos_workflow() {
        let mut todos = vec![
            Todo::new(1, "Task 1".to_string()),
            Todo::new(2, "Task 2".to_string()),
            Todo::new(3, "Task 3".to_string())
        ];

        // Start task 2
        todos[1].status = TodoStatus::InProgress;
        let mut timer = Timer::new();
        timer.start();
        todos[1].timer = Some(timer);

        // Verify states
        assert!(matches!(todos[0].status, TodoStatus::Pending));
        assert!(matches!(todos[1].status, TodoStatus::InProgress));
        assert!(matches!(todos[2].status, TodoStatus::Pending));

        assert!(todos[0].timer.is_none());
        assert!(todos[1].timer.is_some());
        assert!(todos[2].timer.is_none());

        // Complete task 2
        todos[1].status = TodoStatus::Completed;
        todos[1].timer = None;

        assert!(matches!(todos[1].status, TodoStatus::Completed));
        assert!(todos[1].timer.is_none());
    }

    #[test]
    fn test_timer_output_across_lifecycle() {
        let mut timer = Timer::new();

        // Before start
        assert_eq!(timer.output(), "25:00");

        // After start
        timer.start();
        thread::sleep(Duration::from_secs(1));
        let output = timer.output();
        assert!(output.starts_with("24:"));

        // Check not finished
        assert!(!timer.is_finished());
    }
}
