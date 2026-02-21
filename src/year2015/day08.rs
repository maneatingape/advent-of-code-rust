//! # Matchsticks
//!
//! While [regular expressions](https://en.wikipedia.org/wiki/Regular_expression) may feel like a
//! natural choice, it's much faster and easier to simply treat the input as a single stream of raw
//! ASCII `u8` bytes without splitting line by line.
//!
//! For part one we skip over the first quote of each line. The last quote on each line increases
//! the difference by two since every line is enclosed with two quotes. If we encounter a
//! hexadecimal escape then four characters become one so the difference increases by three.
//! The sequences `\\` and `\"` both increase the difference by one.
//!
//! Part two is even more straightforward. Quotes and backslashes need to be escaped so increase
//! the difference by one. Each newline increases the difference by two.
const NEWLINE: u8 = b'\n';
const QUOTE: u8 = b'\"';
const SLASH: u8 = b'\\';
const ESCAPE: u8 = b'x';

pub fn parse(input: &str) -> &[u8] {
    input.as_bytes()
}

pub fn part1(input: &[u8]) -> usize {
    // Skip very first quote to prevent double counting.
    let mut index = 1;
    let mut result = 0;

    while index < input.len() {
        let skip = match input[index] {
            SLASH => match input[index + 1] {
                ESCAPE => 4,
                _ => 2,
            },
            QUOTE => 3,
            _ => 1,
        };
        result += skip - 1;
        index += skip;
    }

    result
}

pub fn part2(input: &[u8]) -> u32 {
    input
        .iter()
        .map(|&b| match b {
            // Escape special characters.
            QUOTE | SLASH => 1,
            // Each line needs two enclosing quotes.
            NEWLINE => 2,
            _ => 0,
        })
        .sum()
}
