use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Point(pub i32, pub i32);

pub const ORIGIN: Point = Point(0, 0);
pub const UP: Point = Point(0, -1);
pub const DOWN: Point = Point(0, 1);
pub const LEFT: Point = Point(-1, 0);
pub const RIGHT: Point = Point(1, 0);

pub static ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}