//! # I Was Told There Would Be No Math
//!
//! To extract the numbers when parsing the input we use our utility [`iter_unsigned`] and [`chunk`]
//! functions.
//!
//! Sorting the dimensions in ascending order makes calculating the smallest side or smallest
//! perimeter straightforward.
//!
//! [`iter_unsigned`]: crate::util::parse
//! [`chunk`]: crate::util::iter
use crate::util::integer::*;
use crate::util::iter::*;
use crate::util::parse::*;

type Input = Vec<[u32; 3]>;

pub fn parse(input: &str) -> Input {
    input
        .iter_unsigned()
        .chunk::<3>()
        .map(|[a, b, c]: [u32; 3]| {
            // We only care which element is largest; it is faster to partially sort ourselves
            // than to use sort_unstable.
            let (a, b) = a.minmax(b);
            let (b, c) = b.minmax(c);
            [a, b, c]
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    input.iter().map(|&[l, w, h]| 2 * (l * w + w * h + h * l) + l * w).sum()
}

pub fn part2(input: &Input) -> u32 {
    input.iter().map(|&[l, w, h]| 2 * (l + w) + l * w * h).sum()
}
