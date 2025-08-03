//! # Aunt Sue
//!
//! Brute force search through each aunt until we find one that matches all the facts.
use crate::util::iter::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> usize {
    solve(input, |key, value| match key {
        "akitas" | "vizslas" => value == 0,
        "perfumes" => value == 1,
        "samoyeds" | "cars" => value == 2,
        "children" | "pomeranians" | "trees" => value == 3,
        "goldfish" => value == 5,
        "cats" => value == 7,
        _ => unreachable!(),
    })
}

pub fn part2(input: &str) -> usize {
    solve(input, |key, value| match key {
        "akitas" | "vizslas" => value == 0,
        "perfumes" => value == 1,
        "samoyeds" | "cars" => value == 2,
        "children" => value == 3,
        "pomeranians" => value < 3,
        "goldfish" => value < 5,
        "trees" => value > 3,
        "cats" => value > 7,
        _ => unreachable!(),
    })
}

fn solve(input: &str, predicate: fn(&str, u32) -> bool) -> usize {
    for (index, line) in input.lines().enumerate() {
        if line
            .split([' ', ':', ','])
            .filter(|s| !s.is_empty())
            .chunk::<2>()
            .skip(1)
            .all(|[key, value]| predicate(key, value.unsigned()))
        {
            return index + 1;
        }
    }

    unreachable!()
}
