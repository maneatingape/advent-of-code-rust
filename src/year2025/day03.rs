//! # Lobby
use std::mem::replace;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> u64 {
    solve::<2>(input)
}

pub fn part2(input: &[&str]) -> u64 {
    solve::<12>(input)
}

fn solve<const N: usize>(input: &[&str]) -> u64 {
    let mut batteries = [0; N];

    input
        .iter()
        .map(|&bank| {
            let end = bank.len() - N;
            batteries.copy_from_slice(&bank.as_bytes()[end..]);

            for mut next in bank[..end].bytes().rev() {
                for battery in &mut batteries {
                    if next < *battery {
                        break;
                    }
                    next = replace(battery, next);
                }
            }

            batteries.iter().fold(0, |joltage, &b| 10 * joltage + (b - b'0') as u64)
        })
        .sum()
}
