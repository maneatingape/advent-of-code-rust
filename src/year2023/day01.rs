//! # Trebuchet?!
//!
//! The input can contain overlapping digits such as "twone", so we only remove a letter at a time
//! until the starting or ending digits are found.
use crate::util::parse::*;

/// Use the index of each digit as its implicit value.
const DIGITS: [&[u8]; 9] =
    [b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine"];

pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

pub fn part1(input: &[&[u8]]) -> u32 {
    input
        .iter()
        .map(|line| {
            let first = line.iter().find(|b| b.is_ascii_digit()).unwrap().to_decimal();
            let last = line.iter().rfind(|b| b.is_ascii_digit()).unwrap().to_decimal();
            (10 * first + last) as u32
        })
        .sum()
}

pub fn part2(input: &[&[u8]]) -> u32 {
    input
        .iter()
        .map(|line| {
            let digit = |i: usize| -> Option<u32> {
                if line[i].is_ascii_digit() {
                    return Some(line[i].to_decimal() as u32);
                }
                DIGITS
                    .iter()
                    .zip(1..)
                    .find_map(|(digit, value)| line[i..].starts_with(digit).then_some(value))
            };

            let first = (0..line.len()).find_map(digit).unwrap();
            let last = (0..line.len()).rev().find_map(digit).unwrap();
            10 * first + last
        })
        .sum()
}
