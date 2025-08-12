//! # No Matter How You Slice It
//!
//! Brute force approach using bitmasks for efficiency. Assumes that no claim is wider than 65
//! inches.
use crate::util::iter::*;
use crate::util::parse::*;

type Input = (u32, usize);

/// Each square inch of fabric is stored in a single bit.
/// The fabric is 1000 inches wide requiring sixteen `u64`.
const WIDTH: usize = 16;
const HEIGHT: usize = 1000;

pub fn parse(input: &str) -> Input {
    let claims: Vec<_> = input
        .iter_unsigned::<usize>()
        .chunk::<5>()
        .map(|[_, x1, y1, width, height]| {
            let start = WIDTH * y1 + (x1 / 64);
            let end = start + WIDTH * height;

            // Create bitmask for each claim, for example `#123 @ 3,2: 5x4` becomes `11111000`.
            // Use an intermediate u128 to handle claims up to 65 inches wide.
            let mask: u128 = ((1 << width) - 1) << (x1 % 64);
            let lower = mask as u64;
            let upper = (mask >> 64) as u64;

            (start, end, lower, upper)
        })
        .collect();

    let mut fabric = vec![0; WIDTH * HEIGHT];
    let mut overlap = vec![0; WIDTH * HEIGHT];

    for &(start, end, lower, upper) in &claims {
        for index in (start..end).step_by(WIDTH) {
            overlap[index] |= fabric[index] & lower;
            fabric[index] |= lower;

            if upper > 0 {
                overlap[index + 1] |= fabric[index + 1] & upper;
                fabric[index + 1] |= upper;
            }
        }
    }

    // Count the area of overlapping claims.
    let part_one = overlap.iter().map(|n| n.count_ones()).sum();

    // Find the first claim that doesn't overlap with any other claim.
    let part_two = claims
        .iter()
        .position(|&(start, end, lower, upper)| {
            (start..end).step_by(WIDTH).all(|index| {
                (overlap[index] & lower == 0) && (upper == 0 || overlap[index + 1] & upper == 0)
            })
        })
        .map(|id| id + 1)
        .unwrap();

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}
