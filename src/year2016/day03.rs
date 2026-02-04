//! # Squares With Three Sides
//!
//! We rely on the [`iter`] and [`parse`] utility modules to extract integers from surrounding
//! text then group together in chunks of three.
//!
//! [`iter`]: crate::util::iter
//! [`parse`]: crate::util::parse
use crate::util::integer::*;
use crate::util::iter::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<u32> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[u32]) -> usize {
    count(input.iter().copied())
}

pub fn part2(input: &[u32]) -> usize {
    (0..3).map(|skip| count(input.iter().copied().skip(skip).step_by(3))).sum()
}

fn count(iter: impl Iterator<Item = u32>) -> usize {
    iter.chunk::<3>()
        .filter(|&[a, b, c]| {
            // It is faster to manually sort out the largest element and do one compare
            let (a, b) = a.minmax(b);
            let (b, c) = b.minmax(c);
            a + b > c
        })
        .count()
}
