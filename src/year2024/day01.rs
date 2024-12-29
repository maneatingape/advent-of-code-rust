//! # Historian Hysteria
//!
//! For part 2, the time needed to allocate memory and grow the map is a large percentage
//! of the total. Creating the [`FastMap`] with capacity 1000 reduces this.
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;

type Input = (Vec<u32>, Vec<u32>);

pub fn parse(input: &str) -> Input {
    input.iter_unsigned::<u32>().chunk::<2>().map(|[l, r]| (l, r)).unzip()
}

pub fn part1(input: &Input) -> u32 {
    let (mut left, mut right) = input.clone();

    left.sort_unstable();
    right.sort_unstable();

    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

pub fn part2(input: &Input) -> u32 {
    let (left, right) = input;

    let mut freq = FastMap::with_capacity(1_000);
    right.iter().for_each(|r| *freq.entry(r).or_insert(0) += 1);

    left.iter().filter_map(|l| freq.get(l).map(|f| l * f)).sum()
}
