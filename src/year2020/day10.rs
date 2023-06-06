//! # Adapter Array
//!
//! Part one uses the [`windows`] function to iterate over all pairs counting the differences.
//!
//! Part two is a classic [dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming)
//! problem. The charging outlet can be arranged in exactly one way. Each subsequent adapter can be
//! arranged in the sum of the number of ways that the adapters at -1, -2 and -3 jolts can be
//! arranged.
//!
//! [`windows`]: slice::windows
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<usize> {
    let mut adapters: Vec<_> = input.iter_unsigned().collect();
    adapters.sort_unstable();
    adapters
}

pub fn part1(input: &[usize]) -> usize {
    let mut total = [0, 0, 0, 1];
    total[input[0]] += 1;

    for w in input.windows(2) {
        let diff = w[0].abs_diff(w[1]);
        total[diff] += 1;
    }

    total[1] * total[3]
}

pub fn part2(input: &[usize]) -> usize {
    let last = input.last().unwrap();
    let mut sum = vec![0; last + 1];
    sum[0] = 1;

    for &i in input {
        match i {
            1 => sum[i] = sum[i - 1],
            2 => sum[i] = sum[i - 1] + sum[i - 2],
            _ => sum[i] = sum[i - 1] + sum[i - 2] + sum[i - 3],
        }
    }

    *sum.last().unwrap()
}
