use crate::util::chunk::*;
use crate::util::parse::*;

type Gift = [u32; 3];

pub fn parse(input: &str) -> Vec<Gift> {
    input
        .iter_unsigned()
        .chunk::<3>()
        .map(|chunk| {
            let mut gift = chunk;
            gift.sort_unstable();
            gift
        })
        .collect()
}

pub fn part1(input: &[Gift]) -> u32 {
    input
        .iter()
        .map(|[l, w, h]| 2 * (l * w + w * h + h * l) + l * w)
        .sum()
}

pub fn part2(input: &[Gift]) -> u32 {
    input
        .iter()
        .map(|[l, w, h]| 2 * (l + w) + (l * w * h))
        .sum()
}
