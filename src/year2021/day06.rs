use crate::util::parse::*;

type Input = [u64; 9];

pub fn parse(input: &str) -> Input {
    let mut fish = [0_u64; 9];
    input.to_unsigned_iter().for_each(|i: usize| fish[i] += 1);
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