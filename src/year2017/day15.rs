//! # Dueling Generators
//!
//! Simple but slow solution, implementing each generator as an [`Iterator`].
use crate::util::iter::*;
use crate::util::parse::*;
use std::iter::from_fn;

type Input = [u64; 2];

pub fn parse(input: &str) -> Input {
    input.iter_unsigned().chunk::<2>().next().unwrap()
}

pub fn part1(input: &Input) -> usize {
    let first = generator(input[0], 16807);
    let second = generator(input[1], 48271);
    judge(first, second, 40_000_000)
}

pub fn part2(input: &Input) -> usize {
    let first = generator(input[0], 16807).filter(|&a| a % 4 == 0);
    let second = generator(input[1], 48271).filter(|&b| b % 8 == 0);
    judge(first, second, 5_000_000)
}

fn generator(mut state: u64, factor: u64) -> impl Iterator<Item = u64> {
    from_fn(move || {
        state = (state * factor) % 0x7fffffff;
        Some(state)
    })
}

fn judge(
    first: impl Iterator<Item = u64>,
    second: impl Iterator<Item = u64>,
    items: usize,
) -> usize {
    first.zip(second).take(items).filter(|&(a, b)| a & 0xffff == b & 0xffff).count()
}
