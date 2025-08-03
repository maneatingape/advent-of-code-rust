//! # Medicine for Rudolph
//!
//! Part one is a brute force search and replace of every possibility.
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
    let mut distinct = FastSet::new();

    for (from, to) in replacements {
        for (start, _) in molecule.match_indices(from) {
            let size = molecule.len() - from.len() + to.len();
            let end = start + from.len();

            let mut string = String::with_capacity(size);
            string.push_str(&molecule[..start]);
            string.push_str(to);
            string.push_str(&molecule[end..]);

            distinct.insert(string);
        }
    }

    distinct.len()
}

pub fn part2(input: &Input<'_>) -> usize {
    let (molecule, _) = input;

    let elements = molecule.chars().filter(char::is_ascii_uppercase).count();
    let rn = molecule.matches("Rn").count();
    let ar = molecule.matches("Ar").count();
    let y = molecule.matches('Y').count();

    elements - rn - ar - 2 * y - 1
}
