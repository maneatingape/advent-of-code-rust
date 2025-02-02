//! # Subterranean Sustainability
//!
//! The problem is a one dimensional version of
//! [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).
//!
//! The trick for part two is that the plants will eventually stabilize into a repeating pattern
//! that expands by the same amount each generation. Once 2 deltas between 3 subsequent
//! generations are the same we extrapolate 50 billion generations into the future.
use std::iter::repeat_n;

pub struct Input {
    rules: Vec<usize>,
    state: Tunnel,
}

#[derive(Clone)]
pub struct Tunnel {
    plants: Vec<usize>,
    start: i64,
    sum: i64,
}

pub fn parse(input: &str) -> Input {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
    // Convert ASCII characters to `1` for a plant and `0` for an empty pot.
    let plants: Vec<_> = lines[0][15..].iter().map(|b| (b & 1) as usize).collect();
    // 5 plants gives 2⁵ = 32 possible combinations to consider.
    let mut rules = vec![0; 32];

    // Convert each pattern into an index for fast lookup. For example `..#.#` becomes 5.
    for line in &lines[2..] {
        let binary = line.iter().fold(0, |acc, b| (acc << 1) | (b & 1) as usize);
        rules[binary >> 5] = binary & 1;
    }

    Input { rules, state: Tunnel { plants, start: 0, sum: 0 } }
}

pub fn part1(input: &Input) -> i64 {
    let mut current = input.state.clone();

    for _ in 0..20 {
        current = step(&input.rules, &current);
    }

    current.sum
}

pub fn part2(input: &Input) -> i64 {
    let mut current = input.state.clone();
    let mut delta = 0;
    let mut generations = 0;

    loop {
        let next = step(&input.rules, &current);
        let next_delta = next.sum - current.sum;

        // Two identical deltas indicates that the pattern has stabilized.
        if delta == next_delta {
            break current.sum + delta * (50_000_000_000 - generations);
        }

        current = next;
        delta = next_delta;
        generations += 1;
    }
}

fn step(rules: &[usize], tunnel: &Tunnel) -> Tunnel {
    let mut index = 0;
    let mut sum = 0;
    let mut position = tunnel.start - 2;
    let mut plants = Vec::with_capacity(1_000);

    // Add four extra empty pots to the end to make checking the last pattern easier.
    for plant in tunnel.plants.iter().copied().chain(repeat_n(0, 4)) {
        index = ((index << 1) | plant) & 0b11111;

        sum += position * rules[index] as i64;
        position += 1;

        plants.push(rules[index]);
    }

    // Tunnel expands by 2 pots at each end.
    Tunnel { plants, start: tunnel.start - 2, sum }
}
