//! # Inverse Captcha
//!
//! Modern hardware is so good at shuffling memory around that it's faster to rotate the entire
//! array instead of stepping through elements one at a time with an index modulo array length.
use crate::util::parse::*;

pub fn parse(input: &str) -> &[u8] {
    input.trim().as_bytes()
}

pub fn part1(input: &[u8]) -> u32 {
    captcha(input, 1)
}

pub fn part2(input: &[u8]) -> u32 {
    captcha(input, input.len() / 2)
}

fn captcha(input: &[u8], offset: usize) -> u32 {
    let mut rotated = input.to_vec();
    rotated.rotate_left(offset);

    input
        .iter()
        .zip(rotated.iter())
        .filter_map(|(a, b)| (a == b).then_some(a.to_decimal() as u32))
        .sum()
}
