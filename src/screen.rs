use crate::states::{SetupState, State, WelcomeState};
use std::env;
use std::io::Result;

#[derive(Debug)]
pub struct Screen {
    state: Box<dyn State>,
}

impl Screen {
    pub fn new() -> Screen {
        let first_state: Box<dyn State> = match env::var("GITHUB_TOKEN") {
            Ok(_) => Box::new(WelcomeState::new()),
            Err(_) => Box::new(SetupState::new()),
        };
        Screen { state: first_state }
    }
    pub fn render(&self) -> Result<()> {
        self.state.render()
    }
    pub fn next(&self, next: Box<dyn State>) {
        println!("NEXT {:?}", next);
}

// pub fn start() -> Result<()> {
//     let mut last_command = String::new();
//     loop {
//         print!("\x1B[2J\x1B[1;1H"); // clear
//         println!("  Good Morning Github!");
//         println!();
//         println!("  List of available commands:");
//         println!("  - 1: select repo");
//         println!("  - q: quit");
//         println!();
//         if last_command != "" {
//             println!("Last command: {}", last_command);
//         }
//         bunt::print!("{$bg:yellow+black+bold} Some scope {/$} >> ");
//         stdout().flush()?;
//         let mut input = String::new();
//         stdin().read_line(&mut input).unwrap();

//         last_command = input.clone();

//         println!("line input: {}", input);
//         // current_command = input;

//         // let mut parts = input.trim().split_whitespace();
//         // let command = parts.next().unwrap();
//         // let args = parts;

//         // let mut child = Command::new(command).args(args).spawn().unwrap();

//         // don't accept another command until this one completes
//     }
//     // child.wait()?;
// }
