//! # Cube Conundrum
use crate::util::iter::*;
use crate::util::parse::*;

type Game = (u32, u32, u32);

/// Parse each game into the maximum red, green and blue values.
pub fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace().chunk::<2>().skip(1).fold(
                (0, 0, 0),
                |(r, g, b), [amount, color]| {
                    let amount = amount.unsigned();
                    match color.as_bytes()[0] {
                        b'r' => (r.max(amount), g, b),
                        b'g' => (r, g.max(amount), b),
                        b'b' => (r, g, b.max(amount)),
                        _ => unreachable!(),
                    }
                },
            )
        })
        .collect()
}

/// Sum the ids for all valid games.
pub fn part1(input: &[Game]) -> usize {
    input
        .iter()
        .zip(1..)
        .filter_map(|(&(r, g, b), id)| (r <= 12 && g <= 13 && b <= 14).then_some(id))
        .sum()
}

/// Sum the product of maximum red, green and blue values.
pub fn part2(input: &[Game]) -> u32 {
    input.iter().map(|(r, g, b)| r * g * b).sum()
}
