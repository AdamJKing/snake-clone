use rand::Rng;
use std::cmp::*;
use std::collections::vec_deque::*;
use tui::{buffer::Buffer as TuiBuffer, layout::Rect, widgets::Widget};

#[derive(PartialEq, PartialOrd)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    fn at(x: u16, y: u16) -> Point {
        Point { x, y }
    }
}

pub struct Grid {
    width: u16,
    height: u16,
    snake: Snake,
}

pub struct Snake(VecDeque<Point>);

impl Snake {
    fn new(starting_point: Point) -> Snake {
        let mut points = VecDeque::new();
        points.push_front(starting_point);
        Snake(points)
    }
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Grid {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0, width);
        let y = rng.gen_range(0, height);
        let snake = Snake::new(Point::at(x, y));

        Grid {
            width,
            height,
            snake,
        }
    }
}

fn draw_chunk_at(buff: &mut TuiBuffer, x: u16, y: u16, chunk_width: u16, chunk_height: u16) {
    for i in x..(x + chunk_width) {
        for j in y..(y + chunk_height) {
            buff.get_mut(i, j).set_symbol(tui::symbols::block::FULL);
        }
    }
}

impl Widget for Grid {
    fn render(self, area: Rect, buff: &mut TuiBuffer) {
        let chunk_width = area.width / self.width;
        let chunk_height = area.height / self.height;

        for &Point { x, y } in &self.snake.0 {
            draw_chunk_at(buff, x, y, chunk_width, chunk_height);
        }
    }
}
