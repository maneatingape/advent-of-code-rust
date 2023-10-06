//! # It Hangs in the Balance
//!
//! To simplify things assumes that the remaining items after the first best combination is found
//! can be split evenly.
//!
//! Sorts the weights in ascending order, then tries combinations of increasing size until a
//! match in found. This will be the answer since the package count is the smallest and the
//! quantum entaglement will also be the lowest.
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<u64> {
    let mut packages: Vec<_> = input.iter_unsigned().collect();
    packages.sort_unstable();
    packages
}

pub fn part1(input: &[u64]) -> u64 {
    let sum: u64 = input.iter().sum();
    let target = sum / 3;
    (1..input.len()).find_map(|size| combinations(input, target, size)).unwrap()
}

pub fn part2(input: &[u64]) -> u64 {
    let sum: u64 = input.iter().sum();
    let target = sum / 4;
    (1..input.len()).find_map(|size| combinations(input, target, size)).unwrap()
}

/// Check all combinations of `size` items returning `None` if no valid solution is found.
fn combinations(packages: &[u64], target: u64, size: usize) -> Option<u64> {
    // Mantain `size` indices, initially set to 0, 1, 2...
    let mut indices: Vec<_> = (0..size).collect();
    // Initial weight for first `size` items.
    let mut weight: u64 = packages.iter().take(size).sum();

    loop {
        // Check for success
        if weight == target {
            let product = indices.iter().map(|&i| packages[i]).product();
            return Some(product);
        }

        // Try to advance the last index. If the last index is at the end, then try to advance
        // the previous index until we reach the root.
        let mut depth = size - 1;
        while indices[depth] == packages.len() - size + depth {
            if depth == 0 {
                return None;
            }
            depth -= 1;
        }

        // Update the first index that is not at the end.
        let from = indices[depth];
        let to = indices[depth] + 1;
        indices[depth] = to;
        weight = weight - packages[from] + packages[to];
        depth += 1;

        // "Wrap" following indices to 1 more than the previous.
        while depth < size {
            let from = indices[depth];
            let to = indices[depth - 1] + 1;
            indices[depth] = to;
            weight = weight - packages[from] + packages[to];
            depth += 1;
        }
    }
}
