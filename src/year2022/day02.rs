//! # Rock Paper Scissors
//!
//! With so few combinations it's possible to precompute the values for each scenario by hand
//! then quickly look them up for each game.
use crate::util::iter::*;

/// Map each line from one of the 9 possible combinations ("A", "B" or "C" followed by "X", "Y" or "Z")
/// to between 0 and 8 inclusive.
///
/// Notes:
/// * [`chunk`] is a convenience extension method to [`Iterator`] that groups the iterator's
/// elements into arrays of a fixed size.
///
/// [`chunk`]: ChunkOps::chunk
/// [`Iterator`]: std::iter::Iterator
pub fn parse(input: &str) -> Vec<usize> {
    input
        .as_bytes()
        .split(u8::is_ascii_whitespace)
        .chunk::<2>()
        .map(|[a, b]| (3 * (a[0] - b'A') + b[0] - b'X') as usize)
        .collect()
}

/// Map each index to a score using a small precomputed lookup table.
pub fn part1(input: &[usize]) -> u32 {
    let score = [4, 8, 3, 1, 5, 9, 7, 2, 6];
    input.iter().map(|&i| score[i]).sum()
}

/// Map each index to a (different) score using a second small precomputed lookup table.
pub fn part2(input: &[usize]) -> u32 {
    let score = [3, 4, 8, 1, 5, 9, 2, 6, 7];
    input.iter().map(|&i| score[i]).sum()
}
