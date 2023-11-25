//! # Knot Hash
//!
//! Instead of reversing elements from the starting position then trying to handle wrap around,
//! its easier use [`rotate_left`] to rotate the array by the same amount so that the starting
//! position is always zero, then take advantage of the built in [`reverse`] method.
//!
//! [`rotate_left`]: slice::rotate_left
//! [`reverse`]: slice::reverse
use crate::util::parse::*;
use std::fmt::Write;

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> u32 {
    let lengths: Vec<_> = input.iter_unsigned().collect();
    let knot = hash(&lengths, 1);
    knot.iter().take(2).map(|&b| b as u32).product()
}

pub fn part2(input: &str) -> String {
    let mut lengths: Vec<_> = input.trim().bytes().map(|b| b as usize).collect();
    lengths.extend([17, 31, 73, 47, 23]);

    let knot = hash(&lengths, 64);
    let mut result = String::new();

    for chunk in knot.chunks_exact(16) {
        let reduced = chunk.iter().fold(0, |acc, n| acc ^ n);
        let _ = write!(&mut result, "{reduced:02x}");
    }

    result
}

fn hash(lengths: &[usize], rounds: usize) -> Vec<u8> {
    let mut knot: Vec<_> = (0..=255).collect();
    let mut position = 0;
    let mut skip = 0;

    for _ in 0..rounds {
        for &length in lengths {
            let next = length + skip;
            knot[0..length].reverse();
            knot.rotate_left(next % 256);
            position += next;
            skip += 1;
        }
    }

    // Rotate the array the other direction so that the original starting position is restored.
    knot.rotate_right(position % 256);
    knot
}
