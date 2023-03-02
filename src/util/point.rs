//! Comprehensive 2 dimensional point implementation. This class is designed to work together
//! with the [`Grid`] class.
//!
//! A common theme in AoC is operations in 2 dimensions. This module provides a [`Point`] struct
//! along with implementations of several of the [`std::ops`] traits to support
//! operator overloading, that allows shorthand expressions such as:
//!
//! ```
//!   # use aoc::util::point::Point;
//!
//!   let a = Point { x: 1, y: 2 };
//!   let b = Point { x: 3, y: 4 };
//!   let k = 2;
//!
//!   assert_eq!(a + b, Point { x: 4, y: 6 });
//!   assert_eq!(a - b, Point { x: -2, y: -2 });
//!   assert_eq!(a * k, Point { x: 2, y: 4 });
//! ```
//!
//! Additionally there are [`clockwise`] and [`counter_clockwise`] functions for 90 degree rotations
//! and a [`manhattan`] function for the
//! [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between 2 points.
//!
//! [`clockwise`]: Point::clockwise
//! [`counter_clockwise`]: Point::counter_clockwise
//! [`manhattan`]: Point::manhattan
//! [`Grid`]: crate::util::grid
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
