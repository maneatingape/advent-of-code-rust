//! # The Tyranny of the Rocket Equation
//!
//! The title of the problem is a reference to the
//! [real-life equation](https://en.wikipedia.org/wiki/Tsiolkovsky_rocket_equation).
use crate::util::parse::*;

/// The [`iter_unsigned`] utility method extracts and parses numbers from surrounding text.
///
/// [`iter_unsigned`]: crate::util::parse
pub fn parse(input: &str) -> Vec<u32> {
    input.iter_unsigned().collect()
}

/// Calculate fuel requirements following the formula.
pub fn part1(input: &[u32]) -> u32 {
    input.iter().map(|mass| mass / 3 - 2).sum()
}

/// Calculate the fuel requirements taking into account that fuel needs more fuel to lift it.
/// Mass of 8 or below results in zero or negative fuel so we can stop.
pub fn part2(input: &[u32]) -> u32 {
    input
        .iter()
        .copied()
        .map(|mut mass| {
            let mut fuel = 0;
            while mass > 8 {
                mass = mass / 3 - 2;
                fuel += mass;
            }
            fuel
        })
        .sum()
}
