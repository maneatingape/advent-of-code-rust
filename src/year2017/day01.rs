//! # Inverse Captcha
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
    let split = input.len() - offset;
    sum(&input[..split], &input[offset..]) + sum(&input[split..], &input[..offset])
}

fn sum(a: &[u8], b: &[u8]) -> u32 {
    a.iter().zip(b).filter_map(|(a, b)| (a == b).then_some(a.to_decimal() as u32)).sum()
}
