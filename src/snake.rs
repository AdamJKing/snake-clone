use crate::geo::*;
use std::collections::vec_deque;
use std::collections::vec_deque::VecDeque;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Snake {
    points: VecDeque<Point>,
    pub movement: Movement,
}

pub struct Iter<'a> {
    internal: vec_deque::Iter<'a, Point>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Point;
    fn next(&mut self) -> Option<&'a Point> {
        self.internal.next()
    }
}

pub fn move_snake(grid: Grid, snake: &Snake) -> Option<Snake> {
    let mut points = snake.points.clone();
    let &(x, y) = snake.head();

    if !grid.in_bounds((x + 1, y + 1)) {
        return None;
    }

    if snake.movement == Movement::Left && x == 0 {
        return None;
    }

    if snake.movement == Movement::Down && y == 0 {
        return None;
    }

    match &snake.movement {
        Movement::Still => (),
        Movement::Up => {
            points.push_front((x, y + 1));
            points.pop_back();
        }

        Movement::Right => {
            points.push_front((x + 1, y));
            points.pop_back();
        }

        Movement::Down => {
            points.push_front((x, y - 1));
            points.pop_back();
        }

        Movement::Left => {
            points.push_front((x - 1, y));
            points.pop_back();
        }
    }

    Some(Snake {
        points,
        movement: snake.movement,
    })
}

impl Snake {
    pub fn new(starting_point: Point) -> Snake {
        let mut points = VecDeque::new();
        points.push_front(starting_point);
        Snake {
            points,
            movement: Movement::Still,
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            internal: self.points.iter(),
        }
    }

    pub fn movement(&self, movement: Movement) -> Snake {
        Snake {
            movement,
            points: self.points.clone(),
        }
    }

    pub fn head(&self) -> &Point {
        &self.points[0]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Movement {
    Still,
    Up,
    Right,
    Down,
    Left,
}

impl Movement {
    pub fn can_move_from(&self, movement: &Movement) -> bool {
        match self {
            Movement::Still => true,
            Movement::Left => movement != &Movement::Right,
            Movement::Up => movement != &Movement::Down,
            Movement::Right => movement != &Movement::Left,
            Movement::Down => movement != &Movement::Up,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_snake_is_one_point() {
        let snake = Snake::new((0, 0));
        assert_eq!(snake.points.len(), 1);
    }

    #[test]
    fn moving_snake_outside_of_grid() {
        let grid = Grid {
            width: 10,
            height: 10,
        };

        let snake = Snake::new((0, 0)).movement(Movement::Left);
        let moved_snake = move_snake(grid, &snake);
        assert!(moved_snake.is_none());

        let snake = Snake::new((0, 0)).movement(Movement::Down);
        let moved_snake = move_snake(grid, &snake);
        assert!(moved_snake.is_none());

        let snake = Snake::new((10, 10)).movement(Movement::Right);
        let moved_snake = move_snake(grid, &snake);
        assert!(moved_snake.is_none());

        let snake = Snake::new((10, 10)).movement(Movement::Up);
        let moved_snake = move_snake(grid, &snake);
        assert!(moved_snake.is_none());
    }

    #[test]
    fn moving_inside_the_grid() {
        let grid = Grid {
            width: 10,
            height: 10,
        };

        let snake = Snake::new((5, 5)).movement(Movement::Left);
        let moved_snake = move_snake(grid, &snake);
        assert_eq!(moved_snake.expect("snake is in grid").head(), &(4, 5));

        let snake = Snake::new((5, 5)).movement(Movement::Down);
        let moved_snake = move_snake(grid, &snake);
        assert_eq!(moved_snake.expect("snake is in grid").head(), &(5, 4));

        let snake = Snake::new((5, 5)).movement(Movement::Right);
        let moved_snake = move_snake(grid, &snake);
        assert_eq!(moved_snake.expect("snake is in grid").head(), &(6, 5));

        let snake = Snake::new((5, 5)).movement(Movement::Up);
        let moved_snake = move_snake(grid, &snake);
        assert_eq!(moved_snake.expect("snake is in grid").head(), &(5, 6));
    }
}
