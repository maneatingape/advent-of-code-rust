//! # Cafeteria
//!
//! We speed things up by first merging ranges. This is possible in `O(n log n)` instead of `O(n²)`
//! complexity by first sorting the ranges in ascending order of their start.
//!
//! Interestingly, part one is harder than part two. We could check every ID against every range,
//! however this is slow. It's much faster instead to first sort IDs in ascending order,
//! then for each range use a [binary search](https://en.wikipedia.org/wiki/Binary_search) to count
//! the number of IDs that it contains. Rust even provides a handy built-in
//! [`partition_point`] method on slices that returns the index of the first ID not less than
//! a given value.
//!
//! [`partition_point`]: https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point
use std::ops::Range;

use crate::util::iter::*;
use crate::util::parse::*;

type Input = (Vec<Range<u64>>, Vec<u64>);

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<Range<u64>> =
        prefix.iter_unsigned().chunk::<2>().map(|[from, to]| from..to + 1).collect();
    let mut ids: Vec<_> = suffix.iter_unsigned().collect();

    ranges.sort_unstable_by_key(|range| range.start);
    ids.sort_unstable();

    // Merge ranges together.
    ranges.dedup_by(|next, range| {
        if next.start < range.end {
            range.end = range.end.max(next.end);
            true
        } else {
            false
        }
    });

    (ranges, ids)
}

pub fn part1(input: &Input) -> usize {
    let (merged, ids) = input;
    let position = |id| ids.partition_point(|&next| next < id);
    merged.iter().map(|range| position(range.end) - position(range.start)).sum()
}

pub fn part2(input: &Input) -> u64 {
    let (merged, _) = input;
    merged.iter().map(|range| range.end - range.start).sum()
}
