use crate::util::parse::*;

const DIGITS: [(&str, u32); 20] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| {
            let first = line.bytes().find(u8::is_ascii_digit).unwrap().to_decimal();
            let last = line.bytes().rev().find(u8::is_ascii_digit).unwrap().to_decimal();
            (10 * first + last) as u32
        })
        .sum()
}

pub fn part2(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| {
            let mut line = *line;

            let first = 'outer: loop {
                for &(digit, value) in &DIGITS {
                    if line.starts_with(digit) {
                        break 'outer value;
                    }
                }
                line = &line[1..];
            };

            let second = 'outer: loop {
                for &(digit, value) in &DIGITS {
                    if line.ends_with(digit) {
                        break 'outer value;
                    }
                }
                line = &line[..line.len() - 1];
            };

            10 * first + second
        })
        .sum()
}
