//! # Cafeteria
use crate::util::iter::*;
use crate::util::parse::*;
use std::ops::Range;

type Input = (Vec<Range<u64>>, Vec<u64>);

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<_> = prefix.iter_unsigned().chunk::<2>().collect();
    let mut ids: Vec<_> = suffix.iter_unsigned().collect();
    let mut range = 0..0;
    let mut merged = Vec::new();

    ranges.sort_unstable();
    ids.sort_unstable();

    for [from, to] in ranges {
        if from < range.end {
            range.end = range.end.max(to + 1);
        } else {
            merged.push(range);
            range = from..to + 1;
        }
    }

    merged.push(range);
    (merged, ids)
}

pub fn part1(input: &Input) -> usize {
    let (merged, ids) = input;
    let position = |id: u64| ids.binary_search(&id).unwrap_or_else(|e| e);
    merged.iter().map(|range| position(range.end) - position(range.start)).sum()
}

pub fn part2(input: &Input) -> u64 {
    let (merged, _) = input;
    merged.iter().map(|range| range.end - range.start).sum()
}
