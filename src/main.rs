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

    let mut input = termion::async_stdin().keys();
    let mut game = Game::new(width, height);

    let tick_length = time::Duration::from_millis(60);

    loop {
        let start = time::Instant::now();

        terminal.draw(|mut f| {
            let game_widget = GameWidget { game: &game };
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
