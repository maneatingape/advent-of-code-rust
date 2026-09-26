//! # Red-Nosed Reports
//!
//! Computes both parts simultaneously. Each pair of levels is converted into deltas of
//! either +1, -1 or 0. For example:
//!
//! * `1 3 6 7 9` => `+1 +1 +1 +1`
//! * `9 7 6 2 1` => `-1 -1 0 -1`
//!
//! If the sum of all deltas equals ±4, then we know that all levels are increasing or
//! decreasing. Any other value indicates either mixed up and down transitions or levels that
//! are too far apart.
//!
//! For part two, we remove each pair of deltas (or single delta at each end) then replace with
//! the delta from the new neighbors on either side.
use crate::util::parse::*;

type Input = (u32, u32);

/// Minimize allocation to only a single `vec` reused for each report.
pub fn parse(input: &str) -> Input {
    let mut report = Vec::new();

    input.lines().fold((0, 0), |(part_one, part_two), line| {
        report.clear();
        report.extend(line.iter_signed::<i32>());

        let (p1, p2) = check(&report);
        (part_one + p1, part_two + p2)
    })
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}

fn check(report: &[i32]) -> (u32, u32) {
    let end = report.len() - 1;
    let target = end as i32;
    let score: i32 = report.array_windows().map(|&[a, b]| delta(a, b)).sum();

    if score.abs() == target {
        return (1, 1);
    }

    let dampened = (0..=end).any(|i| {
        let mut score = score;

        // Snip out each level and replace with a new level computed from neighbors on either side.
        if i > 0 {
            score -= delta(report[i - 1], report[i]);
        }
        if i < end {
            score -= delta(report[i], report[i + 1]);
        }
        if i > 0 && i < end {
            score += delta(report[i - 1], report[i + 1]);
        }

        score.abs() == target - 1
    });

    (0, u32::from(dampened))
}

/// Convert each pair of levels to either +1 for increase, -1 for decrease or 0 for invalid range.
fn delta(a: i32, b: i32) -> i32 {
    match b - a {
        1..=3 => 1,
        -3..=-1 => -1,
        _ => 0,
    }
}
