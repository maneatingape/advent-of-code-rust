//! # Dive!
//!
//! Solves both parts at once, relying on the regular nature of the input.
//! Each number is always a single digit.
use crate::util::parse::*;

type Input = (i32, i32);

pub fn parse(input: &str) -> Input {
    let mut slice = input.as_bytes();
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    while !slice.is_empty() {
        let amount = |index: usize| slice[index].to_decimal() as i32;

        (slice, position, depth, aim) = match slice[0] {
            b'u' => (&slice[5..], position, depth, aim - amount(3)),
            b'd' => (&slice[7..], position, depth, aim + amount(5)),
            b'f' => (&slice[10..], position + amount(8), depth + aim * amount(8), aim),
            _ => unreachable!(),
        }
    }

    (position * aim, position * depth)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}
