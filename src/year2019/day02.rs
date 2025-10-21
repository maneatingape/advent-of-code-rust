//! # 1202 Program Alarm
//!
//! Substituting symbols instead of numbers into the program shows that it calculates the value of:
//!
//! `a * noun + b * verb + c`
//!
//! We can isolate the value of the constants a, b, c in order to speed up subsequent calculations.
//!
//! As the equation is monotonically increasing in both noun and verb, we can efficiently solve
//! part two by binary searching in two dimensions, instead of a slow brute force check of all
//! possible 10,000 combinations.
use crate::util::parse::*;
use std::cmp::Ordering::*;

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

pub fn part2(input: &Input) -> i32 {
    search(input, 0, 99, 0, 99).unwrap()
}

fn check(input: &[usize], first: usize, second: usize) -> usize {
    let code = &mut input.to_vec();
    code[1] = first;
    code[2] = second;

    execute(code)
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

fn search(input: &Input, x1: i32, x2: i32, y1: i32, y2: i32) -> Option<i32> {
    if x1 > x2 || y1 > y2 {
        return None;
    }

    let x = x1.midpoint(x2);
    let y = y1.midpoint(y2);
    let [a, b, c] = *input;
    let result = a * x + b * y + c;

    match result.cmp(&19690720) {
        Equal => Some(100 * x + y),
        Less => search(input, x + 1, x2, y1, y2).or_else(|| search(input, x1, x2, y + 1, y2)),
        Greater => search(input, x1, x - 1, y1, y2).or_else(|| search(input, x1, x2, y1, y - 1)),
    }
}
