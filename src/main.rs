mod commands;
mod screen;
mod states;
mod utils;

use screen::Screen;
use std::io::Result;

fn main() -> Result<()> {
    let mut screen = Screen::new();
    loop {
        screen.render()?
    }
}
