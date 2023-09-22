//! # Tractor Beam
//!
//! Finds the approximate boundary of the upper and lower edges of the beam expressed as a slope.
//! We then skip the relatively expensive intcode test if the x and y coordinates lie outside.
use super::intcode::*;
use crate::util::parse::*;

pub struct Input {
    code: Vec<i64>,
    lower: i64,
    upper: i64,
}

pub fn parse(input: &str) -> Input {
    let code: Vec<_> = input.iter_signed().collect();
    let mut lower = 0;
    let mut upper = 0;

    // Find slope of lower and upper edges, rounding down to prevent false negatives.
    while !test(&code, lower + 1, 50) {
        lower += 1;
    }
    while !test(&code, 50, upper + 1) {
        upper += 1;
    }

    Input { code, lower, upper }
}

pub fn part1(input: &Input) -> i64 {
    let code = &input.code;
    // Handle origin specially
    let mut result = test(code, 0, 0) as i64;

    // The beam is continuous so we only need to find the left and right edges.
    for y in 0..50 {
        let mut left = i64::MAX;
        let mut right = i64::MIN;

        for x in 0..50 {
            if precheck(input, x, y) && test(code, x, y) {
                left = x;
                break;
            }
        }
        for x in (0..50).rev() {
            if precheck(input, x, y) && test(code, x, y) {
                right = x;
                break;
            }
        }
        if left <= right {
            result += right - left + 1;
        }
    }

    result
}

pub fn part2(input: &Input) -> i64 {
    let code = &input.code;
    let mut x = 0;
    let mut y = 0;
    let mut moved = true;

    // Increase the right and bottom edges of our box until they are both inside the beam.
    while moved {
        moved = false;

        while !precheck(input, x, y + 99) || !test(code, x, y + 99) {
            x += 1;
            moved = true;
        }

        while !precheck(input, x + 99, y) || !test(code, x + 99, y) {
            y += 1;
            moved = true;
        }
    }

    10000 * x + y
}

/// Quick check with some false positives but no false negatives.
fn precheck(input: &Input, x: i64, y: i64) -> bool {
    50 * y > input.upper * x && 50 * x > input.lower * y
}

/// Definitive but slower check.
fn test(code: &[i64], x: i64, y: i64) -> bool {
    let mut computer = Computer::new(code);
    computer.input(x);
    computer.input(y);

    let State::Output(result) = computer.run() else { unreachable!() };
    result == 1
}
