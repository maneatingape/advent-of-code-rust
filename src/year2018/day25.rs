//! # Four-Dimensional Adventure
//!
//! This problem is the classic [union find](https://en.wikipedia.org/wiki/Disjoint-set_data_structure).
//! However since we only need the *count* of the distinct sets we can use a much simpler approach.
//!
//! Starting with an arbitrary point we find all other points within range, adding them to a
//! todo list. We then transitively determine the neighbors of those points, and so on until
//! all sets have been found.
use crate::util::iter::*;
use crate::util::parse::*;

#[derive(Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

/// Simple 4D point implementation with Manhattan distance.
impl Point {
    fn from([x, y, z, w]: [i32; 4]) -> Self {
        Point { x, y, z, w }
    }

    fn mahattan(&self, other: Self) -> i32 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.w - other.w).abs()
    }
}

pub fn parse(input: &str) -> Vec<Point> {
    input.iter_signed::<i32>().chunk::<4>().map(Point::from).collect()
}

pub fn part1(input: &[Point]) -> usize {
    let mut constellations = 0;
    let mut remaining = input.to_vec();
    let mut todo = Vec::with_capacity(input.len());

    // Choose arbitrary point and start a new constellation.
    while let Some(start) = remaining.pop() {
        constellations += 1;
        todo.push(start);

        while let Some(point) = todo.pop() {
            let mut i = 0;

            // Find all neighbors, adding them to `todo` in order to transitively find all
            // other points in the constellation.
            while i < remaining.len() {
                if point.mahattan(remaining[i]) <= 3 {
                    todo.push(remaining.swap_remove(i));
                } else {
                    i += 1;
                }
            }
        }
    }

    constellations
}

pub fn part2(_input: &[Point]) -> &'static str {
    "n/a"
}
