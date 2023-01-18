use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<u32> {
    let mut elves: Vec<u32> = input
        .split("\n\n")
        .map(|s| s.lines().map(from::<u32>).sum())
        .collect();
    elves.sort_unstable();
    elves
}

pub fn part1(input: &[u32]) -> u32 {
    input.iter().rev().take(1).sum()
}

pub fn part2(input: &[u32]) -> u32 {
    input.iter().rev().take(3).sum()
}
