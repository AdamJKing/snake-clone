mod grid;

use crate::grid::Grid;
use std::env;
use std::{error::Error, io};
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
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

    let grid = Grid::new(width, height);

    for _ in 0..100 {
        terminal.draw(|mut f| {
            f.render_widget(&grid, f.size());
        })?;
    }

    Ok(())
}
