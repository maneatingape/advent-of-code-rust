//! # Secret Entrance
//!
//! Computes both parts together. Part two left (or negative) turns are easier to handle
//! if we first "reverse" the dial, then treat it as a right turn. The [`rem_euclid`] method
//! is a modulo operator that handles negative values. For example `-1.rem_euclid(100)` is 99.
//!
//! [`rem_euclid`]: https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid
use crate::util::parse::*;

type Input = (i32, i32);

pub fn parse(input: &str) -> Input {
    let directions = input.bytes().filter(|&b| b.is_ascii_uppercase());
    let amounts = input.iter_signed::<i32>();

    // Dial starts at fifty, not zero.
    let mut dial = 50;
    let mut part_one = 0;
    let mut part_two = 0;

    for (direction, amount) in directions.zip(amounts) {
        if direction == b'R' {
            // Right (or positive) turns use normal modulo.
            part_two += (dial + amount) / 100;
            dial = (dial + amount) % 100;
        } else {
            // To avoid an [off by one error](https://en.wikipedia.org/wiki/Off-by-one_error)
            // when the dial is already at zero during a left (or negative) turn, we take the
            // reflected value modulo 100.
            let reversed = (100 - dial) % 100;
            part_two += (reversed + amount) / 100;
            dial = (dial - amount).rem_euclid(100);
        }
        // Compute part one simultaneously.
        part_one += i32::from(dial == 0);
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}
