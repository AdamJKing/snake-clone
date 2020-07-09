mod game;
mod geo;
mod snake;
mod ui;

use crate::ui::GameWidget;
use game::Game;
use std::env;
use std::{error::Error, io};
use std::{thread, time};
use termion::{
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};

use tui::{backend::TermionBackend, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let grid_size = args
        .get(1)
        .expect("Grid size is missing")
        .parse::<u16>()
        .expect("Grid size must be a number");

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut input = termion::async_stdin().keys();
    let mut game = Game::new(grid_size);

    let tick_length = time::Duration::from_millis(75);

    loop {
        let start = time::Instant::now();

        terminal.draw(|mut f| {
            let game_widget = GameWidget::new(&game);
            f.render_widget(game_widget, f.size());
        })?;

        let handled_input = ui::handle_user_input(&game, &mut input)?;

        match handled_input {
            Some(update) => game = update.advance(),
            None => break,
        }

        if start.elapsed() < tick_length {
            thread::sleep(tick_length - start.elapsed())
        }
    }

    Ok(())
}
