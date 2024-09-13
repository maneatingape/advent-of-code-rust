//! # A Maze of Twisty Trampolines, All Alike
//!
//! Part one brute forces the jumps. For part two we can make an observation that the jumps offsets
//! will eventually flip flop between 2 or 3 starting from the beginning, for example:
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
//! We then precompute all possible combination for blocks of size 16, using this to accelerate
//! part two.
use crate::util::parse::*;

const WIDTH: usize = 16;
const LENGTH: usize = 1 << WIDTH;

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
    let mut cache = vec![[(0_u16, 0_u8, 0_u8); LENGTH]; WIDTH];

    // Precompute all possible combinations. For each binary starting number we can start at any
    // offset from 0..16.
    for i in 0..WIDTH {
        for j in 0..LENGTH {
            let mut offset = i as u16;
            let mut value = j as u16;
            let mut steps = 0;

            while offset < 16 {
                value ^= 1 << offset;
                steps += 1;
                offset += 3 - ((value >> offset) & 1);
            }

            cache[i][j] = (value, steps, offset as u8 - i as u8);
        }
    }

    while index < jump.len() {
        if index < coarse {
            // Index lies withing precomputed blocks.
            let base = index / 16;
            let offset = index % 16;
            let value = compact[base] as usize;
            let (next, steps, delta) = cache[offset][value];

            compact[base] = next;
            total += steps as usize;
            index += delta as usize;
        } else {
            // Fall back to part one approach.
            let next = index.wrapping_add(jump[index] as usize);
            jump[index] += if jump[index] == 3 { -1 } else { 1 };
            total += 1;

            // The frontier of twos and threes advances through the jump offsets.
            // Each time it crosses a block of 16 add to the compact binary representation.
            if jump[index] == 2 && index == fine {
                fine += 1;
                if fine % 16 == 0 {
                    let value = (coarse..fine).rev().fold(0, |acc, i| (acc << 1) | (jump[i] & 1));
                    coarse = fine;
                    compact.push(value as u16);
                }
            }

            index = next;
        }
    }

    total
}
