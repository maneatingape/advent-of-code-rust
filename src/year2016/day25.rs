//! # Clock Signal
//!
//! Like [`Day 12`] and [`Day 23`] this problem is all about *reading* code not writing code.
//!
//! Reverse engineering the code shows that it takes the initial value of `a` then adds
//! two constants multiplied by each other to create a seed.
//!
//! This seed is then repeatedly bit shifted right by dividing by 2 using an inefficient linear
//! time loop. The remainder (the bit that drops off) is the output. This means that output
//! sequence is simply the binary digits of `a + offset` in reverse repeated over and over.
//!
//! To obtain the desired pattern we need the next highest binary number that has the
//! pattern `101010..`.
//!
//! [`Day 12`]: crate::year2016::day12
//! [`Day 23`]: crate::year2016::day23
use crate::util::parse::*;

/// Extract the constant offset from the assembunny code.
pub fn parse(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let first: u32 = lines[1].unsigned();
    let second: u32 = lines[2].unsigned();
    first * second
}

pub fn part1(input: &u32) -> u32 {
    let offset = *input;
    let mut result = 0;

    // Find the next number with binary pattern `101010..` greater than the input.
    while result < offset {
        result = (result << 2) | 2;
    }

    result - offset
}

pub fn part2(_input: &u32) -> &'static str {
    "n/a"
}
