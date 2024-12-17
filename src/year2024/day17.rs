//! # Chronospatial Computer
//!
//! Part one implements the computer specification then runs the provided program. The `b` and `c`
//! registers are assumed to be always zero in the provided input. The computer uses a resumable
//! `run` method that returns `Some(out)` to indicate output and `None` to indicate program end.
//! This is the same flexible approach used by the 2019 [`Intcode`] computer.
//!
//! For part two, reverse engineering the assembly shows that it implements the following
//! algorithm:
//!
//! ```none
//!     while a != 0 {
//!         b = // Some hash based on value of a
//!         out b
//!         a >>= 3
//!     }
//! ```
//!
//! This means that the final value of `a` must be zero. Starting with this knowledge we work
//! backwards digit by digit. The right shift wipes out the lowest 3 bits of `a` so there could
//! be 8 possible previous values. We check each possible value recursively, exploring only
//! those that result in the correct program digit.
//!
//! For each new item we check each of the 8 possible combinations against the next digit
//! in reverse, and so on until we have all possible valid starting values of `a`.
//!
//! Although it may seem that checking could grow exponentially to 8¹⁶ potential values,
//! in practice filtering by correct digit keeps the total less than 50.
//!
//! [`Intcode`]: crate::year2019::intcode
use crate::util::parse::*;
use std::ops::ControlFlow;

pub fn parse(input: &str) -> Vec<u64> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[u64]) -> String {
    // We only care about the value of `a`.
    let mut computer = Computer::new(input, input[0]);
    let mut out = Vec::new();

    while let Some(n) = computer.run() {
        let digit = (n as u8 + b'0') as char;
        out.push(digit);
        out.push(',');
    }

    out.pop();
    out.iter().collect()
}

pub fn part2(input: &[u64]) -> u64 {
    // Start with known final value of `a`.
    helper(input, input.len() - 1, 0).break_value().unwrap()
}

fn helper(program: &[u64], index: usize, a: u64) -> ControlFlow<u64> {
    if index == 2 {
        return ControlFlow::Break(a);
    }

    // Try all 8 combination of lower 3 bits.
    for i in 0..8 {
        let next_a = (a << 3) | i;
        let out = Computer::new(program, next_a).run().unwrap();

        if out == program[index] {
            helper(program, index - 1, next_a)?;
        }
    }

    ControlFlow::Continue(())
}

struct Computer<'a> {
    program: &'a [u64],
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
}

impl Computer<'_> {
    /// The values of `b` and `c` are always 0 in the provided inputs.
    fn new(input: &[u64], a: u64) -> Computer<'_> {
        Computer { program: &input[3..], a, b: 0, c: 0, ip: 0 }
    }

    fn run(&mut self) -> Option<u64> {
        while self.ip < self.program.len() {
            // Convenience closures.
            let literal = || self.program[self.ip + 1];
            let combo = || match self.program[self.ip + 1] {
                n @ 0..4 => n,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => unreachable!(),
            };

            // Computer specification.
            match self.program[self.ip] {
                0 => self.a >>= combo(),
                1 => self.b ^= literal(),
                2 => self.b = combo() % 8,
                3 => {
                    if self.a != 0 {
                        self.ip = literal() as usize;
                        continue;
                    }
                }
                4 => self.b ^= self.c,
                5 => {
                    let out = combo() % 8;
                    self.ip += 2;
                    return Some(out);
                }
                6 => self.b = self.a >> combo(),
                7 => self.c = self.a >> combo(),
                _ => unreachable!(),
            }

            self.ip += 2;
        }

        None
    }
}
