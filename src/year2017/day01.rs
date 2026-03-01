//! # Inverse Captcha
use crate::util::parse::*;

pub fn parse(input: &str) -> &[u8] {
    input.trim().as_bytes()
}

pub fn part1(input: &[u8]) -> u32 {
    let last = input.len() - 1;
    sum(&input[..last], &input[1..]) + sum(&input[..1], &input[last..])
}

pub fn part2(input: &[u8]) -> u32 {
    let (first, second) = input.split_at(input.len() / 2);
    2 * sum(first, second)
}

fn sum(a: &[u8], b: &[u8]) -> u32 {
    a.iter().zip(b).filter_map(|(a, b)| (a == b).then_some(a.to_decimal() as u32)).sum()
}
