use crate::commands::Command;
use crate::states::{PullRequestState, State, StateDefinition, StateStatus};
use crate::utils::normalize_input;
use bunt;
use std::io::{stdin, stdout, Result, Write};

#[derive(Debug)]
pub struct WelcomeState {
    definition: StateDefinition,
}

impl WelcomeState {
    pub fn new() -> Self {
        WelcomeState {
            definition: StateDefinition {
                title: String::from("Good Morning Github!"),
                commands: Some(vec![
                    Command::new(String::from("1"), String::from("Pull Requests")),
                    Command::new(String::from("2"), String::from("Issues")),
                    Command::new(String::from("q"), String::from("Quit")),
                ]),
                status: StateStatus::Idle,
            },
        }
    }
}

impl State for WelcomeState {
    fn print_title(&self) {
        print!("\x1B[2J\x1B[1;1H"); // clear
        bunt::println!("{$cyan+bold}  {}{/$}", self.definition.title);
        println!();
    }
    fn print_body(&self) {}
    fn print_commands(&self) {
        if let Some(command_list) = &self.definition.commands {
            println!("  List of available commands:");
            for cmd in command_list.iter() {
                println!("  - {}: {}", cmd.key, cmd.label);
            }
            println!();
        }
    }
    fn print_input(&self) {
        bunt::print!("{$bg:yellow+black+bold} Some scope {/$} >> ");
    }
    fn get_status(&self) -> &StateStatus {
        &self.definition.status
    }
    fn get_next(&self) -> Option<Box<dyn State>> {
        Some(Box::new(PullRequestState::new()))
    }
    fn render(&mut self) -> Result<()> {
        self.print_title();
        self.print_body();
        self.print_commands();
        println!("status: {:?}", self.definition.status);
        self.print_input();
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = normalize_input(input);
        self.definition.status = match input.as_str() {
            "1" => StateStatus::Next,
            "2" => StateStatus::Next,
            "q" => StateStatus::Quit,
            _ => StateStatus::Idle,
        };
        if input.len() == 1 && input == "q" {
            self.definition.status = StateStatus::Quit;
            println!("self.definition.status: {:?}", self.definition.status)
        }
        Ok(())
    }
}
