//! # Gift Shop
//!
//! We solve efficiently by inverting the problem and avoiding slow string manipulation.
//! Instead of checking every possible id within each range for invalid patterns, we generate a list
//! of invalid patterns numerically then check how many overlap the range given by each pair of ids.
//!
//! Let `(n,k)` denote the set of patterns `k` wide in a number `n` digits long, where `k` must
//! divide `n` evenly. For example some `(6,3)` patterns are 123123, 456456 and 789789.
//!
//! ## Part One
//!
//! The input contains ids ranging from 2 to 10 digits, so we check for patterns where `k`
//! is half of `n`, that are `(2, 1)`, `(4, 2)`, `(6, 3)`, `(8, 4)`, and `(10, 5)`.
//!
//! ## Part Two
//!
//! We also consider the remaining patterns where `n` is evenly divisible by `k`. However we need
//! to be careful to avoid double counting. We notice that pattern sets with the same `n` contain
//! other sets when the second `k` is a factor of the first. For example `(8,4)` contains `(8,2)`,
//! as a number such as 23232323 can be split into either two fours or four twos.
//! All sets `(n, k)` contain `(n, 1)`, for example 22222222 could be split into four, two or one.
//!
//! Our high level approach is then:
//! * Count ids in part one
//! * Count the extra ids in part two, subtracting those that overlap.
//!
//! ## Generating ranges
//!
//! Let's use a concrete example of `(9,3)`. This range starts at 100100100 and we can generate
//! the next number in the set by adding 001001001. The step value is given mathematically as
//! `(10ⁿ - 1) / (10ᵏ - 1)`, that is 999999999 / 999 = 1001001. The start value is then `10ᵏ⁻¹`
//! times the step (100 * 1001001 = 100100100) and the end value is `10ᵏ - 1` times the step
//! (999 * 1001001 = 999999999).
//!
//! We then check each range of ids for the minimum and maximum multiple of the step value that it
//! contains, clamping at the start and end values. For example, the id range 50-70 contains
//! 55 and 66 from the set `(2,1)` with a step of 11. A second id range of 100-130 also
//! contains multiples of 11 but these are ignored as they are outside the start and end values.
//!
//! To sum the set values within each range we use the
//! [triangular number](https://en.wikipedia.org/wiki/Triangular_number) formula
//! `(n * (n + 1)) / 2` that sums the numbers from 1 to `n`. For example:
//!
//! * `44 + 55 + 66 + 77`
//! * `(4 * 44) + 11 + 22 + 33`
//! * `(4 * 44) + 11 * (1 + 2 + 3)`,
//! * Replace `1 + 2 + 3` with the formula.
use crate::util::iter::*;
use crate::util::parse::*;

type Range = [u32; 2];
type Pair = [u64; 2];

/// Sets in part one.
const FIRST: [Range; 5] = [[2, 1], [4, 2], [6, 3], [8, 4], [10, 5]];
/// Sets in part two.
const SECOND: [Range; 6] = [[3, 1], [5, 1], [6, 2], [7, 1], [9, 3], [10, 2]];
/// Overlap between sets in part one and part two.
const THIRD: [Range; 2] = [[6, 1], [10, 1]];

pub fn parse(input: &str) -> Vec<Pair> {
    input.iter_unsigned::<u64>().chunk::<2>().collect()
}

pub fn part1(input: &[Pair]) -> u64 {
    sum(&FIRST, input)
}

pub fn part2(input: &[Pair]) -> u64 {
    sum(&FIRST, input) + sum(&SECOND, input) - sum(&THIRD, input)
}

/// Generate the start and end values for a set,
/// then sum the number of values contained in the given id range.
fn sum(ranges: &[Range], input: &[Pair]) -> u64 {
    let mut result = 0;

    for &[digits, size] in ranges {
        // Generate the sequence of invalid digit ids numerically.
        let digits_power = 10_u64.pow(digits);
        let size_power = 10_u64.pow(size);

        let step = (digits_power - 1) / (size_power - 1);
        let start = step * (size_power / 10);
        let end = step * (size_power - 1);

        for &[from, to] in input {
            // Find the first and last multiple of the step size,
            // clamping to the start and end of the set.
            let lower = from.next_multiple_of(step).max(start);
            let upper = to.min(end);

            // Sum invalid ids using triangular number formula.
            if lower <= upper {
                let n = (upper - lower) / step;
                let triangular = n * (n + 1) / 2;
                result += lower * (n + 1) + step * triangular;
            }
        }
    }

    result
}
