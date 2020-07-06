use crate::geo::Point;
use std::collections::vec_deque;
use std::collections::vec_deque::VecDeque;

pub struct Snake {
    points: VecDeque<Point>,
    movement: Movement,
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

    pub fn update(&mut self) {
        let (x, y) = self.points[0];

        match &self.movement {
            Movement::Still => {}
            Movement::Up => {
                self.points.push_front((x, y + 1));
                self.points.pop_back();
            }

            Movement::Right => {
                self.points.push_front((x + 1, y));
                self.points.pop_back();
            }

            Movement::Down => {
                self.points.push_front((x, y - 1));
                self.points.pop_back();
            }

            Movement::Left => {
                self.points.push_front((x - 1, y));
                self.points.pop_back();
            }
        }
    }

    pub fn movement(&mut self, movement: Movement) {
        self.movement = movement;
    }
}

pub enum Movement {
    Still,
    Up,
    Right,
    Down,
    Left,
}
