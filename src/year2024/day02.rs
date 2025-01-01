//! # Red-Nosed Reports
//!
//! Computes boths parts simultaneously. Each pair of levels is converted into deltas of
//! either +1, -1 or 0. For example:
//!
//! * `1 3 6 7 9` => `+1 +1 +1 +1`
//! * `9 7 6 2 1` => `-1 -1 0 -1`
//!
//! If the sum of all delta equals ±4, then we know that all levels are increasing or
//! decreasing. Any other value indicates either mixed up and down transitions or levels that
//! are too far apart.
//!
//! For part two we remove each pair of deltas (or single delta at each end) then replace with
//! the sum of the delta from the new neighbors on either side.
use crate::util::parse::*;

type Input = (u32, u32);

/// Minimize allocation to only a single `vec` reused for each report.
pub fn parse(input: &str) -> Input {
    let mut report = Vec::new();
    let mut part_one = 0;
    let mut part_two = 0;

    for line in input.lines() {
        report.extend(line.iter_signed::<i32>());

        let (p1, p2) = check(&report);
        part_one += p1;
        part_two += p2;

        report.clear();
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}

fn check(report: &[i32]) -> (u32, u32) {
    let size = report.len();
    let score: i32 = (1..size).map(|i| delta(report[i - 1], report[i])).sum();

    if score.abs() == (size - 1) as i32 {
        return (1, 1);
    }

    for i in 0..size {
        let mut score = score;

        // Snip out each level and replace with new level computed from neighbors to either side.
        if i > 0 {
            score -= delta(report[i - 1], report[i]);
        }
        if i < size - 1 {
            score -= delta(report[i], report[i + 1]);
        }
        if i > 0 && i < size - 1 {
            score += delta(report[i - 1], report[i + 1]);
        }

        if score.abs() == (size - 2) as i32 {
            return (0, 1);
        }
    }

    (0, 0)
}

/// Convert each pair of levels to either +1 for increase, -1 for decrease or 0 for invalid range.
fn delta(a: i32, b: i32) -> i32 {
    let diff = b - a;

    if diff.abs() <= 3 { diff.signum() } else { 0 }
}
