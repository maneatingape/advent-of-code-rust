//! # Cafeteria
//!
//! We speed things up by first merging ranges. This is possible in `O(n log n)` instead of `O(n²)`
//! complexity by first sorting the ranges in ascending order of their start.
//!
//! Interestingly, part one is harder than part two. We could check every ID against every range,
//! however this is slow. It's much faster to sort IDs in ascending order,
//! just like the ranges, so that a single O(n) pass can check all of them
//!
use crate::util::iter::*;
use crate::util::parse::*;
// use std::ops::Range;

type Input = (usize, u64);

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<_> = prefix.iter_unsigned().chunk::<2>().collect();
    let mut ids: Vec<u64> = suffix.iter_unsigned().collect();
    let mut merged = Vec::new();
    let mut part2 = 0;
    let mut part1 = 0;

    ranges.sort_unstable();
    ranges.push([u64::MAX,u64::MAX]);  // Guard!

    let mut start = ranges[0][0];
    let mut end = ranges[0][1]+1;

    // Merge ranges together.
    for r in 1..ranges.len() {
        if ranges[r][0] < end {
            end = end.max(ranges[r][1] + 1);
        } else {
            part2 += end-start;
            merged.push([start,end]);
            start = ranges[r][0];
            end = ranges[r][1];
        }
    }
    merged.push([start,end]); // This is the zero length guard range greater than any id

    ids.sort_unstable();
    let mut r = 0;
    for ii in ids {
        while ii >= merged[r][1] {r += 1;}
        if ii >= merged[r][0] { part1 += 1;}
    }
    (part1, part2)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}
