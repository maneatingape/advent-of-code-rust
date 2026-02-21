//! # A Maze of Twisty Trampolines, All Alike
//!
//! Part one brute forces the jumps. For part two, we can make an observation that the jump offsets
//! will eventually flip-flop between 2 or 3 starting from the beginning, for example:
//!
//! ```none
//!     2 3 2 3 -1
//! ```
//!
//! The twos and threes can be represented in binary compact form, using 0 for 2 and 1 for 3:
//!
//! ```none
//!     0101
//! ```
//!
//! We then precompute all possible combinations for blocks of width 16,
//! using this to accelerate part two.
use crate::util::parse::*;
use std::array::from_fn;

pub fn parse(input: &str) -> Vec<i32> {
    input.iter_signed().collect()
}

/// Brute force implementation.
pub fn part1(input: &[i32]) -> usize {
    let mut jump = input.to_vec();
    let mut total = 0;
    let mut index = 0;

    while index < jump.len() {
        let next = index.wrapping_add(jump[index] as usize);
        jump[index] += 1;
        total += 1;
        index = next;
    }

    total
}

#[expect(clippy::needless_range_loop)]
pub fn part2(input: &[i32]) -> usize {
    let mut jump = input.to_vec();
    let mut total = 0;
    let mut index = 0;

    let mut fine = 0;
    let mut coarse = 0;
    let mut compact = Vec::new();

    // Precompute all possible combinations for each binary starting number from 0 to 2^16,
    // starting at any offset from 0..2.
    let cache: Vec<[_; 0x10000]> =
        (0..3).map(|offset| from_fn(|value| compute_block(value, offset))).collect();

    while index < jump.len() {
        if index < coarse {
            if index % 16 >= 3 {
                let j = index / 16;
                let (next, steps, delta) = compute_block(compact[j], index % 16);

                compact[j] = next as usize;
                total += steps as usize;
                index += delta as usize;
            }

            // Index lies within precomputed blocks.
            for j in (index / 16)..(coarse / 16) {
                let value = compact[j];
                let (next, steps, delta) = cache[index % 16][value];

                compact[j] = next as usize;
                total += steps as usize;
                index += delta as usize;
            }
        } else {
            // Fall back to part one approach.
            let next = index.wrapping_add(jump[index] as usize);
            jump[index] += if jump[index] == 3 { -1 } else { 1 };
            total += 1;

            // The frontier of twos and threes advances through the jump offsets.
            // Each time it crosses a block of 16 add to the compact binary representation.
            if jump[index] == 2 && index == fine {
                fine += 1;
                if fine.is_multiple_of(16) {
                    let value = (coarse..fine).rev().fold(0, |acc, i| (acc << 1) | (jump[i] & 1));
                    coarse = fine;
                    compact.push(value as usize);
                }
            }

            index = next;
        }
    }

    total
}

#[inline]
fn compute_block(mut value: usize, mut offset: usize) -> (u16, u8, u8) {
    let start = offset;
    let mut steps = 0;

    while offset < 16 {
        value ^= 1 << offset;
        steps += 1;
        offset += 3 - ((value >> offset) & 1);
    }

    (value as u16, steps, (offset - start) as u8)
}
