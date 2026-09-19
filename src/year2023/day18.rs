//! # Lavaduct Lagoon
//!
//! Similar approach to [`Day 10`] using the [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula#Trapezoid_formula)
//! and [Pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem).
//!
//! One nuance is that we want the number of interior *and* boundary points so the final formula is:
//!
//! `i + b => A - b / 2 + 1 + b => A + b / 2 + 1`
//!
//! [`Day 10`]: crate::year2023::day10
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;

type Move = (Point, i64);
type Input = (Vec<Move>, Vec<Move>);

pub fn parse(input: &str) -> Input {
    input
        .split_ascii_whitespace()
        .chunk::<3>()
        .map(|[a, b, c]| {
            // Parse part one.
            let first = (Point::from(a.as_bytes()[0]), b.signed());

            // Parse part two.
            let direction = match c.as_bytes()[7] {
                b'0' => RIGHT,
                b'1' => DOWN,
                b'2' => LEFT,
                b'3' => UP,
                _ => unreachable!(),
            };
            let hex = &c[2..7];
            let second = (direction, i64::from_str_radix(hex, 16).unwrap());

            (first, second)
        })
        .unzip()
}

pub fn part1(input: &Input) -> i64 {
    lava(&input.0)
}

pub fn part2(input: &Input) -> i64 {
    lava(&input.1)
}

/// Find the volume of the lava which is the number of interior and boundary points.
fn lava(moves: &[Move]) -> i64 {
    let mut height = 0;
    let mut half_area = 0;
    let mut perimeter = 0;

    // Instead of computing the area by the shoelace formula (a determinant per vertex), it is
    // faster to use the trapezoid formula: sum((y[i]+y[i+1])*(x[i]-x[i+1])). But all vertical
    // segments have x[i]-x[i+1]=0, so only the horizontal segments, where y[i]+y[i+1]=2y,
    // actually contribute to the area.
    for &(direction, amount) in moves {
        half_area += amount * height * direction.x as i64;
        height += amount * direction.y as i64;
        perimeter += amount;
    }

    // Pick's theorem counting both interior and boundary points.
    half_area.abs() + perimeter / 2 + 1
}
