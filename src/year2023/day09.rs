//! # Mirage Maintenance
//!
//! We can solve this problem using
//! [binomial coefficients](https://en.wikipedia.org/wiki/Binomial_coefficient).
//!
//! For example, consider a sequence of 3 arbitrary values:
//!
//! ```none
//! 1st:  a        b         c
//! 2nd:    b - a     c - b
//! 3rd:      c - 2b + a
//!
//! Part 1: (c - 2b + a) + (c - b) + (c) = a - 3b + 3c
//! Part 2: a - (b - a) + (c - 2b + a) = 3a - 3b + c
//! ```
//!
//! Looking at the coefficient of each value:
//!
//! ```none
//! Part 1: [1, -3, 3]
//! Part 2: [3, -3, 1]
//! ```
//!
//! Doing this for values of a few different lengths:
//!
//! ```none
//! Part 1: [-1, 4, -6, 4]
//! Part 2: [4, -6, 4, -1]
//!
//! Part 1: [1, -5, 10, -10, 5]
//! Part 2: [5, -10, 10, -5, 1]
//!
//! Part 1: [-1, 6, -15, 20, -15, 6]
//! Part 2: [6, -15, 20, -15, 6, -1]
//! ```
//!
//! Let `n` be the number of values and `k` the index of each value. The coefficient for each value
//! is `(n k)` if `k` is even or `-(n k)` if `k` is odd. For part one we then flip the sign of the
//! sum when `n` is odd.
use crate::util::parse::*;

type Input = (i64, i64);

pub fn parse(input: &str) -> Input {
    // Determine how many numbers are on each row. Assume each row has the same amount.
    let first = input.lines().next().unwrap();
    let count = first.iter_signed::<i64>().count();

    // Calculate [Pascal's Triangle](https://en.wikipedia.org/wiki/Pascal%27s_triangle)
    // for the required row, flipping the sign on each second coefficient.
    let n = count as i64;
    let mut coefficient = 1;
    let mut triangle = vec![1];

    for k in 0..n {
        coefficient = coefficient * (k - n) / (k + 1);
        triangle.push(coefficient);
    }

    // Use adjusted binomial coefficients to calculate answers for each row.
    let mut part_one = 0;
    let mut part_two = 0;

    for (k, value) in (0..count).cycle().zip(input.iter_signed::<i64>()) {
        part_one += value * triangle[k];
        part_two += value * triangle[k + 1];
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> i64 {
    input.0.abs()
}

pub fn part2(input: &Input) -> i64 {
    input.1.abs()
}
