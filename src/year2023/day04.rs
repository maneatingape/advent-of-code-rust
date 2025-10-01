//! # Scratchcards
//!
//! Part two is a dynamic programming problem. Starting with 1 copy of each card we add the extra
//! number of copies to the number of following cards equal to the number of winning numbers.
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            // Numbers are at most 99 so we can use a fixed size array instead of a HashSet.
            let mut found = [false; 100];
            let (win, have) = line.split_once('|').unwrap();
            win.iter_unsigned::<usize>().skip(1).for_each(|i| found[i] = true);
            have.iter_unsigned::<usize>().filter(|&i| found[i]).count()
        })
        .collect()
}

pub fn part1(input: &[usize]) -> u32 {
    input.iter().map(|&n| (1 << n) >> 1).sum()
}

pub fn part2(input: &[usize]) -> u32 {
    // Start with a single copy of each card.
    let mut copies = vec![1; input.len()];

    for (i, &n) in input.iter().enumerate() {
        (0..n).for_each(|j| copies[i + j + 1] += copies[i]);
    }

    copies.iter().sum()
}
