//! # JSAbacusFramework.io
//!
//! ## Part One
//!
//! The utility [`iter_signed`] method extracts numbers from surrounding text and is used directly.
//!
//! ## Part Two
//!
//! We build a tiny custom JSON parser using a
//! [parser combinator](https://en.wikipedia.org/wiki/Parser_combinator) approach,
//! making some simplifying assumptions:
//! * The input is always well formed and does not contain any whitespace.
//! * Arrays and objects contain at least one item.
//! * We don't care about the content of strings, only if they equal "red" or not.
//!
//! Each parsing function returns a [`Result`] struct which has 3 fields:
//! * `next` The index of the character *after* this object. For example parsing "123," returns
//!   a value of 3 for next.
//! * `ignore`: Only true for strings that exactly equal "red", false otherwise and always
//!   false for numbers, arrays and objects.
//! * `value`: For numbers the literal value, for string zero, for arrays the sum of child
//!   items, for objects the sum of child items if no "red" property is present, otherwise zero.
//!
//! [`iter_signed`]: crate::util::parse
use crate::util::parse::*;

const RED: &[u8] = b"red";

struct Result {
    next: usize,
    ignore: bool,
    value: i32,
}

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> i32 {
    input.iter_signed::<i32>().sum()
}

pub fn part2(input: &str) -> i32 {
    parse_json(input.as_bytes(), 0).value
}

/// Parse JSON that has no whitespace.
fn parse_json(input: &[u8], start: usize) -> Result {
    match input[start] {
        b'[' => parse_array(input, start),
        b'{' => parse_object(input, start),
        b'"' => parse_string(input, start),
        _ => parse_number(input, start),
    }
}

/// Parse array assuming it contains at least one element.
fn parse_array(input: &[u8], start: usize) -> Result {
    let mut index = start;
    let mut total = 0;

    while input[index] != b']' {
        let Result { next, value, .. } = parse_json(input, index + 1);
        index = next;
        total += value;
    }

    Result { next: index + 1, ignore: false, value: total }
}

/// Parse object assuming it contains at least one key/value pair.
fn parse_object(input: &[u8], start: usize) -> Result {
    let mut index = start;
    let mut total = 0;
    let mut ignore = false;

    while input[index] != b'}' {
        let Result { next: first, .. } = parse_string(input, index + 1);
        let Result { next: second, ignore: red, value } = parse_json(input, first + 1);
        index = second;
        total += value;
        ignore |= red;
    }

    Result { next: index + 1, ignore: false, value: if ignore { 0 } else { total } }
}

/// Parse a string evaluating only if it equals "red".
fn parse_string(input: &[u8], start: usize) -> Result {
    let start = start + 1;
    let mut end = start;

    while input[end] != b'"' {
        end += 1;
    }

    Result { next: end + 1, ignore: &input[start..end] == RED, value: 0 }
}

/// Parse an integer value.
fn parse_number(input: &[u8], start: usize) -> Result {
    let mut end = start;
    let mut neg = false;
    let mut acc = 0;

    if input[end] == b'-' {
        neg = true;
        end += 1;
    }

    while input[end].is_ascii_digit() {
        acc = 10 * acc + (input[end] - b'0') as i32;
        end += 1;
    }

    Result { next: end, ignore: false, value: if neg { -acc } else { acc } }
}
