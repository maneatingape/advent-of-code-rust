//! # Christmas Tree Farm
use crate::util::iter::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> usize {
    input
        .iter_unsigned::<u32>()
        .skip(6)
        .chunk::<8>()
        .filter(|[w, h, presents @ ..]| (w / 3) * (h / 3) >= presents.iter().sum::<u32>())
        .count()
}

pub fn part2(_input: &str) -> &'static str {
    "n/a"
}
