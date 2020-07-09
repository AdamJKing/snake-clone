pub type Point = (u16, u16);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Grid {
    pub size: u16,
}

impl Grid {
    pub fn in_bounds(&self, point: Point) -> bool {
        let (x, y) = point;
        let in_x = x <= self.size;
        let in_y = y <= self.size;

        in_x && in_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_in_bounds() {
        let grid = Grid { size: 100 };
        assert!(grid.in_bounds((50, 50)));
        assert!(grid.in_bounds((0, 0)));
        assert!(grid.in_bounds((100, 100)));
    }

    #[test]
    fn is_not_in_bounds() {
        let grid = Grid { size: 100 };
        assert!(!grid.in_bounds((110, 0)));
        assert!(!grid.in_bounds((0, 110)));
        assert!(!grid.in_bounds((110, 110)));
    }
}
