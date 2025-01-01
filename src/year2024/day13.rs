//! # Claw Contraption
//!
//! Each claw machine is a system of two linear equations:
//!
//! ```none
//!     (Button A X) * (A presses) + (Button B X) * (B presses) = Prize X
//!     (Button A Y) * (A presses) + (Button B Y) * (B presses) = Prize Y
//! ```
//!
//! Shortening the names and representing as a matrix:
//!
//! ```none
//!     [ ax bx ][ a ] = [ px ]
//!     [ ay by ][ b ] = [ py ]
//! ```
//!
//! To solve we invert the 2 x 2 matrix then premultiply the right column.
use crate::util::iter::*;
use crate::util::parse::*;

type Claw = [i64; 6];

pub fn parse(input: &str) -> Vec<Claw> {
    input.iter_signed().chunk::<6>().collect()
}

pub fn part1(input: &[Claw]) -> i64 {
    input.iter().map(|row| play(row, false)).sum()
}

pub fn part2(input: &[Claw]) -> i64 {
    input.iter().map(|row| play(row, true)).sum()
}

/// Invert the 2 x 2 matrix representing the system of linear equations.
fn play(&[ax, ay, bx, by, mut px, mut py]: &Claw, part_two: bool) -> i64 {
    if part_two {
        px += 10_000_000_000_000;
        py += 10_000_000_000_000;
    }

    // If determinant is zero there's no solution.
    let det = ax * by - ay * bx;
    if det == 0 {
        return 0;
    }

    let mut a = by * px - bx * py;
    let mut b = ax * py - ay * px;

    // Integer solutions only.
    if a % det != 0 || b % det != 0 {
        return 0;
    }

    a /= det;
    b /= det;

    if part_two || (a <= 100 && b <= 100) { 3 * a + b } else { 0 }
}
