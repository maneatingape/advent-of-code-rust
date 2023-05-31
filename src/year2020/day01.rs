use crate::util::parse::*;
use std::cmp::Ordering::*;

pub fn parse(input: &str) -> Vec<u32> {
    let mut expenses: Vec<_> = input.lines().map(from).collect();
    expenses.sort_unstable();
    expenses
}

pub fn part1(input: &[u32]) -> u32 {
    two_sum(input, 2020).unwrap()
}

pub fn part2(input: &[u32]) -> u32 {
    for i in 0..(input.len() - 2) {
        if let Some(product) = two_sum(&input[(i + 1)..], 2020 - input[i]) {
            return input[i] * product;
        }
    }
    unreachable!()
}

fn two_sum(slice: &[u32], target: u32) -> Option<u32> {
    let mut start = 0;
    let mut end = slice.len() - 1;

    while start < end {
        let sum = slice[start] + slice[end];
        match sum.cmp(&target) {
            Less => start += 1,
            Greater => end -= 1,
            Equal => return Some(slice[start] * slice[end]),
        }
    }

    None
}
