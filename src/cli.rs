use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pomonote")]
#[command(about = "A simple CLI pomodoro todo app", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new todo task
    Add {
        /// Description of the todo task
        #[arg(value_name = "DESCRIPTION")]
        description: String,
    },
    
    /// Remove a todo task by ID
    Remove {
        /// ID of the todo task to remove
        #[arg(value_name = "ID")]
        id: u32,
    },
    
    /// Start a todo task (mark as in progress)
    Start {
        /// ID of the todo task to start
        #[arg(value_name = "ID")]
        id: u32,
    },
}
