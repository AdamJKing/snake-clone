use crate::snake::Snake;
use rand::Rng;

pub struct Grid {
    pub width: u16,
    pub height: u16,
    pub snake: Snake,
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Grid {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0, width);
        let y = rng.gen_range(0, height);
        let snake = Snake::new((x, y));

        Grid {
            width,
            height,
            snake,
        }
    }

    pub fn advance(&mut self) {
        self.snake.update();
    }
}
