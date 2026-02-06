use pomonote::reduce::{ reduce, parse_command, Action, AppState };
use pomonote::models::todo::{ Todo, TodoStatus };
use pomonote::utils::OneOrMany;

fn get_initial_state() -> AppState {
    AppState {
        todos: vec![
            Todo::new(1, "todo 1".to_string()),
            Todo::new(2, "todo 2".to_string()),
            Todo::new(3, "todo 3".to_string())
        ],
        input_buffer: String::new(),
        output_buffer: String::new(),
        should_quit: false,
    }
}

#[test]
fn test_reduce_quit() {
    let mut state = get_initial_state();
    reduce(&mut state, Action::Quit);
    assert!(state.should_quit);
}

#[test]
fn test_reduce_add_todo() {
    let mut state = get_initial_state();
    let todo_text = "new todo";
    reduce(&mut state, Action::AddTodo(todo_text.to_string()));
    assert_eq!(state.todos.len(), 4);
    assert_eq!(state.todos.last().unwrap().description, todo_text);
}

#[test]
fn test_reduce_remove_todo() {
    let mut state = get_initial_state();
    reduce(&mut state, Action::RemoveTodo(2));
    assert_eq!(state.todos.len(), 2);
    assert!(!state.todos.iter().any(|t| t.id == 2));
}

#[test]
fn test_reduce_complete_todo() {
    let mut state = get_initial_state();
    reduce(&mut state, Action::CompleteTodo(1));
    assert_eq!(state.todos[0].status, TodoStatus::Completed);
}

#[test]
fn test_reduce_start_todo() {
    let mut state = get_initial_state();
    reduce(&mut state, Action::StartTodo(1));
    assert_eq!(state.todos[0].status, TodoStatus::InProgress);
}

#[test]
fn test_reduce_stop_todo() {
    let mut state = get_initial_state();
    state.todos[0].status = TodoStatus::InProgress;
    reduce(&mut state, Action::StopTodo(1));
    assert_eq!(state.todos[0].status, TodoStatus::Pending);
}

#[test]
fn test_reduce_toggle_status() {
    let mut state = get_initial_state();
    reduce(&mut state, Action::ToggleStatus(1));
    assert_eq!(state.todos[0].status, TodoStatus::InProgress);
    reduce(&mut state, Action::ToggleStatus(1));
    assert_eq!(state.todos[0].status, TodoStatus::Completed);
}

#[test]
fn test_reduce_update_input() {
    let mut state = get_initial_state();
    let input = "hello";
    reduce(&mut state, Action::UpdateInput(input.to_string()));
    assert_eq!(state.input_buffer, input);
}

#[test]
fn test_reduce_clear_input() {
    let mut state = get_initial_state();
    state.input_buffer = "some text".to_string();
    reduce(&mut state, Action::ClearInput);
    assert!(state.input_buffer.is_empty());
}

#[test]
fn test_reduce_set_output() {
    let mut state = get_initial_state();
    let output = "An error occurred";
    reduce(&mut state, Action::SetOutput(output.to_string()));
    assert_eq!(state.output_buffer, output);
}

#[test]
fn test_parse_command_quit() {
    let action = parse_command("quit");
    assert!(matches!(action, OneOrMany::One(Action::Quit)));
}

#[test]
fn test_parse_command_add() {
    let action = parse_command("add new todo item");
    assert!(matches!(action, OneOrMany::One(Action::AddTodo(s)) if s == "new todo item"));
}

#[test]
fn test_parse_command_remove_multiple() {
    let actions = parse_command("rm 1 3");
    if let OneOrMany::Many(actions) = actions {
        assert_eq!(actions.len(), 2);
        assert!(matches!(actions[0], Action::RemoveTodo(1)));
        assert!(matches!(actions[1], Action::RemoveTodo(3)));
    } else {
        panic!("Expected multiple actions");
    }
}

#[test]
fn test_parse_command_toggle_status_implicit() {
    let actions = parse_command("1 2");
    if let OneOrMany::Many(actions) = actions {
        assert_eq!(actions.len(), 2);
        assert!(matches!(actions[0], Action::ToggleStatus(1)));
        assert!(matches!(actions[1], Action::ToggleStatus(2)));
    } else {
        panic!("Expected multiple actions");
    }
}

#[test]
fn test_parse_command_invalid() {
    let action = parse_command("invalid command");
    assert!(matches!(action, OneOrMany::One(Action::SetOutput(s)) if s == "Invalid command"));
}
