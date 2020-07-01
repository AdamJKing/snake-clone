use rand::Rng;
use std::cmp::*;
use std::collections::vec_deque::*;
use tui::widgets::canvas::Painter;
use tui::widgets::canvas::Shape;
use tui::{
    buffer::Buffer as TuiBuffer,
    layout::Rect,
    style::Color,
    widgets::{canvas::Canvas, Block, Borders, Widget},
};

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

struct Square {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: Color,
}

impl Shape for Square {
    fn draw(&self, painter: &mut Painter) {
        let upper_left = painter.get_point(self.x, self.y);
        let bottom_right = painter.get_point(self.x + self.width, self.y + self.height);

        if let (Some((x1, y1)), Some((x2, y2))) = (upper_left, bottom_right) {
            for y in y1..y2 {
                for x in x1..x2 {
                    painter.paint(x, y, self.color)
                }
            }
        }
    }
}

impl Widget for &Grid {
    fn render(self, area: Rect, buff: &mut TuiBuffer) {
        let canvas = Canvas::default()
            .block(Block::default().title("Snake").borders(Borders::ALL))
            .x_bounds([0.0, self.width as f64])
            .y_bounds([0.0, self.height as f64])
            .paint(|ctx| {
                for &Point { x, y } in self.snake.0.iter() {
                    let block = Square {
                        x: x as f64,
                        y: y as f64,
                        width: 1.0,
                        height: 1.0,
                        color: Color::White,
                    };

                    ctx.draw(&block)
                }
            });

        canvas.render(area, buff)
    }
}
