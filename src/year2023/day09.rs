//! # Mirage Maintenance
//!
//! We use exactly four `vec`s for the current line, next line, starting numbers and ending numbers.
//! For efficiency we reuse the `vec`s for each line.
use crate::util::parse::*;

type Input = (i32, i32);

pub fn parse(input: &str) -> Input {
    let mut current = &mut Vec::new();
    let mut next = &mut Vec::new();
    let mut starts = Vec::new();
    let mut ends = Vec::new();

    let mut part_one = 0;
    let mut part_two = 0;

    for line in input.lines() {
        current.extend(line.iter_signed::<i32>());

        while current.iter().any(|&n| n != 0) {
            next.extend(current.windows(2).map(|w| w[1] - w[0]));
            starts.push(current[0]);
            ends.push(current[current.len() - 1]);

            (current, next) = (next, current);
            next.clear();
        }

        part_one += ends.iter().sum::<i32>();
        part_two += starts.iter().rev().fold(0, |acc, s| s - acc);

        current.clear();
        starts.clear();
        ends.clear();
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}
