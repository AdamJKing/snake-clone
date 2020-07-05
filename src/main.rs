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
use tui::style::Color;
use tui::{
    backend::TermionBackend,
    symbols::Marker,
    widgets::{canvas::*, Block, Borders},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let width = args
        .get(1)
        .expect("Width is missing")
        .parse::<u16>()
        .expect("Width must be a number");

    let height = args
        .get(2)
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
    let grid = Grid::new(width, height);

    loop {
        terminal.draw(|mut f| {
            let canvas = Canvas::default()
                .block(Block::default().title("Snake").borders(Borders::ALL))
                .x_bounds([0.0, grid.width as f64])
                .y_bounds([0.0, grid.height as f64])
                .marker(Marker::Dot)
                .paint(|ctx| {
                    let coords: &Vec<(f64, f64)> = &grid
                        .snake
                        .points
                        .iter()
                        .map(|&(x, y)| (x as f64, y as f64))
                        .collect();

                    let points = Points {
                        coords,
                        color: Color::White,
                    };

                    ctx.draw(&points);
                });

            f.render_widget(canvas, f.size());
        })?;

        if let Some(input) = input.next() {
            if let Key::Char('q') = input? {
                break;
            }
        }
    }

    Ok(())
}
