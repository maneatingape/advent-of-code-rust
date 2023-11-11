//! # Corruption Checksum
//!
//! Part two is `O(n²)` complexity but at least we can reduce by a factor of two by sorting
//! each line first, allowing us to only compare each number against those that are greater.
//! As a minor benefit this makes part one faster too.
use crate::util::parse::*;

type Input = Vec<Vec<u32>>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut values: Vec<_> = line.iter_unsigned().collect();
            values.sort_unstable();
            values
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    input.iter().map(|values| values[values.len() - 1] - values[0]).sum()
}

pub fn part2(input: &Input) -> u32 {
    input
        .iter()
        .map(|values| {
            for i in 0..values.len() {
                for j in i + 1..values.len() {
                    if values[j] % values[i] == 0 {
                        return values[j] / values[i];
                    }
                }
            }
            unreachable!()
        })
        .sum()
}
