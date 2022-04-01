mod pull_request;
mod setup;
mod welcome;

use crate::commands::Command;
pub use pull_request::PullRequestState;
pub use setup::SetupState;
pub use welcome::WelcomeState;

use std::io::Result;

pub trait State: std::fmt::Debug {
    fn print_title(&self);
    fn print_body(&self);
    fn print_commands(&self);
    fn print_input(&self);
    fn get_status(&self) -> &StateStatus {
        &StateStatus::Idle
    }
    fn render(&mut self) -> Result<()> {
        Ok(())
    }
    fn get_next(&self) -> Option<Box<dyn State>> {
        None
    }
}

#[derive(Debug)]
pub struct StateDefinition {
    title: String,
    commands: Option<Vec<Command>>,
    status: StateStatus,
}

#[derive(Debug)]
pub enum StateStatus {
    Idle,
    Quit,
    Previous,
    Next,
}
