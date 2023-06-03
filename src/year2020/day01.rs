//! # Report Repair
//!
//! The straightforward approach is to compare every possible pair of elements for part one and
//! every possible triple for part two. This would have `O(n²)` and `O(n³)`time complexity respectively.
//!
//! We can do better with `O(n)` complexity for part one and `O(n²)` for part two.
//!
//! For part one we use an implicit hash table in an array, since values are constrained to between
//! 0 and 2020 and each value is already perfectly hashed. For each entry we check the index
//! at its value. If this is marked then we have seen the reciprocal `2020 - value` before
//! so we can return the product. If not then we mark the reciprocal value in the array.
//!
//! Part 2 reuses the pair finding logic, finding the third element by stepping through the slice
//! one by one and adjusting the target total. To reuse the array without reallocating
//! (which is slow) we use a `round` value instead of `bool`.
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<usize> {
    input.lines().map(from).collect()
}

pub fn part1(input: &[usize]) -> usize {
    let mut hash = [0; 2020];
    two_sum(input, 2020, &mut hash, 1).unwrap()
}

pub fn part2(input: &[usize]) -> usize {
    let mut hash = [0; 2020];

    for i in 0..(input.len() - 2) {
        let first = input[i];
        let round = i + 1;
        let slice = &input[round..];
        let target = 2020 - first;

        if let Some(product) = two_sum(slice, target, &mut hash, round) {
            return first * product;
        }
    }
    unreachable!()
}

fn two_sum(slice: &[usize], target: usize, hash: &mut [usize], round: usize) -> Option<usize> {
    for &i in slice {
        if i < target {
            if hash[i] == round {
                return Some(i * (target - i));
            } else {
                hash[target - i] = round;
            }
        }
    }

    None
}
