//! # The Treachery of Whales
//!
//! Part one is a disguised definition of the mathematical [median](https://en.wikipedia.org/wiki/Median).
//! We can calculate the result immediately using the standard algorithm. Even though there
//! are an even number of crabs, any integer between the 500th and 501st crab inclusive will
//! work (the extra fuel spent by half the crabs perfectly cancels the fuel saved by the other
//! half, when switching between integers in that range).
//!
//! Part two is found by using the [mean](https://en.wikipedia.org/wiki/Mean).
//! However, since this could be a floating point value and we are using integers we need to check
//! both the floor and the ceiling of that result to ensure the correct answer.
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<i32> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i32]) -> i32 {
    let median = median(input);
    input.iter().map(|n| (n - median).abs()).sum()
}

pub fn part2(input: &[i32]) -> i32 {
    let fuel = |target: i32| -> i32 {
        input
            .iter()
            .map(|x| {
                let n = (x - target).abs();
                (n * (n + 1)) / 2
            })
            .sum()
    };

    let mean = mean(input);
    fuel(mean).min(fuel(mean + 1))
}

fn median(input: &[i32]) -> i32 {
    // A radix sort followed by a short-circuiting .position() would also work, but takes
    // more lines of code without much more speed.
    let mut crabs = input.to_vec();
    let middle = crabs.len() / 2;
    *crabs.select_nth_unstable(middle).1
}

fn mean(input: &[i32]) -> i32 {
    input.iter().sum::<i32>() / (input.len() as i32)
}
