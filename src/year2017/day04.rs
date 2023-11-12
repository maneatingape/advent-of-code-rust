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

pub fn part1(input: &Input<'_>) -> u32 {
    let mut result = 0;
    let mut seen = FastSet::new();

    for line in input {
        result += 1;

        for token in line.split_ascii_whitespace() {
            // Insert returns `false` if the value is already in the set.
            if !seen.insert(token.as_bytes()) {
                result -= 1;
                break;
            }
        }

        seen.clear();
    }

    result
}

pub fn part2(input: &Input<'_>) -> u32 {
    let mut result = 0;
    let mut seen = FastSet::new();

    for line in input {
        result += 1;

        for token in line.split_ascii_whitespace() {
            // Calculate the frequency of each letter, as anagrams will have the same values.
            let mut freq = [0_u8; 26];

            for b in token.bytes() {
                freq[(b - b'a') as usize] += 1;
            }

            if !seen.insert(freq) {
                result -= 1;
                break;
            }
        }

        seen.clear();
    }

    result
}
