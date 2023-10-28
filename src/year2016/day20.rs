//! # Firewall Rules
use crate::util::iter::*;
use crate::util::parse::*;

type Range = [u64; 2];

/// The trick to merge ranges efficiently is to sort by the *starting* index.
pub fn parse(input: &str) -> Vec<Range> {
    let mut ranges: Vec<_> = input.iter_unsigned().chunk::<2>().collect();
    ranges.sort_unstable_by_key(|r| r[0]);
    ranges
}

pub fn part1(input: &[Range]) -> u64 {
    let mut index = 0;

    for &[start, end] in input {
        if index < start {
            return index;
        }
        // Ends are not sorted so only increase.
        index = index.max(end + 1);
    }

    unreachable!()
}

pub fn part2(input: &[Range]) -> u64 {
    let mut index = 0;
    let mut total = 0;

    for &[start, end] in input {
        if index < start {
            total += start - index;
        }
        // Ends are not sorted so only increase.
        index = index.max(end + 1);
    }

    total
}
