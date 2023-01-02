use crate::util::parse::to_i32;

pub fn parse(input: &str) -> Vec<i32> {
    let snacks: Vec<&str> = input.lines().collect();
    let mut elves: Vec<i32> = snacks
        .split(|s| s.is_empty())
        .map(|s| s.iter().map(to_i32).sum())
        .collect();
    elves.sort_unstable();
    elves
}

pub fn part1(input: &[i32]) -> i32 {
    input.iter().rev().take(1).sum()
}

pub fn part2(input: &[i32]) -> i32 {
    input.iter().rev().take(3).sum()
}
