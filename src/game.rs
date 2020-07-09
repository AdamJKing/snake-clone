use crate::geo;
use crate::snake::*;
use rand::Rng;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct GameData {
    pub grid: geo::Grid,
    pub snake: Snake,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Game {
    Live(GameData),
    Ended,
}

impl Game {
    pub fn new(grid_size: u16) -> Game {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0, grid_size);
        let y = rng.gen_range(0, grid_size);

        Game::Live(GameData {
            grid: geo::Grid {
                width: grid_size,
                height: grid_size,
            },
            snake: Snake::new((x, y)),
        })
    }

    pub fn advance(&self) -> Game {
        match self {
            Game::Ended => Game::Ended,
            Game::Live(data) => {
                if let Some(snake) = move_snake(data.grid, &data.snake) {
                    Game::Live(GameData { snake, ..*data })
                } else {
                    Game::Ended
                }
            }
        }
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    const EXAMPLE_GAME: Game = Game::Live(GameData {
        grid: Grid {
            width: 100,
            height: 100,
        },
    });

    #[test]
    fn can_equal() {
        assert_eq!(Game::Ended, Game::Ended);
        assert_eq!(EXAMPLE_GAME, EXAMPLE_GAME);
        assert_ne!(EXAMPLE_GAME, Game::Ended);
    }
}
