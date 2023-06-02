//! # Report Repair
//!
//! The straightforward approach is to compare every possible pair of elements for part 1 and
//! every possible triple for part 2. This would have `O(n²)` and `O(n³)`time complexity respectively.
//!
//! We can do better with `O(n)` complexity for part 1 and `O(n²)` for part 2, with an shared
//! upfront cost of `O(logn)`.
//!
//! First sort the slice in ascending order. Then maintain two pointers starting at the beginning
//! and end of the slice. If the sum if greater then decrement the end pointer. If the sum is less
//! decrement the end pointer. If equal then return the product of the pair.
//!
//! Part 2 reuses the pair finding logic, finding the third element by stepping through the slice
//! one by one and adjusting the target total.
use crate::util::parse::*;
use std::cmp::Ordering::*;

pub fn parse(input: &str) -> Vec<u32> {
    let mut expenses: Vec<_> = input.lines().map(from).collect();
    expenses.sort_unstable();
    expenses
}

pub fn part1(input: &[u32]) -> u32 {
    two_sum(input, 2020).unwrap()
}

pub fn part2(input: &[u32]) -> u32 {
    for i in 0..(input.len() - 2) {
        if let Some(product) = two_sum(&input[(i + 1)..], 2020 - input[i]) {
            return input[i] * product;
        }
    }
    unreachable!()
}

fn two_sum(slice: &[u32], target: u32) -> Option<u32> {
    let mut start = 0;
    let mut end = slice.len() - 1;

    while start < end {
        let sum = slice[start] + slice[end];
        match sum.cmp(&target) {
            Less => start += 1,
            Greater => end -= 1,
            Equal => return Some(slice[start] * slice[end]),
        }
    }

    None
}
