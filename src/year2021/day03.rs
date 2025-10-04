//! # Binary Diagnostic
//!
//! Part 1 uses bit manipulation to build up the binary numbers directly one digit at a time.
//!
//! Part 2 clones the input `vec` then uses [`retain`] to efficiently discard numbers that
//! don't meet the criteria.
//!
//! [`retain`]: Vec::retain
pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

pub fn part1(input: &[&[u8]]) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;

    for column in 0..input[0].len() {
        let ones = ones(input, column);
        let zeros = input.len() - ones;

        gamma = (gamma << 1) | u32::from(ones > zeros);
        epsilon = (epsilon << 1) | u32::from(zeros > ones);
    }

    gamma * epsilon
}

pub fn part2(input: &[&[u8]]) -> u32 {
    let gamma = rating(input, |a, b| a >= b);
    let epsilon = rating(input, |a, b| a < b);
    gamma * epsilon
}

fn ones(numbers: &[&[u8]], i: usize) -> usize {
    numbers.iter().filter(|b| b[i] == b'1').count()
}

fn rating(input: &[&[u8]], cmp: fn(usize, usize) -> bool) -> u32 {
    let mut numbers = input.to_vec();
    let mut column = 0;

    while numbers.len() > 1 {
        let ones = ones(&numbers, column);
        let zeros = numbers.len() - ones;
        let keep = if cmp(ones, zeros) { b'1' } else { b'0' };

        numbers.retain(|n| n[column] == keep);
        column += 1;
    }

    numbers[0].iter().fold(0, |acc, &n| (acc << 1) | u32::from(n == b'1'))
}
