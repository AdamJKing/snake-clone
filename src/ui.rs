use crate::game::Food;
use crate::game::Game;
use crate::game::GameData;
use crate::snake::{Movement, Snake};
use termion::event::Key;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Color;
use tui::symbols::Marker;
use tui::widgets::canvas::Canvas;
use tui::widgets::canvas::Context;
use tui::widgets::canvas::Points;
use tui::widgets::Block;
use tui::widgets::Borders;
use tui::widgets::Widget;

const QUIT_KEY: Key = Key::Char('q');

pub struct GameWidget<'a> {
    canvas: Canvas<'a, Box<dyn Fn(&mut Context) + 'a>>,
}

fn paint_snake(ctx: &mut Context, snake: &Snake) {
    let coords: &Vec<(f64, f64)> = &snake.iter().map(|&(x, y)| (x as f64, y as f64)).collect();

    let points = Points {
        coords,
        color: Color::White,
    };

    ctx.draw(&points);
}

fn paint_food(ctx: &mut Context, food: &Food) {
    let (x, y) = food.0;
    ctx.print(x as f64, y as f64, "O", Color::Red);
}

impl<'a> GameWidget<'a> {
    pub fn new(game: &'a Game) -> GameWidget<'a> {
        let block = Block::default().title("Snake").borders(Borders::ALL);
        GameWidget {
            canvas: match game {
                Game::Ended => {
                    let paint_func = Box::new(|ctx: &mut Context| {
                        ctx.print(5.0, 5.0, "Game Over", Color::White);
                    });

                    Canvas::default()
                        .block(block)
                        .x_bounds([0.0, 10.0])
                        .y_bounds([0.0, 10.0])
                        .paint(paint_func)
                }

                Game::Live(GameData {
                    grid, snake, food, ..
                }) => {
                    let paint_func = Box::new(move |ctx: &mut Context| {
                        paint_snake(ctx, &snake);
                        paint_food(ctx, &food);
                    });

                    Canvas::default()
                        .block(block)
                        .x_bounds([0.0, grid.size as f64])
                        .y_bounds([0.0, grid.size as f64])
                        .marker(Marker::Dot)
                        .paint(paint_func)
                }
            },
        }
    }
}

impl Widget for GameWidget<'_> {
    fn render(self, area: Rect, buff: &mut Buffer) {
        self.canvas.render(area, buff);
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
    use crate::game::*;
    use crate::geo::Grid;

    fn example_game() -> Game {
        Game::Live(GameData {
            grid: Grid { size: 100 },
            snake: Snake::new((10, 10)),
            food: Food((15, 15)),
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
