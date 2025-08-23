//! # Binary Boarding
//!
//! The entire part one description is an obfuscated way to describe that each seat id is a 10 bit
//! binary number, where `B` and `R` mean a 1 bit in that position and `F` and `L` mean a 0 bit.
//!
//! To solve part two we can have a little fun. Since we know that only a single seat is missing
//! if we [XOR](https://en.wikipedia.org/wiki/XOR_gate) together all the seat ids from
//! `min` to `max` then XOR with the actual seat ids, the result will be our missing seat id.
pub struct Input {
    min: u32,
    max: u32,
    xor: u32,
}

pub fn parse(input: &str) -> Input {
    let (min, max, xor) = input.lines().fold((u32::MAX, u32::MIN, 0), |(min, max, xor), line| {
        let id = line.bytes().fold(0, |acc, b| (acc << 1) | (b == b'B' || b == b'R') as u32);
        (min.min(id), max.max(id), xor ^ id)
    });

    Input { min, max, xor }
}

pub fn part1(input: &Input) -> u32 {
    input.max
}

pub fn part2(input: &Input) -> u32 {
    let rows = (input.min..=input.max).fold(0, |acc, b| acc ^ b);
    rows ^ input.xor
}
