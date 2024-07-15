//! # Inventory Management System
use crate::util::hash::*;

pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

pub fn part1(input: &[&[u8]]) -> u32 {
    let mut total_twos = 0;
    let mut total_threes = 0;

    for &id in input {
        // Ids are lowercase ASCII only with cardinality of 26.
        let mut freq = [0; 26];
        let mut twos = 0;
        let mut threes = 0;

        for &b in id {
            let index = (b - b'a') as usize;
            let current = freq[index];

            match current {
                0 => (),
                1 => twos += 1,
                2 => {
                    twos -= 1;
                    threes += 1;
                }
                _ => threes -= 1,
            }

            freq[index] += 1;
        }

        if twos > 0 {
            total_twos += 1;
        }
        if threes > 0 {
            total_threes += 1;
        }
    }

    total_twos * total_threes
}

pub fn part2(input: &[&[u8]]) -> String {
    let width = input[0].len();

    let mut seen = FastSet::with_capacity(input.len());
    let mut buffer = [0; 32];

    // Use a set to check for duplicates after replacing a single character with '*' in each column.
    for column in 0..width {
        for &id in input {
            buffer[0..width].copy_from_slice(id);
            buffer[column] = b'*';

            if !seen.insert(buffer) {
                // Convert to String
                return buffer
                    .iter()
                    .filter(|&&b| b.is_ascii_lowercase())
                    .map(|&b| b as char)
                    .collect();
            }
        }

        seen.clear();
    }

    unreachable!()
}
