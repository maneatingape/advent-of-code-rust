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
//! To beat the record the following condition must hold:
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
    first
        .iter_unsigned::<u128>()
        .zip(second.iter_unsigned::<u128>())
        .map(|(time, distance)| {
            // Use the quadratic formula to find the start and end positions.
            let root = (time * time - 4 * distance).isqrt();
            let mut start = (time - root).div_ceil(2);
            let mut end = time.midpoint(root);

            // As we're using integer math we may need to adjust 1 step.
            if start * (time - start) > distance {
                start -= 1;
            }
            if end * (time - end) > distance {
                end += 1;
            }

            end - start - 1
        })
        .product()
}
