mod display;
mod models;
mod reduce;

use crossterm::{
    event::{ DisableMouseCapture, EnableMouseCapture },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
};
use models::todo::Todo;
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
    let todos = get_todos();
    let res = reduce::run_app(&mut terminal, todos);

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

fn get_todos() -> Vec<Todo> {
    Todo::load_all()
}
