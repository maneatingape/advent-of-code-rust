use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Mul, Sub};

pub const ORIGIN: Point = Point { x: 0, y: 0 };
pub const UP: Point = Point { x: 0, y: -1 };
pub const DOWN: Point = Point { x: 0, y: 1 };
pub const LEFT: Point = Point { x: -1, y: 0 };
pub const RIGHT: Point = Point { x: 1, y: 0 };
pub const ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_u32(self.x as u32);
        hasher.write_u32(self.y as u32);
    }
}

impl Point {
    pub fn from_byte(b: &u8) -> Point {
        match b {
            b'^' => UP,
            b'v' => DOWN,
            b'<' => LEFT,
            b'>' => RIGHT,
            _ => unreachable!(),
        }
    }

    pub fn from_string(s: &str) -> Point {
        match s {
            "U" => UP,
            "D" => DOWN,
            "L" => LEFT,
            "R" => RIGHT,
            _ => unreachable!(),
        }
    }

    pub fn clockwise(self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn counter_clockwise(self) -> Point {
        Point {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn manhattan(self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
