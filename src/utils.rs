use std::{ error::Error, io, panic };

use crossterm::{
    event::{ DisableMouseCapture },
    execute,
    terminal::{ disable_raw_mode, LeaveAlternateScreen },
};
use ratatui::{ backend::{ Backend }, Terminal };

pub fn setup_panic_handler() {
    let original_hook = panic::take_hook();
    panic::set_hook(
        Box::new(move |panic_info| {
            let _ = disable_raw_mode();
            let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
            original_hook(panic_info);
        })
    );
}

pub fn cleanup_terminal<B: Backend + io::Write>(
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

pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}
