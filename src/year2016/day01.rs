//! # No Time for a Taxicab
//!
//! The solution is short as it leverages three utility classes, [`hash`] for speedy sets,
//! [`parse`] for extracting integers from surrounding text and [`point`] for two dimensional
//! rotations and translations.
//!
//! [`hash`]: crate::util::hash
//! [`parse`]: crate::util::parse
//! [`point`]: crate::util::point
use crate::util::hash::*;
use crate::util::parse::*;
use crate::util::point::*;

type Pair = (u8, i32);

pub fn parse(input: &str) -> Vec<Pair> {
    let first = input.bytes().filter(u8::is_ascii_uppercase);
    let second = input.iter_signed();
    first.zip(second).collect()
}

pub fn part1(input: &[Pair]) -> i32 {
    let mut position = ORIGIN;
    let mut direction = UP;

    for &(turn, amount) in input {
        direction =
            if turn == b'L' { direction.counter_clockwise() } else { direction.clockwise() };
        position += direction * amount;
    }

    position.manhattan(ORIGIN)
}

pub fn part2(input: &[Pair]) -> i32 {
    let mut position = ORIGIN;
    let mut direction = UP;
    let mut visited = FastSet::with_capacity(1000);

    for &(turn, amount) in input {
        direction =
            if turn == b'L' { direction.counter_clockwise() } else { direction.clockwise() };

        for _ in 0..amount {
            position += direction;
            if !visited.insert(position) {
                return position.manhattan(ORIGIN);
            }
        }
    }

    unreachable!()
}
