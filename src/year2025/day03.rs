//! # Lobby
pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

pub fn part1(input: &[&[u8]]) -> u64 {
    solve(input, 2)
}

pub fn part2(input: &[&[u8]]) -> u64 {
    solve(input, 12)
}

fn solve(banks: &[&[u8]], limit: usize) -> u64 {
    banks
        .iter()
        .map(|&bank| {
            let mut max = 0;
            let mut start = 0;

            (0..limit).rev().fold(0, |joltage, digit| {
                let end = bank.len() - digit;

                (max, start) = (start..end).fold((0, 0), |(max, start), i| {
                    if bank[i] > max { (bank[i], i + 1) } else { (max, start) }
                });

                10 * joltage + (max - b'0') as u64
            })
        })
        .sum()
}
