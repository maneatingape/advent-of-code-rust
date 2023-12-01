//! # Trebuchet?!
//!
//! The input can contain overlapping digits such as "twone", so we only remove a letter at a time
//! until the starting or ending digits are found.
use crate::util::parse::*;

/// Use the index of each digit as its implicit value.
const DIGITS: [&[u8]; 9] =
    [b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine"];

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| {
            let first = line.bytes().find(u8::is_ascii_digit).unwrap().to_decimal();
            let last = line.bytes().rfind(u8::is_ascii_digit).unwrap().to_decimal();
            (10 * first + last) as u32
        })
        .sum()
}

pub fn part2(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| {
            let mut line = line.as_bytes();

            let first = 'outer: loop {
                if line[0].is_ascii_digit() {
                    break line[0].to_decimal() as usize;
                }
                for (value, digit) in DIGITS.iter().enumerate() {
                    if line.starts_with(digit) {
                        break 'outer value + 1;
                    }
                }
                line = &line[1..];
            };

            let last = 'outer: loop {
                if line[line.len() - 1].is_ascii_digit() {
                    break line[line.len() - 1].to_decimal() as usize;
                }
                for (value, digit) in DIGITS.iter().enumerate() {
                    if line.ends_with(digit) {
                        break 'outer value + 1;
                    }
                }
                line = &line[..line.len() - 1];
            };

            10 * first + last
        })
        .sum()
}
