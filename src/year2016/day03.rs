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
    count(input.iter())
}

pub fn part2(input: &[u32]) -> usize {
    (0..3).map(|skip| count(input.iter().skip(skip).step_by(3))).sum()
}

fn count<'a, I>(iter: I) -> usize
where
    I: Iterator<Item = &'a u32>,
{
    iter.chunk::<3>().filter(|&[&a, &b, &c]| a + b > c && a + c > b && b + c > a).count()
}
