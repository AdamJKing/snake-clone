#[derive(PartialEq, PartialOrd)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn at(x: u16, y: u16) -> Point {
        Point { x, y }
    }
}
