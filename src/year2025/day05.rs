//! # Cafeteria
//!
//! We speed things up by first merging ranges. This is possible in `O(n log n)` instead of `O(n²)`
//! complexity by first sorting the ranges in ascending order of their start.
//!
//! Interestingly, part one is harder than part two. We could check every ID against every range,
//! however this is slow. It's much faster instead to first sort IDs in ascending order,
//! then for each range use a [binary search](https://en.wikipedia.org/wiki/Binary_search) to count
//! the number of IDs that it contains. Rust even provides a handy built-in
//! [`binary_search`] method on slices.
//!
//! [`binary_search`]: https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
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

    // Merge ranges together.
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
