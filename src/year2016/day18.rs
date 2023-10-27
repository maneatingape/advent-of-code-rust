//! # Like a Rogue
//!
//! We represent a trap with a 1 bit and a safe tile with a 0 bit storing the entire row
//! in a [`u128`]. We then use bitwise logic to calculate the next row.
//!
//! Writing out the [truth table](https://en.wikipedia.org/wiki/Truth_table) for the rules:
//!
//! | Left | Center | Right |
//! | ---- | ------ | ----- |
//! |    1 |      1 |     0 |
//! |    0 |      1 |     1 |
//! |    1 |      0 |     0 |
//! |    0 |      0 |     1 |
//!
//! We can see that the value of the center doesn't matter and that the next tile will be a trap
//! if the left and right values are different. We calculate this for all traps at the same time
//! with a bitwise [XOR](https://en.wikipedia.org/wiki/XOR_gate).
pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u32 {
    count(input, 40)
}

pub fn part2(input: &str) -> u32 {
    count(input, 400_000)
}

fn count(input: &str, rows: u32) -> u32 {
    let width = input.len() as u32;
    // We don't use the full 128 bit width so create a mask the same width as the input
    // to prevent bits spilling over.
    let mask = (1 << width) - 1;

    // Represent each trap as a `1` bit.
    let mut total = 0;
    let mut row = input.bytes().fold(0, |acc, b| (acc << 1) | (b == b'^') as u128);

    for _ in 0..rows {
        // Count the traps in each row.
        total += row.count_ones();
        // Only consider the left and right values for the next row.
        row = (row << 1) ^ (row >> 1) & mask;
    }

    // We want the number of safe tiles so convert from the number of traps.
    rows * width - total
}
