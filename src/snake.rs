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

pub fn move_snake(grid: &Grid, snake: &Snake) -> Option<Snake> {
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
        Movement::Still => Some(snake.clone()),
        Movement::Up => {
            let next_move = (x, y + 1);

            if snake.points.contains(&next_move) {
                None
            } else {
                points.push_front(next_move);
                points.pop_back();
                Some(Snake { points, ..*snake })
            }
        }

        Movement::Right => {
            let next_move = (x + 1, y);

            if snake.points.contains(&next_move) {
                None
            } else {
                points.push_front(next_move);
                points.pop_back();
                Some(Snake { points, ..*snake })
            }
        }

        Movement::Down => {
            let next_move = (x, y - 1);

            if snake.points.contains(&next_move) {
                None
            } else {
                points.push_front(next_move);
                points.pop_back();
                Some(Snake { points, ..*snake })
            }
        }

        Movement::Left => {
            let next_move = (x - 1, y);

            if snake.points.contains(&next_move) {
                None
            } else {
                points.push_front(next_move);
                points.pop_back();
                Some(Snake { points, ..*snake })
            }
        }
    }
}

impl Snake {
    pub fn new(starting_point: Point) -> Snake {
        let mut points = VecDeque::new();

        points.push_front(starting_point);
        points.push_front(starting_point);
        points.push_front(starting_point);
        points.push_front(starting_point);
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

    pub fn increase_length(&self) -> Snake {
        let mut points = self.points.clone();
        points.push_front(*self.head());
        Snake { points, ..*self }
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
        let grid = Grid { size: 10 };

        let snake = Snake::new((0, 0)).movement(Movement::Left);
        let moved_snake = move_snake(&grid, &snake);
        assert!(moved_snake.is_none());

        let snake = Snake::new((0, 0)).movement(Movement::Down);
        let moved_snake = move_snake(&grid, &snake);
        assert!(moved_snake.is_none());

        let snake = Snake::new((10, 10)).movement(Movement::Right);
        let moved_snake = move_snake(&grid, &snake);
        assert!(moved_snake.is_none());

        let snake = Snake::new((10, 10)).movement(Movement::Up);
        let moved_snake = move_snake(&grid, &snake);
        assert!(moved_snake.is_none());
    }

    #[test]
    fn moving_inside_the_grid() {
        let grid = Grid { size: 10 };

        let snake = Snake::new((5, 5)).movement(Movement::Left);
        let moved_snake = move_snake(&grid, &snake);
        assert_eq!(moved_snake.expect("snake is in grid").head(), &(4, 5));

        let snake = Snake::new((5, 5)).movement(Movement::Down);
        let moved_snake = move_snake(&grid, &snake);
        assert_eq!(moved_snake.expect("snake is in grid").head(), &(5, 4));

        let snake = Snake::new((5, 5)).movement(Movement::Right);
        let moved_snake = move_snake(&grid, &snake);
        assert_eq!(moved_snake.expect("snake is in grid").head(), &(6, 5));

        let snake = Snake::new((5, 5)).movement(Movement::Up);
        let moved_snake = move_snake(&grid, &snake);
        assert_eq!(moved_snake.expect("snake is in grid").head(), &(5, 6));
    }
}
