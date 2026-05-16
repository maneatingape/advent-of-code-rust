//! # Springdroid Adventure
//!
//! Jumps are always 4 tiles wide, landing on `D`. If needed we can jump again immediately
//! landing on `H`. The intcode program runs faster if the springscript is shorter, so although
//! multiple scripts work, any solution that treats intcode as a black box will be better if
//! the script is minimized.
//!
//! ## Part One
//!
//! We jump if any of `A`, `B` or `C` are holes and there is ground where we will land at `D`.
//! This takes 7 instructions:
//!
//! `J = (NOT A OR NOT B OR NOT C) AND D`
//!
//! Using [De Morgan's laws](https://en.wikipedia.org/wiki/De_Morgan%27s_laws) we can simplify
//! to 5 instructions:
//!
//! `J = NOT (A AND B AND C) AND D`
//!
//! But in practice, all input files are set up so that part 1 passes even when we never
//! probe `B`, simplifying to 4 instructions:
//!
//! `J = NOT (A AND C) AND D`
//!
//! ## Part Two
//!
//! Now `B` matters, and we also need a new rule to jump if `H` is ground when `C` is a hole,
//! to trigger a double jump. This can be done in 6 instructions. Suprisingly, we never needed
//! to use register `T`.
use super::intcode::*;
use crate::util::parse::*;

const WALK: &str = "\
OR A J
AND C J
NOT J J
AND D J
WALK
";

const RUN: &str = "\
NOT H J
OR C J
AND B J
AND A J
NOT J J
AND D J
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
