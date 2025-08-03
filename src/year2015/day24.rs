//! # It Hangs in the Balance
//!
//! To simplify things assumes that the remaining items after the first best combination is found
//! can be split evenly.
//!
//! Sorts the weights in ascending order, then tries combinations of increasing size until a
//! match in found. This will be the answer since the package count is the smallest and the
//! quantum entaglement will also be the lowest.
use crate::util::bitset::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<u32> {
    let mut packages: Vec<_> = input.iter_unsigned().collect();
    packages.sort_unstable();
    packages
}

pub fn part1(input: &[u32]) -> u64 {
    combinations(input, 3)
}

pub fn part2(input: &[u32]) -> u64 {
    combinations(input, 4)
}

/// Breadth first search over all possible package combinations as we want the fewest possible
/// packages that sum to the target weight. Uses a bitmask as a set for each package combination.
fn combinations(input: &[u32], groups: u32) -> u64 {
    let target = input.iter().sum::<u32>() / groups;
    let mut current = &mut Vec::with_capacity(100_000);
    let mut next = &mut Vec::with_capacity(100_000);

    // Start with no packages.
    current.push((0_u32, 0_u32));

    loop {
        for (weight, packages) in current.drain(..) {
            // Find the next highest power of two.
            let start = 32 - packages.leading_zeros() as usize;

            // Add one package at a time to this combination.
            for i in start..input.len() {
                let next_weight = weight + input[i];
                let next_packages = packages | (1 << i);

                if next_weight == target {
                    return next_packages.biterator().map(|i| input[i] as u64).product();
                }
                if next_weight > target {
                    break;
                }

                next.push((next_weight, next_packages));
            }
        }

        (current, next) = (next, current);
    }
}
