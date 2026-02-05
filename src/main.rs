mod display;
mod models;
mod reduce;
mod utils;

use crossterm::{
    event::{ EnableMouseCapture },
    execute,
    terminal::{ enable_raw_mode, EnterAlternateScreen },
};
use models::todo::Todo;
use ratatui::{ backend::{ CrosstermBackend }, Terminal };
use std::{ error::Error, io };
use utils::{ setup_panic_handler, cleanup_terminal };
fn main() -> Result<(), Box<dyn Error>> {
    run_app()
}

fn run_app() -> Result<(), Box<dyn Error>> {
    setup_panic_handler();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let todos = get_todos();
    let res = reduce::run_app(&mut terminal, todos);
    let cleanup_result = cleanup_terminal(terminal);

    if let Err(err) = res {
        eprintln!("Application error: {err:?}");
    }

    cleanup_result?;

    Ok(())
}

fn get_todos() -> Vec<Todo> {
    Todo::load_all()
}
