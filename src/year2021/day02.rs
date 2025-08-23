//! # Dive!
//!
//! Both part 1 and part 2 rely on the [`fold`] method. This method comes in useful for a lot
//! of Advent of Code problems so is handy to know about. The input is parsed into a tuple enum
//! [`Sub`] for convenience.
//!
//! [`fold`]: Iterator::fold
use crate::util::iter::*;
use crate::util::parse::*;

#[derive(Clone, Copy)]
pub enum Sub {
    Up(i32),
    Down(i32),
    Forward(i32),
}

pub fn parse(input: &str) -> Vec<Sub> {
    input
        .split_ascii_whitespace()
        .chunk::<2>()
        .map(|[first, second]| {
            let amount = second.signed();
            match first {
                "up" => Sub::Up(amount),
                "down" => Sub::Down(amount),
                "forward" => Sub::Forward(amount),
                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn part1(input: &[Sub]) -> i32 {
    let (position, depth) =
        input.iter().copied().fold((0, 0), |(position, depth), next| match next {
            Sub::Up(n) => (position, depth - n),
            Sub::Down(n) => (position, depth + n),
            Sub::Forward(n) => (position + n, depth),
        });
    position * depth
}

pub fn part2(input: &[Sub]) -> i32 {
    let (position, depth, _) =
        input.iter().copied().fold((0, 0, 0), |(position, depth, aim), next| match next {
            Sub::Up(n) => (position, depth, aim - n),
            Sub::Down(n) => (position, depth, aim + n),
            Sub::Forward(n) => (position + n, depth + aim * n, aim),
        });
    position * depth
}
