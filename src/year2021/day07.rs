//! # The Treachery of Whales
//!
//! Part 1 is a disguised definition of the mathematical [median](https://en.wikipedia.org/wiki/Median).
//! We can calculate the result immediately using the standard algorithm.
//!
//! Part 2 is found by using the [mean](https://en.wikipedia.org/wiki/Mean).
//! However since this could a floating point value and we are using integers we need to check
//! 3 values total, the rounded result and one value on either side to ensure the correct answer.
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<i32> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i32]) -> i32 {
    let median = median(input);
    input.iter().map(|n| (n - median).abs()).sum()
}

pub fn part2(input: &[i32]) -> i32 {
    let mean = mean(input);
    let triangle = |x: i32, mean: i32| {
        let n = (x - mean).abs();
        (n * (n + 1)) / 2
    };

    (-1..=1).map(|delta| input.iter().map(|&x| triangle(x, mean + delta)).sum()).min().unwrap()
}

fn median(input: &[i32]) -> i32 {
    let mut crabs = input.to_vec();
    crabs.sort_unstable();

    let half = input.len() / 2;
    if crabs.len().is_multiple_of(2) { crabs[half - 1].midpoint(crabs[half]) } else { crabs[half] }
}

fn mean(input: &[i32]) -> i32 {
    let sum: i32 = input.iter().sum();
    sum / (input.len() as i32)
}
