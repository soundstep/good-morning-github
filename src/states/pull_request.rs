use crate::commands::Command;
use crate::states::{State, StateDefinition, StateStatus, WelcomeState};
use crate::utils::normalize_input;
use bunt;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::Result;
use std::io::{stdin, stdout, Write};

#[derive(Debug)]
pub struct PullRequestState {
    definition: StateDefinition,
}

impl PullRequestState {
    pub fn new() -> Self {
        PullRequestState {
            definition: StateDefinition {
                title: String::from("Pull Requests"),
                commands: Some(vec![
                    Command::new(String::from("esc"), String::from("Back")),
                    Command::new(String::from("q"), String::from("Quit")),
                ]),
                status: StateStatus::Idle,
            },
        }
    }
}

impl State for PullRequestState {
    fn print_title(&self) {
        print!("\x1B[2J\x1B[1;1H");
        bunt::println!(
            "{$cyan+bold}  {}{/$} {:?}",
            self.definition.title,
            self.definition.status
        );
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
    fn get_previous(&self) -> Option<Box<dyn State>> {
        Some(Box::new(WelcomeState::new()))
    }
    fn render(&mut self) -> Result<()> {
        self.print_title();
        self.print_body();
        self.print_commands();
        self.print_input();
        stdout().flush()?;
        let mut input = String::new();
        // check for escape
        enable_raw_mode().unwrap();
        let event = read().unwrap();
        let next_key = match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
            }) => Some(KeyCode::Esc),
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
            }) => {
                print!("{}", c);
                stdout().flush()?;
                Some(KeyCode::Char(c))
            }
            _ => None,
        };
        disable_raw_mode().unwrap();
        if next_key == Some(KeyCode::Esc) {
            self.definition.status = StateStatus::Previous;
        } else {
            stdin().read_line(&mut input).unwrap();
            let input = normalize_input(input);
            let input = match next_key {
                Some(KeyCode::Char(c)) => c.to_string() + &input,
                _ => input,
            };
            // println!("input {}", input);
            self.definition.status = match input.as_str() {
                "1" => StateStatus::Next,
                "2" => StateStatus::Next,
                "q" => StateStatus::Quit,
                _ => StateStatus::Idle,
            };
            if input.len() == 1 && input == "q" {
                self.definition.status = StateStatus::Quit;
            }
        }
        Ok(())
    }
}
