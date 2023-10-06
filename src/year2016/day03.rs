//! # Squares With Three Sides
//!
//! We rely on the [`iter`] and [`parse`] utility modules to extract integers from surrounding
//! text then group together in chunks of three.
//!
//! [`iter`]: crate::util::iter
//! [`parse`]: crate::util::parse
use crate::util::iter::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<u32> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[u32]) -> usize {
    count(input.iter().copied())
}

pub fn part2(input: &[u32]) -> usize {
    let first = count(input.iter().copied().step_by(3));
    let second = count(input.iter().copied().skip(1).step_by(3));
    let third = count(input.iter().copied().skip(2).step_by(3));
    first + second + third
}

fn count(iter: impl Iterator<Item = u32>) -> usize {
    iter.chunk::<3>().filter(|&[a, b, c]| a + b > c && a + c > b && b + c > a).count()
}
