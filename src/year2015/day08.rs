//! # Matchsticks
//!
//! While [regular expressions](https://en.wikipedia.org/wiki/Regular_expression) may feel like a
//! natural choice, it's much faster and easier to simply treat the input as a stream of raw
//! ASCII `u8` bytes including newlines.
//!
//! For part one we run a small state machine using [`fold`] to keep track of the current and
//! previous characters. If we encounter a hexadecimal escape then four characters become one so the
//! difference increases by three. The sequences `\\` and `\"` both increase the difference by one.
//! Each newline increases the difference by two since every line is enclosed with two quotes.
//!
//! Part two is even more straightforward with no need for statekeeping. Quotes and backslashes
//! need to be escaped so increase the difference by one. As before each newline increases by the
//! difference by two.
//!
//! [`fold`]: Iterator::fold
const NEWLINE: u8 = 10;
const QUOTE: u8 = 34;
const SLASH: u8 = 92;
const ESCAPE: u8 = 120;

pub fn parse(input: &str) -> &[u8] {
    input.as_bytes()
}

pub fn part1(input: &[u8]) -> u32 {
    let (_, result) = input.iter().fold((false, 0), |(flag, count), &b| match (flag, b) {
        (true, ESCAPE) => (false, count + 3),
        (true, _) => (false, count + 1),
        (false, SLASH) => (true, count),
        (false, NEWLINE) => (false, count + 2),
        _ => (false, count),
    });
    result
}

pub fn part2(input: &[u8]) -> u32 {
    input
        .iter()
        .map(|&b| match b {
            QUOTE | SLASH => 1,
            NEWLINE => 2,
            _ => 0,
        })
        .sum()
}
