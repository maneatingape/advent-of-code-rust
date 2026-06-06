//! # Custom Customs
//!
//! This is a disguised binary question like the previous [`day 5`].
//!
//! We can store each passenger's answers as an implicit set in a `u32` since the cardinality
//! is only 26. For each yes answer we set a bit, shifting left based on the letter. For example
//! `acf` would be represented as `100101`.
//!
//! For part one to find groups where any person answered yes, we reduce the group using
//! [bitwise OR](https://en.wikipedia.org/wiki/Bitwise_operation) then count the number of ones
//! for each group using the blazing fast [`count_ones`] intrinsic.
//!
//! Part two is very similar, except that we use a bitwise AND instead. It is faster to run
//! both parts at once during the parse.
//!
//! [`day 5`]: crate::year2020::day05
//! [`count_ones`]: u32::count_ones
use std::iter::once;

type Input = (u32, u32);

pub fn parse(input: &str) -> Input {
    let mut any = 0_u32;
    let mut all = u32::MAX;
    let mut passenger = 0;
    let mut part_one = 0;
    let mut part_two = 0;
    let iter = input.bytes();
    let mut check_group = false;

    // End the input with a double newline, to include last group in the total.
    for ch in iter.chain(once(b'\n')) {
        match ch {
            b'\n' if check_group => {
                // A second newline ends one group and prepares for the next.
                check_group = false;
                part_one += any.count_ones();
                part_two += all.count_ones();
                any = 0;
                all = u32::MAX;
            }
            b'\n' => {
                // A single newline separates passengers within a group.
                any |= passenger;
                all &= passenger;
                passenger = 0;
                check_group = true;
            }
            _ => {
                // A letter sets the next bit for the given passenger.
                // Masking to bits 1-26 is more efficient than subtracting to bits 0-25.
                check_group = false;
                passenger |= 1 << (ch & 31);
            }
        }
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
