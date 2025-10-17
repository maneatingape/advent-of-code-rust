//! # Calorie Counting
//! Sums groups of numbers separated by blank lines into a `vec` sorted in ascending order.
//!
//! Since we don't care what order the highest values are returned in [`select_nth_unstable`] would
//! also work, and in theory is a little faster, however the difference was negligible when benchmarking.
//!
//! [`select_nth_unstable`]: slice::select_nth_unstable
use crate::util::parse::*;

/// Parse and group lines.
pub fn parse(input: &str) -> Vec<u32> {
    let mut elves: Vec<u32> = input.split("\n\n").map(|s| s.iter_unsigned::<u32>().sum()).collect();
    elves.sort_unstable();
    elves
}

/// Use a reverse iterator to find the elf with the most calories.
pub fn part1(input: &[u32]) -> u32 {
    input.iter().rev().take(1).sum()
}

/// Use a reverse iterator to sum the calories of the 3 highest elves.
pub fn part2(input: &[u32]) -> u32 {
    input.iter().rev().take(3).sum()
}
