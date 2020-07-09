use crate::game::Game;
use crate::game::GameData;
use crate::snake::{Movement, Snake};
use termion::event::Key;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Color;
use tui::style::Style;
use tui::symbols::Marker;
use tui::widgets::canvas::Canvas;
use tui::widgets::canvas::Context;
use tui::widgets::canvas::Points;
use tui::widgets::Block;
use tui::widgets::Borders;
use tui::widgets::Paragraph;
use tui::widgets::Text;
use tui::widgets::Widget;

const QUIT_KEY: Key = Key::Char('q');

pub struct GameWidget<'a> {
    pub game: &'a Game,
}

fn paint_snake(ctx: &mut Context, snake: &Snake) {
    let coords: &Vec<(f64, f64)> = &snake.iter().map(|&(x, y)| (x as f64, y as f64)).collect();

    let points = Points {
        coords,
        color: Color::White,
    };

    ctx.draw(&points);
}

fn render_live_game<'a>(
    width: u16,
    height: u16,
    snake: &'a Snake,
) -> Canvas<'a, impl Fn(&mut Context) + 'a> {
    Canvas::default()
        .block(Block::default().title("Snake").borders(Borders::ALL))
        .x_bounds([0.0, width as f64])
        .y_bounds([0.0, height as f64])
        .marker(Marker::Dot)
        .paint(move |ctx| paint_snake(ctx, snake))
}

impl Widget for GameWidget<'_> {
    fn render(self, area: Rect, buff: &mut Buffer) {
        match self.game {
            Game::Ended => {
                let game_over_msg = &Text::raw("Game Over");
                Paragraph::new(std::iter::once(game_over_msg))
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .render(area, buff);
            }

            Game::Live(GameData { grid, snake }) => {
                let canvas = render_live_game(grid.width, grid.height, snake);
                canvas.render(area, buff);
            }
        }
    }
}

fn to_movement(key: Key) -> Option<Movement> {
    match key {
        Key::Left => Some(Movement::Left),
        Key::Up => Some(Movement::Up),
        Key::Right => Some(Movement::Right),
        Key::Down => Some(Movement::Down),
        _ => None,
    }
}

fn update_snake(key: Key, snake: &Snake) -> Snake {
    match to_movement(key) {
        Some(mvmnt) => {
            if snake.movement.can_move_from(&mvmnt) {
                snake.movement(mvmnt)
            } else {
                snake.clone()
            }
        }
        None => snake.clone(),
    }
}

pub fn handle_user_input<T>(game: &Game, input: &mut T) -> Result<Option<Game>, std::io::Error>
where
    T: Iterator<Item = Result<Key, std::io::Error>>,
{
    let next_event = input.next().transpose()?;

    let result = if let Some(key) = next_event {
        if key == QUIT_KEY {
            None
        } else {
            Some(match game {
                Game::Live(data) => {
                    let snake = update_snake(key, &data.snake);
                    Game::Live(GameData { snake, ..*data })
                }
                Game::Ended => Game::Ended,
            })
        }
    } else {
        Some(game.clone())
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geo::Grid;

    fn example_game() -> Game {
        Game::Live(GameData {
            grid: Grid {
                width: 100,
                height: 100,
            },
            snake: Snake::new((10, 10)),
        })
    }

    #[test]
    fn shortcuts_on_quit_event() {
        let game = example_game();
        let result = handle_user_input(&game, &mut std::iter::once(Ok(Key::Char('q')))).unwrap();
        assert!(result.is_none());

        let result = handle_user_input(&game, &mut std::iter::once(Ok(Key::Char('a')))).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn ignore_user_input_when_game_ended() {
        let mut input = [Key::Left, Key::Right, Key::Down, Key::Up]
            .iter()
            .map(|&k| Ok(k));

        let result = handle_user_input(&Game::Ended, &mut input);

        match result {
            Ok(Some(game)) => assert_eq!(game, Game::Ended),
            other => panic!(format!("Unexpected output: {:?}", other)),
        }
    }

    #[test]
    fn returns_current_game_when_no_user_input() {
        let game = example_game();
        let result = handle_user_input(&game, &mut std::iter::empty());

        match result {
            Ok(Some(result)) => assert_eq!(result, game),
            other => panic!(format!("Unexpected output: {:?}", other)),
        }
    }
}
