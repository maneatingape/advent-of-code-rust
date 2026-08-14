//! # Handheld Halting
//!
//! A brute force implementation that changes every `Jmp` or `Nop` in the input one at a time then
//! tests the result would have `O(n²)` complexity for part two.
//!
//! We can solve part two in `O(n)` complexity, executing each instruction at most twice. We start
//! the same as the brute force solution by stepping through the input speculatively changing each
//! `Nop` to a `Jmp` or vice-versa, then executing the remaining program from that point and
//! checking if it finishes.
//!
//! The trick is to reuse the `seen` vec that stores if we have executed an instruction before.
//! As each previous failed code path will have executed some instructions, trying to execute an
//! instruction twice means that we know immediately we are on a bad path and can stop.
use crate::util::iter::*;
use crate::util::parse::*;

pub enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Instruction {
    fn from([a, b]: [&str; 2]) -> Self {
        let amount = b.signed();
        match a {
            "acc" => Self::Acc(amount),
            "jmp" => Self::Jmp(amount),
            "nop" => Self::Nop(amount),
            _ => unreachable!(),
        }
    }
}

enum State {
    Infinite(isize),
    Halted(isize),
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input.split_ascii_whitespace().chunk::<2>().map(Instruction::from).collect()
}

pub fn part1(input: &[Instruction]) -> isize {
    let mut seen = vec![false; input.len()];

    match execute(input, 0, 0, &mut seen) {
        State::Infinite(acc) => acc,
        State::Halted(_) => unreachable!(),
    }
}

pub fn part2(input: &[Instruction]) -> isize {
    let mut pc = 0;
    let mut acc = 0;
    let mut seen = vec![false; input.len()];

    loop {
        match input[pc] {
            Instruction::Acc(arg) => {
                pc += 1;
                acc += arg;
            }
            Instruction::Jmp(arg) => {
                let speculative = pc + 1;
                match execute(input, speculative, acc, &mut seen) {
                    State::Infinite(_) => pc = pc.wrapping_add_signed(arg),
                    State::Halted(acc) => break acc,
                }
            }
            Instruction::Nop(arg) => {
                let speculative = pc.wrapping_add_signed(arg);
                match execute(input, speculative, acc, &mut seen) {
                    State::Infinite(_) => pc += 1,
                    State::Halted(acc) => break acc,
                }
            }
        }
    }
}

fn execute(input: &[Instruction], mut pc: usize, mut acc: isize, seen: &mut [bool]) -> State {
    loop {
        if pc >= input.len() {
            break State::Halted(acc);
        } else if seen[pc] {
            break State::Infinite(acc);
        }

        seen[pc] = true;

        match input[pc] {
            Instruction::Acc(arg) => {
                pc += 1;
                acc += arg;
            }
            Instruction::Jmp(arg) => {
                pc = pc.wrapping_add_signed(arg);
            }
            Instruction::Nop(_) => {
                pc += 1;
            }
        }
    }
}
