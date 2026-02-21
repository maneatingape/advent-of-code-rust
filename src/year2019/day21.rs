//! # Springdroid Adventure
//!
//! Jumps are always 4 tiles wide, landing on `D`. If needed we can jump again immediately
//! landing on `H`.
//!
//! ## Part One
//!
//! We jump if any of `A`, `B` or `C` are holes and there is ground where we will land at `D`.
//! This take 7 instructions:
//!
//! `J = (NOT A OR NOT B OR NOT C) AND D`
//!
//! Using [De Morgan's laws](https://en.wikipedia.org/wiki/De_Morgan%27s_laws) we can simplify
//! to 5 instructions:
//!
//! `J = NOT (A AND B AND C) AND D`
//!
//! ## Part Two
//!
//! We add two rules, either `H` needs to be ground so that we double jump immediately or `E`
//! needs to be ground, so that we can wait and not jump too early.
use super::intcode::*;
use crate::util::parse::*;

const WALK: &str = "\
OR A J
AND B J
AND C J
NOT J J
AND D J
WALK
";

const RUN: &str = "\
OR A J
AND B J
AND C J
NOT J J
AND D J
OR E T
OR H T
AND T J
RUN
";

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i64]) -> i64 {
    survey(input, WALK)
}

pub fn part2(input: &[i64]) -> i64 {
    survey(input, RUN)
}

fn survey(input: &[i64], springscript: &str) -> i64 {
    let mut computer = Computer::new(input);
    computer.input_ascii(springscript);

    let mut result = 0;
    while let State::Output(next) = computer.run() {
        result = next;
    }
    result
}
