//! # Rock Paper Scissors
//!
//! With so few combinations it's possible to precompute the values for each scenario by hand
//! then quickly look them up for each game.

/// Map each line from one of the 9 possible combinations ("A", "B" or "C" followed by "X", "Y" or "Z")
/// to between 0 and 8 inclusive.
pub fn parse(input: &str) -> Vec<usize> {
    input.as_bytes().chunks_exact(4).map(|c| (3 * (c[0] - b'A') + c[2] - b'X') as usize).collect()
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
