use crate::geo::Point;
use std::collections::vec_deque::*;

pub struct Snake(VecDeque<Point>, Movement);

impl Snake {
    pub fn new(starting_point: Point) -> Snake {
        let mut points = VecDeque::new();
        points.push_front(starting_point);
        Snake(points, Movement::Still)
    }
}

impl IntoIterator for Snake {
    type Item = Point;
    type IntoIter = <VecDeque<Point> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

enum Movement {
    Still,
    Up,
    Right,
    Down,
    Left,
}
