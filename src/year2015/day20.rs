//! # Infinite Elves and Infinite Houses
//!
//! The amount of presents that each house receives in part one is 10 times the
//! [divisor function](https://en.wikipedia.org/wiki/Divisor_function) `σ`.
//! For example the divisors of 6 are 1, 2, 3 and 6, so house 6 receives
//! 10 + 20 + 30 + 60 = 120 presents.
//!
//! It's highly likely that the answer will be a
//! [superabundant number](https://en.wikipedia.org/wiki/Superabundant_number) but there's no way
//! to easily *prove* that so we brute force check every possible solution. The approach is similar
//! to a reverse [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes),
//! iterating first over each elf, then over each house adding the presents.
//! Somewhat unintuitively the `ln(x)` asymptotic complexity of this approach is much lower
//! than the `n√n` complexity of finding the factors of each number.
//!
//! To speed things up we make one high level optimization and a few low tweaks.
//!
//! The high level optimization is an observation from user `warbaque` on the
//! [Day 20 solution thread](https://www.reddit.com/r/adventofcode/comments/3xjpp2/day_20_solutions/)
//! that [Robin's inequality](https://en.wikipedia.org/wiki/Divisor_function#Growth_rate)
//! shows that the `σ(n)` function is lower than `eᵞ * n * ln(ln(n))` for all `n` greater than 5040.
//! This means that we can determine a starting index close to the final result, skipping over a
//! huge chunk of numbers.
//!
//! The low level tweaks reduce the amount of work that needs to be done.
//! We break the search range into fixed size blocks from `start` to `end`,
//! for example 200000 to 299999 with a block size of 100000. Then we can make some observations:
//!
//! * Elf 1 visits each house once.
//! * Elves from `start` to `end` each visit a different house exactly once,
//!   bringing start, start + 1, ... presents.
//! * Elves from `end / 2` to `start` skip over all the houses.
//! * Elves from `block size` to `end / 2` visit *at most* one house as the increment is
//!   greater than the size of the block.
//! * Elves from `2` to `block size` may visit any number of times.

// More explicit syntax fits in with surrounding code better.
#![allow(clippy::needless_range_loop)]

use crate::util::parse::*;

const BLOCK: usize = 100_000;

type Input = (u32, usize);

pub fn parse(input: &str) -> Input {
    let robins_inequality = [
        (100000, 4352000),
        (200000, 8912250),
        (300000, 13542990),
        (400000, 18218000),
        (500000, 22925240),
        (600000, 27657740),
        (700000, 32410980),
        (800000, 37181790),
        (900000, 41967820),
        (1000000, 46767260),
        (1100000, 51578680),
        (1200000, 56400920),
        (1300000, 61233020),
        (1400000, 66074170),
        (1500000, 70923680),
        (1600000, 75780960),
        (1700000, 80645490),
        (1800000, 85516820),
        (1900000, 90394550),
        (2000000, 95278320),
    ];

    // Find a starting block closer to the answer. Part two presents overall are lower than
    // part one so the answer will also be after this block.
    let (target, mut start) = (input.unsigned(), 0);

    for (key, value) in robins_inequality {
        if target >= value {
            start = key;
        } else {
            break;
        }
    }

    assert!(target > 5040);
    assert!(start > 100000);

    (target, start)
}

pub fn part1(input: &Input) -> usize {
    let (target, mut start) = *input;
    let mut end = start + BLOCK;
    let mut houses = vec![0; BLOCK];

    loop {
        // Elves that visit exactly once.
        for i in 0..BLOCK {
            houses[i] = 10 * (1 + start + i) as u32;
        }

        // Elves that visit at most once.
        for i in BLOCK..end / 2 {
            let presents = 10 * i as u32;
            let j = next_multiple_of(start, i) - start;

            if j < BLOCK {
                houses[j] += presents;
            }
        }

        // All remaining elves.
        for i in 2..BLOCK {
            let presents = 10 * i as u32;
            let mut j = next_multiple_of(start, i) - start;

            while j < BLOCK {
                houses[j] += presents;
                j += i;
            }
        }

        if let Some(found) = houses.iter().position(|&p| p >= target) {
            break start + found;
        }

        start += BLOCK;
        end += BLOCK;
    }
}

pub fn part2(input: &Input) -> usize {
    let (target, mut start) = *input;
    let mut end = start + BLOCK;
    let mut houses = vec![0; BLOCK];

    loop {
        // Elves that visit exactly once (not including elf 1 anymore).
        for i in 0..BLOCK {
            houses[i] = 11 * (start + i) as u32;
        }

        // Elves that visit at most once.
        for i in BLOCK..end / 2 {
            let presents = 11 * i as u32;
            let j = next_multiple_of(start, i) - start;

            if j < BLOCK {
                houses[j] += presents;
            }
        }

        // All remaining elves. We can start higher than 2.
        for i in start / 50..BLOCK {
            let presents = 11 * i as u32;
            let mut j = next_multiple_of(start, i) - start;
            let mut remaining = 51 - div_ceil(start, i);

            while j < BLOCK && remaining > 0 {
                houses[j] += presents;
                j += i;
                remaining -= 1;
            }
        }

        if let Some(found) = houses.iter().position(|&p| p >= target) {
            break start + found;
        }

        start += BLOCK;
        end += BLOCK;
    }
}

#[inline]
fn div_ceil(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

#[inline]
fn next_multiple_of(a: usize, b: usize) -> usize {
    div_ceil(a, b) * b
}
