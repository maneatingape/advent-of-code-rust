//! # Inventory Management System
use crate::util::hash::*;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> u32 {
    let mut twos = 0;
    let mut threes = 0;

    for &id in input {
        // Ids are lowercase ASCII only with cardinality of 26.
        let mut freq = [0; 26];

        for b in id.bytes() {
            freq[(b - b'a') as usize] += 1;
        }

        twos += freq.contains(&2) as u32;
        threes += freq.contains(&3) as u32;
    }

    twos * threes
}

pub fn part2(input: &[&str]) -> String {
    let width = input[0].len();
    let mut seen = FastSet::with_capacity(input.len());

    // Use a set to check for duplicates by comparing the prefix and suffix of IDs excluding one
    // column at a time.
    for column in 0..width {
        for &id in input {
            let pair @ (prefix, suffix) = (&id[..column], &id[column + 1..]);
            if !seen.insert(pair) {
                return format!("{prefix}{suffix}");
            }
        }
        seen.clear();
    }

    unreachable!()
}
