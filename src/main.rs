mod commands;
mod screen;
mod states;

use screen::Screen;
use std::io::Result;

fn main() -> Result<()> {
    let screen = Screen::new();
    loop {
        screen.render()?
    }
}
