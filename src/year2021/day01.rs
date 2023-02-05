use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<u32> {
    input.lines().map(from).collect()
}

pub fn part1(input: &[u32]) -> usize {
    input.windows(2).filter(|w| w[0] < w[1]).count()
}

pub fn part2(input: &[u32]) -> usize {
    input.windows(4).filter(|w| w[0] < w[3]).count()
}
