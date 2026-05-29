//! # 1202 Program Alarm
//!
//! Substituting symbols instead of numbers into the program shows that it calculates the value of:
//!
//! `a * noun + b * verb + c`
//!
//! We can isolate the value of the constants a, b, c in order to speed up subsequent calculations.
//!
//! Since the equation is linear in noun and verb, and since a is much larger than b, we can
//! efficiently solve part two by directly reversing the algebra.
use crate::util::parse::*;

type Input = [i32; 3];

pub fn parse(input: &str) -> Input {
    let code: Vec<_> = input.iter_unsigned().collect();
    let a = check(&code, 1, 0);
    let b = check(&code, 0, 1);
    let c = check(&code, 0, 0);

    [a - c, b - c, c]
}

pub fn part1([a, b, c]: &Input) -> i32 {
    a * 12 + b * 2 + c
}

pub fn part2([a, b, c]: &Input) -> i32 {
    let goal = 19690720 - c;
    let x = goal / a;
    let y = (goal % a) / b;
    x * 100 + y
}

fn check(code: &[usize], first: usize, second: usize) -> i32 {
    let mut pc = 0;
    let code = &mut code.to_vec()[..];

    code[1] = first;
    code[2] = second;

    loop {
        match code[pc] {
            1 => code[code[pc + 3]] = code[code[pc + 1]] + code[code[pc + 2]],
            2 => code[code[pc + 3]] = code[code[pc + 1]] * code[code[pc + 2]],
            _ => break code[0] as i32,
        }
        pc += 4;
    }
}
