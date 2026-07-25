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

/// The square keypad is bounded by a square, starting on `5` in the middle.
pub fn part1(input: &[&str]) -> String {
    let keypad = "123\n456\n789";
    code(input, keypad, ORIGIN, |p| p.x.abs() <= 1 && p.y.abs() <= 1)
}

/// The diamond keypad is bounded by a diamond, starting on `5` at the left.
pub fn part2(input: &[&str]) -> String {
    let keypad = "##1##\n#234#\n56789\n#ABC#\n##D##";
    code(input, keypad, Point::new(-2, 0), |p| p.manhattan(ORIGIN) <= 2)
}

/// Follows the instructions with the keypad centered on the origin, pushing the key reached at
/// the end of each line. Moves that leave the keypad are ignored.
fn code(input: &[&str], keypad: &str, start: Point, inside: impl Fn(Point) -> bool) -> String {
    let digits = Grid::parse(keypad);
    // Translates from origin centered coordinates back into grid coordinates.
    let center = Point::new(digits.width / 2, digits.height / 2);

    let mut position = start;
    let mut result = String::new();

    for line in input {
        for b in line.bytes() {
            let next = position + Point::from(b);
            if inside(next) {
                position = next;
            }
        }
        result.push(digits[position + center] as char);
    }

    result
}
