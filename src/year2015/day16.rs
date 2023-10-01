//! # Aunt Sue
//!
//! Brute force search through each aunt until we find one that matches all the facts.
use crate::util::iter::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> usize {
    let predicate = |key: &str, value: &str| match key {
        "akitas" | "vizslas" => value == "0",
        "perfumes" => value == "1",
        "samoyeds" | "cars" => value == "2",
        "children" | "pomeranians" | "trees" => value == "3",
        "goldfish" => value == "5",
        "cats" => value == "7",
        _ => unreachable!(),
    };
    solve(input, predicate)
}

pub fn part2(input: &str) -> usize {
    let predicate = |key: &str, value: &str| match key {
        "akitas" | "vizslas" => value == "0",
        "perfumes" => value == "1",
        "samoyeds" | "cars" => value == "2",
        "children" => value == "3",
        "pomeranians" => value.unsigned::<u32>() < 3,
        "goldfish" => value.unsigned::<u32>() < 5,
        "trees" => value.unsigned::<u32>() > 3,
        "cats" => value.unsigned::<u32>() > 7,
        _ => unreachable!(),
    };
    solve(input, predicate)
}

fn solve(input: &str, predicate: impl Fn(&str, &str) -> bool) -> usize {
    'outer: for (index, line) in input.lines().enumerate() {
        let tokens = line.split([' ', ':', ',']).filter(|s| !s.is_empty());

        for [key, value] in tokens.chunk::<2>().skip(1) {
            if !predicate(key, value) {
                continue 'outer;
            }
        }

        return index + 1;
    }

    unreachable!()
}
