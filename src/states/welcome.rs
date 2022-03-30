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
                commands: Some(Vec::new()),
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
    fn render(&self) -> Result<()> {
        self.print_title();
        self.print_body();
        println!("  List of available commands:");
        println!("  - 1: select repo");
        println!("  - q: quit");
        println!();
        bunt::print!("{$bg:yellow+black+bold} Some scope {/$} >> ");
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        Ok(())
    }
}
