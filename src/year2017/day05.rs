//! # A Maze of Twisty Trampolines, All Alike
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<i32> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i32]) -> u32 {
    let mut jump = input.to_vec();
    let mut index = 0;
    let mut result = 0;

    while index < jump.len() {
        let next = index.wrapping_add(jump[index] as usize);
        jump[index] += 1;
        index = next;
        result += 1;
    }

    result
}

pub fn part2(input: &[i32]) -> u32 {
    let mut jump = input.to_vec();
    let mut index = 0;
    let mut result = 0;

    while index < jump.len() {
        let next = index.wrapping_add(jump[index] as usize);
        if jump[index] < 3 {
            jump[index] += 1;
        } else {
            jump[index] -= 1;
        }
        index = next;
        result += 1;
    }

    result
}
