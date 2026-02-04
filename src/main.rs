mod commands;
mod display;
mod models;
use crossterm::{
    event::{ self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
};
use models::todo::{ Todo, TodoStatus };
use ratatui::{ backend::{ Backend, CrosstermBackend }, Terminal };
use std::{ error::Error, io, panic };

fn main() -> Result<(), Box<dyn Error>> {
    // Setup better panic handler
    setup_panic_handler();
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut todos = get_todos();
    let mut input_buffer = String::new();
    let res = run_app(&mut terminal, &mut todos, &mut input_buffer);

    // restore terminal - use result to ensure proper cleanup
    let cleanup_result = cleanup_terminal(terminal);

    // Print any errors after cleanup
    if let Err(err) = res {
        eprintln!("Application error: {err:?}");
    }

    cleanup_result?;

    Ok(())
}

fn setup_panic_handler() {
    let original_hook = panic::take_hook();
    panic::set_hook(
        Box::new(move |panic_info| {
            let _ = disable_raw_mode();
            let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
            original_hook(panic_info);
        })
    );
}

fn cleanup_terminal<B: Backend + io::Write>(
    mut terminal: Terminal<B>
) -> Result<(), Box<dyn Error>> {
    // Disable raw mode first
    disable_raw_mode()?;

    // Clear the terminal
    terminal.clear()?;

    // Leave alternate screen and restore mouse
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;

    // Show cursor
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    todos: &mut Vec<Todo>,
    input_buffer: &mut String
) -> io::Result<()> {
    let mut output_buffer = String::new();
    loop {
        terminal.draw(|f| display::ui(f, todos, input_buffer, &output_buffer))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                output_buffer.clear();
                match key.code {
                    KeyCode::Esc => {
                        // Exit immediately
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        let input = input_buffer.trim();
                        if !input.is_empty() {
                            let parts: Vec<&str> = input.splitn(2, ' ').collect();
                            let command = parts[0].to_lowercase();

                            match command.as_str() {
                                "quit" | "exit" | "q" => {
                                    // Exit immediately
                                    return Ok(());
                                }
                                // ... rest of your commands
                                _ => {
                                    output_buffer = "Invalid command".to_string();
                                }
                            }
                        }
                        input_buffer.clear();
                    }
                    KeyCode::Char(c) => {
                        input_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        input_buffer.pop();
                    }
                    _ => {}
                }
            }
        }
    }
}

fn get_todos() -> Vec<Todo> {
    Todo::load_all()
}
