//! # Factory
use crate::util::bitset::*;
use crate::util::parse::*;

type Machine = (u64, Vec<u64>, Vec<u64>);

pub fn parse(input: &str) -> Vec<Machine> {
    let mut tokens = Vec::new();

    input
        .lines()
        .map(|line| {
            tokens.clear();
            tokens.extend(line.split_ascii_whitespace());

            let last = tokens.len() - 1;
            let lights = tokens[0]
                .bytes()
                .skip(1)
                .enumerate()
                .fold(0, |acc, (i, b)| acc | (u64::from(b == b'#') << i));
            let buttons = tokens[1..last]
                .iter()
                .map(|token| token.iter_unsigned::<u64>().fold(0, |acc, i| acc | (1 << i)))
                .collect();
            let joltages = tokens[last].iter_unsigned::<u64>().collect();

            (lights, buttons, joltages)
        })
        .collect()
}

pub fn part1(input: &[Machine]) -> u32 {
    input
        .iter()
        .map(|(lights, buttons, _)| {
            let limit = 1 << buttons.len();
            let mut set = 0;

            loop {
                set += 1;
                let mut n = (1 << set) - 1;

                while n < limit {
                    if *lights == n.biterator().fold(0, |acc, i| acc ^ buttons[i]) {
                        return set;
                    }
                    n = next_same_bits(n);
                }
            }
        })
        .sum()
}

pub fn part2(_input: &[Machine]) -> u32 {
    123456789
}

fn next_same_bits(n: i32) -> i32 {
    let smallest = n & -n;
    let ripple = n + smallest;
    let ones = n ^ ripple;
    let next = (ones >> 2) / smallest;
    ripple | next
}
