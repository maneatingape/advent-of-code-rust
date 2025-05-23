//! # Opening the Turing Lock
//!
//! Reverse engineering the code shows that it calculates the length of the
//! [3n + 1 sequence](https://en.wikipedia.org/wiki/Collatz_conjecture)
//! for one of two different numbers chosen depending on whether `a` is 0 or 1.
//!
//! The code fast enough to emulate directly without needing any understanding of what it's doing.
use crate::util::parse::*;

pub enum Op {
    Hlf,
    Tpl,
    IncA,
    IncB,
    Jmp(usize),
    Jie(usize),
    Jio(usize),
}

pub fn parse(input: &str) -> Vec<Op> {
    input
        .lines()
        .enumerate()
        .map(|(i, s)| match s {
            "hlf a" => Op::Hlf,
            "tpl a" => Op::Tpl,
            "inc a" => Op::IncA,
            "inc b" => Op::IncB,
            _ => {
                let index = i.wrapping_add(s.signed::<i32>() as usize);
                match &s[..3] {
                    "jmp" => Op::Jmp(index),
                    "jie" => Op::Jie(index),
                    "jio" => Op::Jio(index),
                    _ => unreachable!(),
                }
            }
        })
        .collect()
}

pub fn part1(input: &[Op]) -> u64 {
    execute(input, 0)
}

pub fn part2(input: &[Op]) -> u64 {
    execute(input, 1)
}

fn execute(input: &[Op], mut a: u64) -> u64 {
    let mut pc = 0;
    let mut b = 0;

    while pc < input.len() {
        match input[pc] {
            Op::Hlf => {
                a /= 2;
                pc += 1;
            }
            Op::Tpl => {
                a *= 3;
                pc += 1;
            }
            Op::IncA => {
                a += 1;
                pc += 1;
            }
            Op::IncB => {
                b += 1;
                pc += 1;
            }
            Op::Jmp(index) => pc = index,
            Op::Jie(index) => pc = if a.is_multiple_of(2) { index } else { pc + 1 },
            Op::Jio(index) => pc = if a == 1 { index } else { pc + 1 },
        }
    }

    b
}
