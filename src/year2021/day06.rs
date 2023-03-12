//! # Lanternfish
//!
//! The key observation is that all fish of the same age behave the same, so we only
//! need to store the *total* of each fish per day, rather than each fish individually.
//!
//! Another optimization trick is rather than modifying the array by removing the fish at day 0,
//! then shifting each fish total down by 1, we can simply increment what we consider the
//! head of the array modulo 9 to achieve the same effect in place.
use crate::util::parse::*;

type Input = [u64; 9];

pub fn parse(input: &str) -> Input {
    let mut fish = [0_u64; 9];
    input.iter_unsigned().for_each(|i: usize| fish[i] += 1);
    fish
}

pub fn part1(input: &Input) -> u64 {
    simulate(input, 80)
}

pub fn part2(input: &Input) -> u64 {
    simulate(input, 256)
}

fn simulate(input: &Input, days: usize) -> u64 {
    let mut fish = *input;
    (0..days).for_each(|day| fish[(day + 7) % 9] += fish[day % 9]);
    fish.iter().sum()
}
