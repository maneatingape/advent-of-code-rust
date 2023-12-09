//! # Mirage Maintenance
//!
//! We can solve this problem using
//! [binomial coefficients](https://en.wikipedia.org/wiki/Binomial_coefficient).
//!
//! For example consider an sequence of 3 arbitrary values:
//!
//! ```none
//!    1st:  a        b         c
//!    2nd:    b - a     c - b
//!    3rd:      c - 2b + a
//!
//!    Part 1: (c - 2b + a) + (c - b) + (c) = a - 3b + 3c
//!    Part 2: a - (b - a) + (c - 2b + a) = 3a - 3b + c
//! ```
//!
//! Looking at the coefficient of each value:
//!
//! ```none
//!     Part 1: [1, -3, 3]
//!     Part 2: [3, -3, 1]
//! ```
//!
//! Doing this for values of a few different lengths:
//!
//! ```none
//!    Part 1: [-1, 4, -6, 4]
//!    Part 2: [4, -6, 4, -1]
//!
//!    Part 1: [1, -5, 10, -10, 5]
//!    Part 2: [5, -10, 10, -5, 1]
//!
//!    Part 1: [-1, 6, -15, 20, -15, 6]
//!    Part 2: [6, -15, 20, -15, 6, -1]
//! ```
//!
//! Let `n` be the number of values and `k` the index of each value. The coefficient for each value
//! is `(n k)` if `k` is even or `-(n k)` if `k` is odd. For part one we then flip the sign of the
//! sum when `n` is odd.
use crate::util::parse::*;

type Input = (i64, i64);

pub fn parse(input: &str) -> Input {
    // Determine how many numbers are on each row. Assume each row has the same amount.
    let (prefix, _) = input.split_once('\n').unwrap();
    let row = prefix.iter_signed::<i64>().count();

    // Calculate [Pascal's Triangle](https://en.wikipedia.org/wiki/Pascal%27s_triangle)
    // for the required row.
    let mut triangle = vec![1];

    for i in 0..row {
        let mut next = Vec::with_capacity(i + 2);

        next.push(1);
        next.extend(triangle.windows(2).map(|w| w[0] + w[1]));
        next.push(1);

        triangle = next;
    }

    // Flip sign of each even numbered column.
    triangle.iter_mut().step_by(2).for_each(|c| *c = -*c);

    // Use adjusted binomial coefficients to calculate answers for each row.
    let mut part_one = 0;
    let mut part_two = 0;

    for line in input.lines() {
        for (k, value) in line.iter_signed::<i64>().enumerate() {
            part_one += value * triangle[k];
            part_two += value * triangle[k + 1];
        }
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> i64 {
    // Positive if row size is even, negative is row size is odd. Take absolute value.
    input.0.abs()
}

pub fn part2(input: &Input) -> i64 {
    input.1
}
