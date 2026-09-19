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
    let directions = input.bytes().filter(u8::is_ascii_uppercase);
    let amounts = input.iter_signed::<i32>();

    // Dial starts at fifty, not zero.
    let (_, part_one, part_two) = directions.zip(amounts).fold(
        (50, 0, 0),
        |(dial, part_one, part_two), (direction, amount)| {
            let (dial, zeros) = if direction == b'R' {
                // Right (or positive) turns use normal modulo.
                let total = dial + amount;
                (total % 100, total / 100)
            } else {
                // To avoid an off-by-one error when the dial is already at zero during a left
                // (or negative) turn, take the reflected value modulo 100.
                let reversed = (100 - dial) % 100;
                ((dial - amount).rem_euclid(100), (reversed + amount) / 100)
            };

            (dial, part_one + i32::from(dial == 0), part_two + zeros)
        },
    );

    (part_one, part_two)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}
