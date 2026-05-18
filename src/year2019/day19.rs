//! # Tractor Beam
//!
//! The intcode program computes a linear inequality: returning true if an integer point lies on
//! or between two lines through the origin, often with irrational slope. The intcode program was
//! designed so that the two lines are close enough that there are no integer solutions when `y=1`,
//! so there are intentionally one or two discontinuities between the origin and the bulk of the
//! beam. This solution finds the approximate boundary of the upper and lower edges of the beam
//! expressed as an integer ratio for slope. We then skip the relatively expensive intcode test if
//! the x and y coordinates lie outside. Once we identify an edge past the initial discontinuities,
//! scaling along the lines buys more accuracy and thus fewer later intcode runs.
//!
//! For part 2, we can further speed up the process by using geometry to hone in on a viable
//! target to start searching at. Our target point `(x,y)` is related to our two slopes as:
//! ```none
//!   scale*y = upper*(x+99)
//!   scale*x = lower*(y+99)
//! ```
//! Those two equations can be represented in matrix form:
//! ```none
//!   [upper-scale][x] = [-99*upper]
//!   [scale-lower][y] = [ 99*lower]
//! ```
//! where inverting the matrix gives a solution:
//! ```none
//!   determinant = scale * scale - lower * upper
//!   x = 99 * (lower * upper + lower * scale) / determinant
//!   y = 99 * (lower * upper + upper * scale) / determinant
//! ```
use super::intcode::*;
use crate::util::parse::*;

pub struct Input {
    code: Vec<i64>,
    scale: i64,
    lower: i64, // The slope scale/lower just outside left boundary.
    upper: i64, // The slope upper/scale just outside right boundary.
}

pub fn parse(input: &str) -> Input {
    // Pick an initial scale large enough to be past the discontinuities for all known inputs.
    let code: Vec<_> = input.iter_signed().collect();
    let mut lower = 1;
    let mut upper = 1;
    let mut scale = 5;

    // Find approximate slope of lower and upper edges, rounding down to prevent false negatives.
    // Each scaling iteration adds another bit of accuracy to our approximation.
    while scale < 1024 {
        scale *= 2;
        lower *= 2;
        upper *= 2;
        while !test(&code, lower + 1, scale) {
            lower += 1;
        }
        while !test(&code, scale, upper + 1) {
            upper += 1;
        }
    }

    Input { code, scale, lower, upper }
}

pub fn part1(input: &Input) -> i64 {
    // The origin is always set, but no other point occurs on that row or column.
    let mut result = 1;

    // Scan all remaining points; this works even on lines with no integer hits.
    for y in 1..50 {
        let left = (1..50).find(|&x| inside(input, x, y));
        let right = (left.unwrap_or(50)..50).rfind(|&x| inside(input, x, y));
        if let Some((left, right)) = left.zip(right) {
            result += right - left + 1;
        }
    }

    result
}

pub fn part2(input: &Input) -> i64 {
    // See comments above about derivation of initial guess for x and y.
    let determinant = input.scale * input.scale - input.lower * input.upper;
    let mut x = 99 * (input.lower * input.upper + input.lower * input.scale) / determinant;
    let mut y = 99 * (input.lower * input.upper + input.upper * input.scale) / determinant;
    let mut moved = true;

    // Increase the right and bottom edges of our box until they are both inside the beam.
    while moved {
        moved = false;

        while !inside(input, x, y + 99) {
            x += 1;
            moved = true;
        }

        while !inside(input, x + 99, y) {
            y += 1;
            moved = true;
        }
    }

    10000 * x + y
}

/// Skip the relatively expensive intcode test if the point lies outside the beam's slopes.
/// The slope check has some false positives but no false negatives.
fn inside(input: &Input, x: i64, y: i64) -> bool {
    input.scale * y > input.upper * x
        && input.scale * x > input.lower * y
        && test(&input.code, x, y)
}

/// Definitive but slower check.
fn test(code: &[i64], x: i64, y: i64) -> bool {
    let mut computer = Computer::new(code);
    computer.input(x);
    computer.input(y);

    let State::Output(result) = computer.run() else { unreachable!() };
    result == 1
}
