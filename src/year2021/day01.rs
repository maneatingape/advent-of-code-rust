//! # Sonar Sweep
//!
//! The built-in [`windows`] method comes in handy for this solution. For part one a straightforward
//! sliding window of size 2 allows us to compare each 2 consecutive values.
//!
//! For part two we can use a trick to simplify. If we consider the first 2 windows of 3 elements
//! each:
//!
//! ```none
//!   A1 A2 A3
//!      B1 B2 B3
//! ```
//!
//! then the middle 2 elements are always in common, so the subsequent window is greater only
//! if the last element is greater than the first. This means we can pick a sliding window of
//! size 4 and compare the first and last elements, without having to sum intermediate elements.
//!
//! [`windows`]: slice::windows
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<u32> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[u32]) -> usize {
    input.windows(2).filter(|w| w[0] < w[1]).count()
}

pub fn part2(input: &[u32]) -> usize {
    input.windows(4).filter(|w| w[0] < w[3]).count()
}
