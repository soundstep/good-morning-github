use crate::states::{State, StateDefinition};
use bunt;
use std::io::Result;

#[derive(Debug)]
pub struct SetupState {
    definition: StateDefinition,
}

impl SetupState {
    pub fn new() -> Self {
        SetupState {
            definition: StateDefinition {
                title: String::from("Still sleeping!"),
                commands: None,
            },
        }
    }
}

impl State for SetupState {
    fn print_title(&self) {
        print!("\x1B[2J\x1B[1;1H"); // clear
        bunt::println!("{$red+bold}  {}{/$}", self.definition.title);
        println!();
    }
    fn print_body(&self) {
        bunt::println!(
            "\x20 An enviroment variable named {$yellow}GITHUB_TOKEN{/$} is required.\n \
            \x20It should contain a Github PAT (Personal Access Token).\n \
            \x20To create one visit: https://github.com/settings/tokens/new.\n \
            \x20Scope: repo:status, gist, notifications and read:user."
        );
        println!();
    }
    fn print_commands(&self) {}
    fn print_input(&self) {}
    fn render(&self) -> Result<()> {
        self.print_title();
        self.print_body();
        std::process::exit(1);
    }
}
