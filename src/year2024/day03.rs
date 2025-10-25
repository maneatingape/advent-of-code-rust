//! # Mull It Over
//!
//! Solves both parts simultaneously using a custom parser instead of
//! [regex](https://en.wikipedia.org/wiki/Regular_expression).
use crate::util::parse::*;

type Input = (u32, u32);

pub fn parse(input: &str) -> Input {
    let memory = input.as_bytes();
    let mut index = 0;
    let mut enabled = true;
    let mut part_one = 0;
    let mut part_two = 0;

    while index < memory.len() {
        // Skip junk characters
        if memory[index] != b'm' && memory[index] != b'd' {
            index += 1;
            continue;
        }

        // Check possible prefixes
        if memory[index..].starts_with(b"mul(") {
            index += 4;
        } else if memory[index..].starts_with(b"do()") {
            index += 4;
            enabled = true;
            continue;
        } else if memory[index..].starts_with(b"don't()") {
            index += 7;
            enabled = false;
            continue;
        } else {
            index += 1;
            continue;
        }

        // Parse first number
        let first = parse_number(memory, &mut index);

        // First delimiter
        if memory[index] != b',' {
            continue;
        }
        index += 1;

        // Parse second number
        let second = parse_number(memory, &mut index);

        // Second delimiter
        if memory[index] != b')' {
            continue;
        }
        index += 1;

        // Multiply
        let product = first * second;
        part_one += product;
        part_two += if enabled { product } else { 0 };
    }

    (part_one, part_two)
}

fn parse_number(memory: &[u8], index: &mut usize) -> u32 {
    let mut number = 0;
    while memory[*index].is_ascii_digit() {
        number = 10 * number + memory[*index].to_decimal() as u32;
        *index += 1;
    }
    number
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
