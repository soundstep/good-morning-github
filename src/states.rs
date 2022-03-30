mod setup;
mod welcome;

use crate::commands::Command;
pub use setup::SetupState;
pub use welcome::WelcomeState;

// use crate::commands::Command;
use std::io::Result;

pub trait State: std::fmt::Debug {
    fn print_title(&self);
    fn print_body(&self);
    fn print_commands(&self);
    fn print_input(&self);
    fn render(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct StateDefinition {
    title: String,
    commands: Option<Vec<Command>>,
}
