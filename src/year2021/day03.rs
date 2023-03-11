//! # Binary Diagnostic
//!
//! Part 1 uses bit manipulation to build up the binary numbers directly one digit at a time.
//!
//! Part 2 clones the input `vec` then uses [`swap_remove`] to efficiently discard numbers that
//! don't meet the criteria without having to move all subsequent elements.
//!
//! [`swap_remove`]: Vec::swap_remove

pub struct Input<'a> {
    width: usize,
    numbers: Vec<&'a [u8]>,
}

pub fn parse(input: &str) -> Input {
    let numbers: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    Input {
        width: numbers[0].len(),
        numbers,
    }
}

pub fn part1(input: &Input) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..input.width {
        let sum = sum(&input.numbers, i);
        if sum > input.numbers.len() - sum {
            gamma = (gamma << 1) | 1;
            epsilon <<= 1;
        } else {
            gamma <<= 1;
            epsilon = (epsilon << 1) | 1;
        }
    }

    gamma * epsilon
}

pub fn part2(input: &Input) -> u32 {
    let gamma = rating(input, |a, b| a >= b);
    let epsilon = rating(input, |a, b| a < b);
    gamma * epsilon
}

fn sum(numbers: &[&[u8]], i: usize) -> usize {
    let total: usize = numbers.iter().map(|b| b[i] as usize).sum();
    total - 48 * numbers.len()
}

fn fold(numbers: &[u8], width: usize) -> u32 {
    numbers
        .iter()
        .take(width)
        .fold(0, |acc, &n| (acc << 1) | (n & 1) as u32)
}

fn rating(input: &Input, cmp: impl Fn(usize, usize) -> bool) -> u32 {
    let mut numbers = input.numbers.clone();

    for i in 0..input.width {
        let sum = sum(&numbers, i);
        let keep = if cmp(sum, numbers.len() - sum) {
            b'1'
        } else {
            b'0'
        };
        filter(&mut numbers, i, keep);
        if numbers.len() == 1 {
            return fold(numbers[0], input.width);
        }
    }

    unreachable!()
}

fn filter(numbers: &mut Vec<&[u8]>, i: usize, keep: u8) {
    let mut j = 0;

    while j < numbers.len() {
        if numbers[j][i] == keep {
            j += 1;
        } else {
            numbers.swap_remove(j);
        }
    }
}
