//! # Wait For It
//!
//! We can solve analytically using the quadratic formula.
//! * `x` is time spent holding the button.
//! * `t` is the duration of the race.
//! * `d` is the record distance.
//!
//! Then the distance travelled is:
//!
//! * `x * (t - x)`
//!
//! To beat the record the following conditition must hold:
//!
//! * `x * (t - x) = d`
//! * `x² - tx +d = 0`
//!
//! The start and end times where we will be the record are given by the roots of the
//! quadratic equation which we can solve using the
//! [quadratic formula](https://en.wikipedia.org/wiki/Quadratic_formula).
//!
//! * `(t ± √(t² - 4d)) / 2`
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> u128 {
    race(input[0], input[1])
}

pub fn part2(input: &[&str]) -> u128 {
    race(&merge(input[0]), &merge(input[1]))
}

fn merge(line: &str) -> String {
    line.chars().filter(char::is_ascii_digit).collect()
}

fn race(first: &str, second: &str) -> u128 {
    let times: Vec<u128> = first.iter_unsigned().collect();
    let distances: Vec<u128> = second.iter_unsigned().collect();
    let mut result = 1;

    for (&time, &distance) in times.iter().zip(distances.iter()) {
        // Use the quadratic formula to find the start and end positions.
        let root = isqrt(time * time - 4 * distance);
        let mut start = (time - root).div_ceil(2);
        let mut end = (time + root) / 2;

        // As we're using integer math we may need to adjust 1 step.
        while start * (time - start) > distance {
            start -= 1;
        }
        while end * (time - end) > distance {
            end += 1;
        }

        result *= end - start - 1;
    }

    result
}

/// Binary search to find the
/// [integer square root](https://en.wikipedia.org/wiki/Integer_square_root).
fn isqrt(n: u128) -> u128 {
    let mut start = 0;
    let mut end = n;

    while start < end {
        let middle = (start + end) / 2;

        if middle * middle > n {
            end = middle;
        } else {
            start = middle + 1;
        }
    }

    start
}
