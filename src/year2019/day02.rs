//! # 1202 Program Alarm
//!
//! Substituting symbols instead of numbers into the program shows that it calculates the value of:
//!
//! `a * noun + b * verb + c`
//!
//! We can isolate the value of the constants a, b, c in order to speed up subsequent calculations.
//!
//! Since the equation is linear in noun and verb, we can efficiently solve part two by iterating
//! over possible noun values and computing the corresponding verb directly.
use crate::util::parse::*;

type Input = [i32; 3];

pub fn parse(input: &str) -> Input {
    let code: Vec<_> = input.iter_unsigned().collect();

    let c = check(&code, 0, 0) as i32;
    let a = check(&code, 1, 0) as i32;
    let b = check(&code, 0, 1) as i32;

    [a - c, b - c, c]
}

pub fn part1([a, b, c]: &Input) -> i32 {
    a * 12 + b * 2 + c
}

pub fn part2([a, b, c]: &Input) -> i32 {
    (0..100)
        .find_map(|x| {
            let y = (19690720 - a * x - c) / b;
            (a * x + b * y + c == 19690720 && (0..100).contains(&y)).then_some(100 * x + y)
        })
        .unwrap()
}

fn check(input: &[usize], first: usize, second: usize) -> usize {
    let mut code = input.to_vec();
    code[1] = first;
    code[2] = second;

    execute(&mut code)
}

fn execute(code: &mut [usize]) -> usize {
    let mut pc = 0;

    loop {
        match code[pc] {
            1 => code[code[pc + 3]] = code[code[pc + 1]] + code[code[pc + 2]],
            2 => code[code[pc + 3]] = code[code[pc + 1]] * code[code[pc + 2]],
            _ => break code[0],
        }
        pc += 4;
    }
}
