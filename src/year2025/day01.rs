//! # Secret Entrance
//!
//! Part two left turns are easier if we first "reverse" the dial, then treat it as a right turn.
use crate::util::parse::*;

type Input = (i32, i32);

pub fn parse(input: &str) -> Input {
    let directions = input.bytes().filter(|&b| b.is_ascii_uppercase());
    let amounts = input.iter_signed::<i32>();

    let mut dial = 50;
    let mut part_one = 0;
    let mut part_two = 0;

    for (direction, amount) in directions.zip(amounts) {
        if direction == b'R' {
            part_two += (dial + amount) / 100;
            dial = (dial + amount) % 100;
        } else {
            let reversed = (100 - dial) % 100;
            part_two += (reversed + amount) / 100;
            dial = (dial - amount).rem_euclid(100);
        }
        part_one += i32::from(dial == 0);
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}
