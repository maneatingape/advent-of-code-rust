//! # Like a Rogue
//!
//! We represent a trap with a 1 bit and a safe tile with a 0 bit.
//! Writing out the [truth table](https://en.wikipedia.org/wiki/Truth_table) for the rules:
//!
//! | Left | Center | Right | Output |
//! | ---- | ------ | ----- | ------ |
//! |    1 |      1 |     0 |      1 |
//! |    0 |      1 |     1 |      1 |
//! |    1 |      0 |     0 |      1 |
//! |    0 |      0 |     1 |      1 |
//! |    1 |      1 |     1 |      0 |
//! |    0 |      1 |     0 |      0 |
//! |    1 |      0 |     1 |      0 |
//! |    0 |      0 |     0 |      0 |
//!
//! We can see that the value of the center doesn't matter and that the next tile will be a trap
//! if the left and right values are different. We calculate this for all traps at the same time
//! with a bitwise [XOR](https://en.wikipedia.org/wiki/XOR_gate).
//!
//! Since even columns depend only on odd columns and vice-versa, we split the input into two,
//! storing each half using 50 bits of a `u64`.
pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> usize {
    count(input, 40)
}

pub fn part2(input: &str) -> usize {
    count(input, 400_000)
}

fn count(input: &str, rows: usize) -> usize {
    // We don't use all the bits in each `u64` so create a mask the same width as
    // half the input to prevent bits spilling over.
    let mask = (1 << (input.len() / 2)) - 1;
    // Represent each trap as a `1` bit.
    let traps = |acc: u64, b: u8| (acc << 1) | u64::from(b == b'^');

    // Split traps into two halves.
    let mut even = input.bytes().step_by(2).fold(0, traps);
    let mut odd = input.bytes().skip(1).step_by(2).fold(0, traps);
    let mut total = 0;

    for _ in 0..rows {
        // Count the traps in each row.
        total += even.count_ones() + odd.count_ones();

        // Calculate the next row of even traps from odd traps and vice-versa.
        (even, odd) = (odd ^ (odd >> 1), even ^ ((even << 1) & mask));
    }

    // We want the number of safe tiles so convert from the number of traps.
    input.len() * rows - total as usize
}
