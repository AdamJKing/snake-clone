use crate::geo::Point;
use std::collections::vec_deque::*;

pub struct Snake {
    pub points: VecDeque<Point>,
}

impl Snake {
    pub fn new(starting_point: Point) -> Snake {
        let mut points = VecDeque::new();
        points.push_front(starting_point);
        Snake { points }
    }
}

enum Movement {
    Still,
    Up,
    Right,
    Down,
    Left,
}
