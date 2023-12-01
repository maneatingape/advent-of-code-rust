//! # Cube Conundrum
use crate::util::iter::*;
use crate::util::parse::*;

pub struct Game(u32, u32, u32);

/// Parse each game into the maximum red, green and blue values.
pub fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace().chunk::<2>().skip(1).fold(
                Game(0, 0, 0),
                |Game(r, g, b), [amount, color]| {
                    let amount = amount.unsigned();
                    match color.as_bytes()[0] {
                        b'r' => Game(r.max(amount), g, b),
                        b'g' => Game(r, g.max(amount), b),
                        b'b' => Game(r, g, b.max(amount)),
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
        .enumerate()
        .filter_map(|(id, &Game(r, g, b))| (r <= 12 && g <= 13 && b <= 14).then_some(id + 1))
        .sum()
}

/// Sum the product of maximum red, green and blue values.
pub fn part2(input: &[Game]) -> u32 {
    input.iter().map(|Game(r, g, b)| r * g * b).sum()
}
