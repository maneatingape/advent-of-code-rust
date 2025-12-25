//! # High-Entropy Passphrases
//!
//! ## Part One
//!
//! We use a [`FastSet`] to detect duplicates. Sorting the words in each line
//! then checking for duplicates in adjacent values also works but is slower.
//!
//! ## Part Two
//!
//! To detect anagrams we first convert each word into a histogram of its letter frequency values.
//! As the cardinality is at most 26 we can use a fixed size array to represent the set.
//!
//! Then a [`FastSet`] is used to detect duplicates. Sorting the letters in each word so that
//! anagrams become the same also works but is slower.
use crate::util::hash::*;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> usize {
    let mut seen = FastSet::new();
    input
        .iter()
        .filter(|line| {
            seen.clear();
            line.split_ascii_whitespace().all(|token| seen.insert(token))
        })
        .count()
}

pub fn part2(input: &[&str]) -> usize {
    let mut seen = FastSet::new();
    input
        .iter()
        .filter(|line| {
            seen.clear();
            line.split_ascii_whitespace().all(|token| seen.insert(letter_frequency(token)))
        })
        .count()
}

/// Convert a token to its letter frequency histogram.
/// Only 26 elements are needed but 32 is faster to hash.
#[inline]
fn letter_frequency(token: &str) -> [u8; 32] {
    let mut freq = [0; 32];
    for b in token.bytes() {
        freq[(b - b'a') as usize] += 1;
    }
    freq
}
