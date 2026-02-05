#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;
    use pomonote::models::todo::{ Todo, TodoStatus };
    use pomonote::reduce::{ reduce, Action, AppState };
    use pomonote::app::{ handle_key_event };

    fn initial_state() -> AppState {
        AppState {
            todos: vec![Todo::new(1, "First todo".to_string())],
            input_buffer: String::new(),
            output_buffer: String::new(),
            should_quit: false,
        }
    }

    #[test]
    fn test_add_todo() {
        let state = initial_state();
        let action = Action::AddTodo("New todo".to_string());
        let new_state = reduce(state, action);
        assert_eq!(new_state.todos.len(), 2);
        assert_eq!(new_state.todos[1].description, "New todo");
        assert_eq!(new_state.todos[1].id, 2);
    }

    #[test]
    fn test_remove_todo() {
        let state = initial_state();
        let action = Action::RemoveTodo(1);
        let new_state = reduce(state, action);
        assert!(new_state.todos.is_empty());
    }

    #[test]
    fn test_complete_todo() {
        let state = initial_state();
        let action = Action::CompleteTodo(1);
        let new_state = reduce(state, action);
        assert!(matches!(new_state.todos[0].status, TodoStatus::Completed));
    }

    #[test]
    fn test_start_todo() {
        let state = initial_state();
        let action = Action::StartTodo(1);
        let new_state = reduce(state, action);
        assert!(matches!(new_state.todos[0].status, TodoStatus::InProgress));
        assert!(new_state.todos[0].timer.is_some());
    }

    #[test]
    fn test_stop_todo() {
        let mut state = initial_state();
        let start_action = Action::StartTodo(1);
        let new_state = reduce(state, start_action);

        let stop_action = Action::StopTodo(1);
        let final_state = reduce(new_state, stop_action);
        assert!(matches!(final_state.todos[0].status, TodoStatus::Pending));
        assert!(final_state.todos[0].timer.is_none());
    }

    #[test]
    fn test_update_input() {
        let state = initial_state();
        let action = Action::UpdateInput("test".to_string());
        let new_state = reduce(state, action);
        assert_eq!(new_state.input_buffer, "test");
    }

    #[test]
    fn test_clear_input() {
        let mut state = initial_state();
        state.input_buffer = "test".to_string();
        let action = Action::ClearInput;
        let new_state = reduce(state, action);
        assert_eq!(new_state.input_buffer, "");
    }

    #[test]
    fn test_quit_action() {
        let state = initial_state();
        let action = Action::Quit;
        let new_state = reduce(state, action);
        assert!(new_state.should_quit);
    }

    #[test]
    fn test_handle_key_event_char() {
        let action = handle_key_event(KeyCode::Char('a'), "");
        assert!(matches!(action, Action::UpdateInput(s) if s == "a"));
    }

    #[test]
    fn test_handle_key_event_backspace() {
        let action = handle_key_event(KeyCode::Backspace, "abc");
        assert!(matches!(action, Action::UpdateInput(s) if s == "ab"));
    }

    #[test]
    fn test_handle_key_event_enter_add() {
        let action = handle_key_event(KeyCode::Enter, "add new task");
        assert!(matches!(action, Action::AddTodo(s) if s == "new task"));
    }

    #[test]
    fn test_handle_key_event_enter_quit() {
        let action = handle_key_event(KeyCode::Enter, "quit");
        assert!(matches!(action, Action::Quit));
    }

    #[test]
    fn test_handle_key_event_esc() {
        let action = handle_key_event(KeyCode::Esc, "");
        assert!(matches!(action, Action::Quit));
    }
}
