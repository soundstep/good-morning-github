#[path = "./commands.rs"]
mod commands;
mod setup;
mod welcome;

use commands::Command;
pub use setup::SetupState;
pub use welcome::WelcomeState;

// use crate::commands::Command;
use std::io::Result;

pub trait State: std::fmt::Debug {
    fn print_title(&self);
    fn print_body(&self);
    fn render(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct StateDefinition {
    title: String,
    commands: Option<Vec<Command>>,
}
