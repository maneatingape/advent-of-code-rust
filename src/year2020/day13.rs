//! # Shuttle Search
//!
//! Part two is the [Chinese Remainder Theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem).
//! The integers n₁, n₂, ... nₖ map to the bus ids which happen to be prime. This satisfies the
//! requirement that the integers are [pairwise coprime](https://en.wikipedia.org/wiki/Coprime_integers#Coprimality_in_sets).
//!
//! For simplicity we use the "search by sieving" method. We start at zero with a step the size of
//! the first integer. Then we search for each subsequent integer located at the correct offset of
//! minutes and multiply the step by the new integer. This preserve the relative offset at each step
//! in the next search.
use crate::util::parse::*;

pub struct Input {
    timestamp: usize,
    buses: Vec<(usize, usize)>,
}

pub fn parse(input: &str) -> Input {
    let lines: Vec<_> = input.lines().collect();
    let timestamp = lines[0].unsigned();
    let buses: Vec<_> = lines[1]
        .split(',')
        .enumerate()
        .filter(|&(_, id)| id != "x")
        .map(|(offset, id)| (offset, id.unsigned()))
        .collect();
    Input { timestamp, buses }
}

pub fn part1(input: &Input) -> usize {
    let (id, next) = input
        .buses
        .iter()
        .map(|(_, id)| (id, id - input.timestamp % id))
        .min_by_key(|&(_, next)| next)
        .unwrap();

    id * next
}

pub fn part2(input: &Input) -> usize {
    let (mut time, mut step) = input.buses[0];

    for (offset, id) in &input.buses[1..] {
        let remainder = id - offset % id;

        while time % id != remainder {
            time += step;
        }

        step *= id;
    }

    time
}
