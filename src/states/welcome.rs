// #[path = "../commands.rs"]
// mod commands;

use crate::commands::Command;
use crate::states::{State, StateDefinition};
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
    fn render(&self) -> Result<()> {
        self.print_title();
        self.print_body();
        self.print_commands();
        self.print_input();
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        Ok(())
    }
}
