//! # Sensor Boost
//!
//! This problem is essentially a unit test for the full intcode computer.
use super::intcode::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i64]) -> i64 {
    run(input, 1)
}

pub fn part2(input: &[i64]) -> i64 {
    run(input, 2)
}

fn run(input: &[i64], value: i64) -> i64 {
    let mut computer = Computer::new(input);
    computer.input(value);

    match computer.run() {
        State::Output(result) => result,
        _ => unreachable!(),
    }
}
