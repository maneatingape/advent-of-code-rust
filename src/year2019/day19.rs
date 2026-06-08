//! # Tractor Beam
//!
//! The intcode program computes a linear inequality: returning true if an integer point lies on
//! or between two lines through the origin with irrational slope. The intcode program was designed
//! so that the two lines are close enough that there are no integer solutions when `y=1`, so there
//! are intentionally one or two discontinuities between the origin and the bulk of the beam. This
//! solution finds the approximate boundary of the upper and lower edges of the beam expressed
//! as an integer ratio for slope. We then skip the relatively expensive intcode test if the x
//! and y coordinates lie outside. Once we identify an edge past the initial discontinuities,
//! scaling along the lines buys more accuracy and thus fewer later intcode runs.
//!
//! For part 2, we can further speed up the process by using geometry to hone in on a viable
//! target to start searching at. If we consider a scaled-down beam with target point A on slope
//! upper as (x1, y1), and point B on slope lower as (x2, y2), the bounding box has a top left
//! corner at point S at (x2, y1):
//! ```none
//!      1
//!      11
//!       111
//!        1111
//!         11111
//!          111111
//!           1111111
//!            11111111
//!             111SxxA11  <--- (x1, y1)
//!              11xxxx1111
//!               1xxxx111111
//!      (x2, y2)> Bxxx11111111
//!                 1111111111111
//! ```
//!
//! For a 100x100 box, we have a system of equations m1=upper/scale≈y1/x1, x1=x2+99,
//! m2=scale/lower≈y2/x2, and y2=y1+99, which we solve as:
//! ```none
//!     x2 = ((m1 * 99 + 99) / (m2 - m1)
//!        ≈ ((upper/scale * 99) + 99) / (scale/lower - upper/scale)
//!        ≈ ((99*upper/scale) + (99*scale/scale)) / ((scale*scale/scale*lower - lower*upper/scale*lower))
//!        ≈ ((99 * (upper + scale)) * (scale*lower)) / (scale * (scale*scale - lower*upper))
//!        ≈ (99*lower * (upper + scale)) / (scale*scale - lower*upper)
//!     y1 = m2 * x2 - 99
//!        ≈ scale*x2 / lower - 99
//! ```
use super::intcode::*;
use crate::util::parse::*;

pub struct Input {
    code: Vec<i64>,
    scale: i64,
    lower: i64, // slope scale/lower just outside left boundary
    upper: i64, // slope upper/scale just outside right boundary
}

pub fn parse(input: &str) -> Input {
    // Pick an initial scale large enough to be past the discontinuities for all known inputs.
    let code: Vec<_> = input.iter_signed().collect();
    let mut lower = 1;
    let mut upper = 1;
    let mut scale = 5;

    // Find approximate slope of lower and upper edges, rounding down to prevent false negatives.
    // Scale the boundary for slightly more accuracy.
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
    let code = &input.code;
    // The origin is always set, and no other point on that row.
    let mut result = 1;

    // Scanning the remaining lines works even around the known discontinuity at y=1, by finding
    // the left and right edges if any.
    for y in 2..50 {
        let left = (1..50).find(|&x| precheck(input, x, y) && test(code, x, y));
        let right = (1..50).rfind(|&x| precheck(input, x, y) && test(code, x, y));
        if let (Some(l), Some(r)) = (left, right) {
            result += r - l + 1;
        }
    }

    result
}

pub fn part2(input: &Input) -> i64 {
    // See comments above about derivation of initial guess for x and y.
    let code = &input.code;
    let mut x = 99 * input.lower * (input.upper + input.scale)
        / (input.scale * input.scale - input.lower * input.upper);
    let mut y = input.scale * x / input.lower - 99;
    let mut moved = true;

    // Increase the right and bottom edges of our box until they are both inside the beam.
    while moved {
        moved = false;

        while !precheck(input, x, y + 99) || !test(code, x, y + 99) {
            x += 1;
            moved = true;
        }

        while !precheck(input, x + 99, y) || !test(code, x + 99, y) {
            y += 1;
            moved = true;
        }
    }

    10000 * x + y
}

/// Quick check with some false positives but no false negatives.
fn precheck(input: &Input, x: i64, y: i64) -> bool {
    input.scale * y > input.upper * x && input.scale * x > input.lower * y
}

/// Definitive but slower check.
fn test(code: &[i64], x: i64, y: i64) -> bool {
    let mut computer = Computer::new(code);
    computer.input(x);
    computer.input(y);

    let State::Output(result) = computer.run() else { unreachable!() };
    result == 1
}
