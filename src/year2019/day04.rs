//! # Secure Container
//!
//! We speed things up by only checking numbers that have digits in non-decreasing order for pairs.
//! These numbers become rapidly less dense as the password value increases and there
//! are only 3003 total of these numbers with 6 digits.
use crate::util::iter::*;
use crate::util::parse::*;

type Input = (u32, u32);

pub fn parse(input: &str) -> Input {
    let [start, end] = input.iter_unsigned::<u32>().chunk::<2>().next().unwrap();

    let mut digits = to_digits(start);
    let end = to_digits(end);

    let mut part_one = 0;
    let mut part_two = 0;

    // Increase the starting number to the next number that has all digits in non-decreasing order
    // to ensure that the incrementing logic in the search loop works correctly.
    // For example 223450 => 223455, 120568 => 122222 and 439999 => 444444.
    if let Some(index) = digits.windows(2).position(|w| w[0] > w[1]) {
        let next = digits[index];
        digits[index..].fill(next);
    }

    while digits <= end {
        // Build a 5 bit binary mask with a `1` if two adjacent digits are equal.
        let mask = digits.windows(2).fold(0, |acc, w| (acc << 1) | (w[0] == w[1]) as u32);

        // Password must contain at least one pair.
        part_one += (mask != 0) as u32;
        // Password must contain at least one pair that's not part of a larger group.
        part_two += (mask & !(mask >> 1) & !(mask << 1) != 0) as u32;

        // Find the next number with all digits in non-decreasing order.
        let index = digits.iter().rposition(|&d| d < b'9').unwrap();
        let next = digits[index] + 1;
        digits[index..].fill(next);
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}

fn to_digits(n: u32) -> [u8; 6] {
    format!("{n:06}").into_bytes().try_into().unwrap()
}
