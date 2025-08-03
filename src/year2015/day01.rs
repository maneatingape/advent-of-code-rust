//! # Not Quite Lisp
//!
//! The input is first converted into bytes. This is safe as it contains only ASCII characters.
//! Then each parenthesis is parsed into either +1 or -1.
pub fn parse(input: &str) -> Vec<i32> {
    input.trim().bytes().map(|b| if b == b'(' { 1 } else { -1 }).collect()
}

pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

pub fn part2(input: &[i32]) -> usize {
    let mut floor = 0;
    input
        .iter()
        .position(|&b| {
            floor += b;
            floor < 0
        })
        .map(|i| i + 1)
        .unwrap()
}
