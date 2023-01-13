use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub const ORIGIN: Point = Point { x: 0, y: 0 };
pub const UP: Point = Point { x: 0, y: -1 };
pub const DOWN: Point = Point { x: 0, y: 1 };
pub const LEFT: Point = Point { x: -1, y: 0 };
pub const RIGHT: Point = Point { x: 1, y: 0 };

pub static ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Point {
    pub fn manhattan(self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
