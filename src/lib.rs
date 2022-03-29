use bunt;
use std::io::{stdin, stdout, Result, Write};

pub fn start() -> Result<()> {
    let mut last_command = String::new();
    loop {
        print!("\x1B[2J\x1B[1;1H"); // clear
        println!("  Good Morning Github!");
        println!();
        println!("  List of available commands:");
        println!("  - 1: select repo");
        println!("  - q: quit");
        println!();
        if last_command != "" {
            println!("Last command: {}", last_command);
        }
        bunt::print!("{$bg:yellow+black+bold} Some scope {/$} >> ");
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        last_command = input.clone();

        println!("line input: {}", input);
        // current_command = input;

        // let mut parts = input.trim().split_whitespace();
        // let command = parts.next().unwrap();
        // let args = parts;

        // let mut child = Command::new(command).args(args).spawn().unwrap();

        // don't accept another command until this one completes
    }
    // child.wait()?;
}