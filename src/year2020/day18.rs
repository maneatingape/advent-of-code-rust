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
use crate::util::parse::*;
use std::slice::Iter;

/// A custom iterator treats each line of input as a parenthesized expression to add, so that
/// the solver operates over the entire buffer without breaking it into lines.
struct FlatIter<'a> {
    bytes: Iter<'a, u8>,
    state: State,
}

#[derive(Clone, Copy)]
enum State {
    Start, // Need opening '(' for start of line.
    Body,  // Emitting normal bytes from within a line.
    End,   // Just supplied closing ')' at end of line.
    Plus,  // Emitting '+' between lines.
}

impl<'a> FlatIter<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self { bytes: input.iter(), state: State::Start }
    }
}

impl Iterator for FlatIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::Start => {
                if self.bytes.len() == 0 {
                    return None;
                }
                self.state = State::Body;
                Some(b'(')
            }
            State::Body => {
                match self.bytes.next() {
                    // The expressions are consistently formatted so encountering a space just means
                    // skip and return the next character that will always be present.
                    Some(&b' ') => self.bytes.next().copied(),
                    Some(&b'\n') | None => {
                        self.state = State::End;
                        Some(b')')
                    }
                    other => other.copied(),
                }
            }
            State::End => {
                if self.bytes.len() == 0 {
                    None
                } else {
                    self.state = State::Plus;
                    Some(b'+')
                }
            }
            State::Plus => {
                self.state = State::Body;
                Some(b'(')
            }
        }
    }
}

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> u64 {
    fn helper(bytes: &mut FlatIter<'_>) -> u64 {
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

    let mut flat_iter = FlatIter::new(input.as_bytes());
    helper(&mut flat_iter)
}

pub fn part2(input: &str) -> u64 {
    fn helper(bytes: &mut FlatIter<'_>) -> u64 {
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

    let mut flat_iter = FlatIter::new(input.as_bytes());
    helper(&mut flat_iter)
}

/// Convenience wrapper around [`FlatIter`] iterator. Encountering a `)` is also considered end of
/// sequence.
fn next(bytes: &mut FlatIter<'_>) -> Option<u8> {
    match bytes.next() {
        None | Some(b')') => None,
        other => other,
    }
}

/// Convenience wrapper to return the value of either the next raw digit literal or a
/// sub-expression nested in parentheses.
fn value(bytes: &mut FlatIter<'_>, helper: fn(&mut FlatIter<'_>) -> u64) -> u64 {
    match next(bytes).unwrap() {
        b'(' => helper(bytes),
        b => b.to_decimal() as u64,
    }
}
