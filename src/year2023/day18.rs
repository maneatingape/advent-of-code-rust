//! # Lavaduct Lagoon
//!
//! Similar approach to [`Day 10`] using the [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula)
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

type Move = (Point, i32);
type Input = (Vec<Move>, Vec<Move>);

pub fn parse(input: &str) -> Input {
    let mut first = Vec::with_capacity(1_000);
    let mut second = Vec::with_capacity(1_000);

    for [a, b, c] in input.split_ascii_whitespace().chunk::<3>() {
        // Parse part one
        let direction = Point::from(a.as_bytes()[0]);
        let amount = b.signed();
        first.push((direction, amount));

        // Parse part two
        let direction = match c.as_bytes()[7] {
            b'0' => RIGHT,
            b'1' => DOWN,
            b'2' => LEFT,
            b'3' => UP,
            _ => unreachable!(),
        };
        let hex = &c[2..c.len() - 2];
        let amount = i32::from_str_radix(hex, 16).unwrap();
        second.push((direction, amount));
    }

    (first, second)
}

pub fn part1(input: &Input) -> i64 {
    lava(&input.0)
}

pub fn part2(input: &Input) -> i64 {
    lava(&input.1)
}

/// Find the volume of the lava which is the number of interior and boundary points.
fn lava(moves: &[Move]) -> i64 {
    let mut position = ORIGIN;
    let mut area = 0;
    let mut perimeter = 0;

    for &(direction, amount) in moves {
        let previous = position;
        position += direction * amount;
        area += determinant(previous, position);
        perimeter += amount as i64;
    }

    // Pick's theorem counting both interior and boundary points.
    area / 2 + perimeter / 2 + 1
}

/// Find the determinant of each pair of points casting to `i64` to prevent overflow.
fn determinant(a: Point, b: Point) -> i64 {
    (a.x as i64) * (b.y as i64) - (a.y as i64) * (b.x as i64)
}
