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
    input.iter().map(|values| values.last().unwrap() - values.first().unwrap()).sum()
}

pub fn part2(input: &Input) -> u32 {
    input
        .iter()
        .map(|values| {
            for (i, &smaller) in values.iter().enumerate() {
                for &larger in &values[i + 1..] {
                    if larger.is_multiple_of(smaller) {
                        return larger / smaller;
                    }
                }
            }
            unreachable!()
        })
        .sum()
}
