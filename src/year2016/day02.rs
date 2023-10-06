//! # Bathroom Security
//!
//! Relies heavily on the [`point`] and [`grid`] modules.
//!
//! [`grid`]: crate::util::grid
//! [`point`]: crate::util::point
use crate::util::grid::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> String {
    let digits = Grid::parse("123\n456\n789");
    let mut position = ORIGIN;
    let mut result = String::new();

    for line in input {
        for b in line.bytes() {
            let next = position + Point::from(b);
            if next.x.abs() <= 1 && next.y.abs() <= 1 {
                position = next;
            }
        }
        result.push(digits[position + Point::new(1, 1)] as char);
    }

    result
}

pub fn part2(input: &[&str]) -> String {
    let digits = Grid::parse("##1##\n#234#\n56789\n#ABC#\n##D##");
    let mut position = Point::new(-2, 0);
    let mut result = String::new();

    for line in input {
        for b in line.bytes() {
            let next = position + Point::from(b);
            if next.manhattan(ORIGIN) <= 2 {
                position = next;
            }
        }
        result.push(digits[position + Point::new(2, 2)] as char);
    }

    result
}
