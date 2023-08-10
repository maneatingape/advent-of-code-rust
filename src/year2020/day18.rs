//! # Operation Order
//!
//! For part one the operator precedence is the same so we proceed from left to right for
//! each expression. Parentheses are handled via recursion, so we return either when encountering
//! the end of the string or a `)` character.
//!
//! For part two whenever we encounter the lower priority `*` operator then we *implicitly* insert
//! parentheses around the remaining expression. For example:
//!
//! * 1 * 2 * 3 * 4 => 1 * (2 * (3 * (4)))
//! * 1 + 2 * 3 + 4 => 1 + 2 * (3 + 4)
//! * 1 + (2 * 3 * 4) + 5 => 1 + (2 * (3 * (4))) + 5
use std::str::Bytes;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> u64 {
    fn helper(bytes: &mut Bytes) -> u64 {
        let mut total = value(bytes, helper);

        while let Some(operation) = next(bytes) {
            let value = value(bytes, helper);
            if operation == b'+' {
                total += value;
            } else {
                total *= value;
            }
        }

        total
    }

    input.iter().map(|line| helper(&mut line.bytes())).sum()
}

pub fn part2(input: &[&str]) -> u64 {
    fn helper(bytes: &mut Bytes) -> u64 {
        let mut total = value(bytes, helper);

        while let Some(operation) = next(bytes) {
            if operation == b'+' {
                total += value(bytes, helper);
            } else {
                // Implicitly insert '(' and ')' around the remaining sub-expression so when it
                // finishes we break too.
                total *= helper(bytes);
                break;
            }
        }

        total
    }

    input.iter().map(|line| helper(&mut line.bytes())).sum()
}

/// Convenience wrapper around [`Bytes`] iterator. Encountering a `)` is also considered end of
/// sequence. The expressions are consistently formatted so encountering a space just means
/// skip and return the next character that will always be present.
fn next(bytes: &mut Bytes) -> Option<u8> {
    match bytes.next() {
        None | Some(b')') => None,
        Some(b' ') => bytes.next(),
        other => other,
    }
}

/// Convenience wrapper to return the value of either the next raw digit literal or a
/// sub-expression nested in parentheses.
fn value(bytes: &mut Bytes, helper: impl Fn(&mut Bytes) -> u64) -> u64 {
    match next(bytes).unwrap() {
        b'(' => helper(bytes),
        b => (b - b'0') as u64,
    }
}
