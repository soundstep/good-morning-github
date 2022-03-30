use bunt;
use std::io::{stdin, stdout, Result, Write};

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
    fn render(&self) -> Result<()> {
        self.print_title();
        self.print_body();
        // if last_command != "" {
        //     println!("Last command: {}", last_command);
        // }
        // bunt::print!("{$bg:yellow+black+bold} Some scope {/$} >> ");
        // stdout().flush()?;
        // let mut input = String::new();
        // stdin().read_line(&mut input).unwrap();
        // Ok(())
        std::process::exit(1);
    }
}

// impl StateDefinition {
//     fn new(title: String, commands: Vec<Command>) -> Self {
//         StateDefinition { title, commands }
//     }
// }

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
        // if last_command != "" {
        //     println!("Last command: {}", last_command);
        // }
        bunt::print!("{$bg:yellow+black+bold} Some scope {/$} >> ");
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        Ok(())
    }
}
