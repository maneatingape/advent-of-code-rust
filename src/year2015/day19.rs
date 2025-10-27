//! # Medicine for Rudolph
//!
//! Part one is a brute force search and replace of every possibility with two optimizations.
//! Replacements that add the same number of extra molecules are grouped together, as different
//! length strings can never match.
//!
//! Next replacement ranges are sorted into ascending order. Non-overlapping ranges can never match,
//! so checking for other equals string only needs to consider ranges that intersect.
//!
//! Part two uses the analysis from `askalski` provided on the
//! [Day 19 solution megathread](https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/).
use crate::util::hash::*;

type Input<'a> = (&'a str, Vec<(&'a str, &'a str)>);

pub fn parse(input: &str) -> Input<'_> {
    let (replacements, molecule) = input.rsplit_once("\n\n").unwrap();
    (molecule, replacements.lines().map(|line| line.split_once(" => ").unwrap()).collect())
}

pub fn part1(input: &Input<'_>) -> usize {
    let (molecule, replacements) = input;

    let mut groups = FastMap::new();
    let mut modified = Vec::new();
    let mut result = 0;

    // Group replacements of the same size together.
    for &(from, to) in replacements {
        let extra = to.len() - from.len();
        groups.entry(extra).or_insert_with(Vec::new).push((from, to));
    }

    for (_, group) in groups {
        // Build list of all possible modified strings.
        for (from, to) in group {
            for (start, _) in molecule.match_indices(from) {
                let end = start + from.len();
                modified.push((start, end, to));
            }
        }

        modified.sort_unstable_by_key(|&(start, ..)| start);

        'outer: for (i, &(start, end, to)) in modified.iter().enumerate() {
            for &(start2, _, to2) in &modified[i + 1..] {
                // Stop checking early once ranges no longer overlap.
                if start2 >= start + to.len() {
                    break;
                }

                // Compare replaced sections for equality.
                let first = to.bytes().chain(molecule[end..].bytes());
                let second = molecule[start..start2].bytes().chain(to2.bytes());

                if first.zip(second).all(|(a, b)| a == b) {
                    continue 'outer;
                }
            }

            result += 1;
        }

        modified.clear();
    }

    result
}

pub fn part2(input: &Input<'_>) -> usize {
    let (molecule, _) = input;

    let elements = molecule.bytes().filter(u8::is_ascii_uppercase).count();
    let rn = molecule.matches("Rn").count();
    let ar = molecule.matches("Ar").count();
    let y = molecule.matches('Y').count();

    elements - rn - ar - 2 * y - 1
}
