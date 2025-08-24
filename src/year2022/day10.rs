//! # Cathode-Ray Tube
use crate::util::parse::*;

/// Tokenizes the input treating both "noop" and "addx" as no-ops to obtain the correct
/// instruction timing. Produces a `vec` of the absolute values of `x` from cycle 0 to 241.
pub fn parse(input: &str) -> Vec<i32> {
    let mut x = 1;
    let mut xs = vec![1];

    for token in input.split_ascii_whitespace() {
        match token {
            "noop" | "addx" => (),
            delta => x += delta.signed::<i32>(),
        }
        xs.push(x);
    }

    xs
}

/// Converts between the 0-based indexing produced by the `parse` function and the 1-based indexing
/// used by the problem statement.
pub fn part1(input: &[i32]) -> i32 {
    input.iter().enumerate().skip(19).step_by(40).map(|(i, x)| ((i + 1) as i32) * x).sum()
}

/// Returns pixels as a multi-line [`String`] so that the entire function can be integration tested.
pub fn part2(input: &[i32]) -> String {
    let mut result = String::new();

    for row in input.chunks_exact(40) {
        result.push('\n');
        for (i, &c) in row.iter().enumerate() {
            result.push(if ((i as i32) - c).abs() <= 1 { '#' } else { '.' });
        }
    }

    result
}
