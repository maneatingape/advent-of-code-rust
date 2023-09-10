//! # Sunny with a Chance of Asteroids
use super::intcode::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i64]) -> i64 {
    run(input, 1)
}

pub fn part2(input: &[i64]) -> i64 {
    run(input, 5)
}

/// Start `IntCode` computer in its own thread, sending a single initial value.
/// Receives multiple values from the output channel returning only the last one.
fn run(input: &[i64], value: i64) -> i64 {
    let mut computer = Computer::new(input);
    computer.input(&[value]);

    let mut result = 0;
    while let State::Output(next) = computer.run() {
        result = next;
    }
    result
}
