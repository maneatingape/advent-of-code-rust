use crate::util::collection::*;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|rucksack| {
            let (a, b) = rucksack.split_at(rucksack.len() / 2);
            priority(mask(a) & mask(b))
        })
        .sum()
}

pub fn part2(input: &[&str]) -> u32 {
    input
        .iter()
        .chunked::<3>()
        .map(|[a, b, c]| priority(mask(a) & mask(b) & mask(c)))
        .sum()
}

fn mask(s: &str) -> u128 {
    s.as_bytes().iter().fold(0, |acc, b| acc | 1 << b)
}

fn priority(mask: u128) -> u32 {
    let zeroes = mask.trailing_zeros();
    match zeroes {
        65..=90 => zeroes - 38,
        97..=122 => zeroes - 96,
        _ => unreachable!(),
    }
}
