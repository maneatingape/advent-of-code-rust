//! # Two-Factor Authentication
//!
//! The pixels are sparse enough that's it efficient to store them as [`Point`] objects and
//! manipulate individually. Pixels don't overlap so we can use a vec instead of a set to store
//! distinct points without overcounting.
//!
//! [`Point`]: crate::util::point
use crate::util::grid::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Vec<Point> {
    let amounts = input.iter_signed::<i32>().chunk::<2>();
    let mut points = Vec::new();

    for (line, [a, b]) in input.lines().zip(amounts) {
        if line.starts_with("rect") {
            for x in 0..a {
                for y in 0..b {
                    points.push(Point::new(x, y));
                }
            }
        } else if line.starts_with("rotate row") {
            for point in &mut points {
                if point.y == a {
                    point.x = (point.x + b) % 50;
                }
            }
        } else {
            for point in &mut points {
                if point.x == a {
                    point.y = (point.y + b) % 6;
                }
            }
        }
    }

    points
}

pub fn part1(input: &[Point]) -> usize {
    input.len()
}

pub fn part2(input: &[Point]) -> String {
    let mut grid = Grid::new(50, 6, '.');

    (0..6).for_each(|y| grid[Point::new(0, y)] = '\n');
    input.iter().for_each(|&p| grid[p + RIGHT] = '#');

    grid.bytes.iter().collect()
}
