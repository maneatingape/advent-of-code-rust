//! # Inventory Management System
use crate::util::hash::*;

pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

pub fn part1(input: &[&[u8]]) -> u32 {
    let mut twos = 0;
    let mut threes = 0;

    for &id in input {
        // Ids are lowercase ASCII only with cardinality of 26.
        let mut freq = [0_u8; 26];

        for &b in id {
            freq[(b - b'a') as usize] += 1;
        }

        twos += freq.contains(&2) as u32;
        threes += freq.contains(&3) as u32;
    }

    twos * threes
}

pub fn part2(input: &[&[u8]]) -> String {
    let width = input[0].len();
    let mut seen = FastSet::with_capacity(input.len());

    // Use a set to check for duplicates by comparing the prefix and suffix of IDs excluding one
    // column at a time.
    for column in 0..width {
        for &id in input {
            let prefix = &id[..column];
            let suffix = &id[column + 1..];

            if !seen.insert([prefix, suffix]) {
                // Convert to String
                return prefix.iter().chain(suffix).copied().map(char::from).collect();
            }
        }

        seen.clear();
    }

    unreachable!()
}
