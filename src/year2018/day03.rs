//! # No Matter How You Slice It
//!
//! Brute force approach using bitmasks for efficiency. Assumes that no claim is wider than 65
//! inches.
use crate::util::iter::*;
use crate::util::parse::*;

type Input = (u32, usize);

pub fn parse(input: &str) -> Input {
    let claims: Vec<_> = input
        .iter_unsigned::<usize>()
        .chunk::<5>()
        .map(|[_, x1, y1, width, height]| {
            let start = 16 * y1 + (x1 / 64);
            let end = start + 16 * height;

            // Create bitmask for each claim, for example `#123 @ 3,2: 5x4` becomes `11111000`.
            // Use an intermediate u128 to handle claims up to 65 inches wide.
            let mask: u128 = ((1 << width) - 1) << (x1 % 64);
            let lower = mask as u64;
            let upper = (mask >> 64) as u64;

            (start, end, lower, upper)
        })
        .collect();

    // Each square inch of fabric is stored in a single bit.
    // The fabric is 1000 inches wide requiring sixteen `u64`.
    let mut fabric = vec![0; 16 * 1000];
    let mut overlap = vec![0; 16 * 1000];

    for &(start, end, lower, upper) in &claims {
        for index in (start..end).step_by(16) {
            overlap[index] |= fabric[index] & lower;
            fabric[index] |= lower;

            if upper > 0 {
                overlap[index + 1] |= fabric[index + 1] & upper;
                fabric[index + 1] |= upper;
            }
        }
    }

    // Find the first claim that doesn't overlap with any other claim.
    let position = claims.iter().position(|&(start, end, lower, upper)| {
        (start..end).step_by(16).all(|index| {
            (overlap[index] & lower == 0) && (upper == 0 || overlap[index + 1] & upper == 0)
        })
    });

    let part_one = overlap.iter().map(|n| n.count_ones()).sum();
    let part_two = position.unwrap() + 1;
    (part_one, part_two)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}
