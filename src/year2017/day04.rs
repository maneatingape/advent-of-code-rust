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

type Input<'a> = Vec<&'a str>;

pub fn parse(input: &str) -> Input<'_> {
    input.lines().collect()
}

pub fn part1(input: &Input<'_>) -> usize {
    let mut seen = FastSet::new();
    input
        .iter()
        .filter(|line| {
            seen.clear();
            line.split_ascii_whitespace().all(|token| seen.insert(token.as_bytes()))
        })
        .count()
}

pub fn part2(input: &Input<'_>) -> usize {
    // Only 26 elements are needed but 32 is faster to hash.
    fn convert(token: &str) -> [u8; 32] {
        let mut freq = [0; 32];
        for b in token.bytes() {
            freq[(b - b'a') as usize] += 1;
        }
        freq
    }

    let mut seen = FastSet::new();
    input
        .iter()
        .filter(|line| {
            seen.clear();
            line.split_ascii_whitespace().all(|token| seen.insert(convert(token)))
        })
        .count()
}
