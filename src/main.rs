mod geo;
mod grid;
mod snake;

use crate::grid::Grid;
use std::env;
use std::{error::Error, io};
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{backend::TermionBackend, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let width = args
        .get(1)
        .expect("Width is missing")
        .parse::<u16>()
        .expect("Width must be a number");

    let height = args
        .get(1)
        .expect("Height is missing")
        .parse::<u16>()
        .expect("Height must be a number");

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut input = io::stdin().keys();

    loop {
        terminal.draw(|mut f| {
            let grid = Grid::new(width, height);
            f.render_widget(grid, f.size());
        })?;

        if let Some(input) = input.next() {
            if let Key::Char('q') = input? {
                break;
            }
        }
    }

    Ok(())
}
