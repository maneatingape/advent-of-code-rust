use crate::util::iter::*;

pub fn parse(input: &str) -> Vec<usize> {
    input
        .as_bytes()
        .split(|b| b.is_ascii_whitespace())
        .chunk::<2>()
        .map(|[a, b]| 3 * ((a[0] as usize) - 65) + ((b[0] as usize) - 88))
        .collect()
}

pub fn part1(input: &[usize]) -> u32 {
    let score = [4, 8, 3, 1, 5, 9, 7, 2, 6];
    input.iter().map(|&i| score[i]).sum()
}

pub fn part2(input: &[usize]) -> u32 {
    let score = [3, 4, 8, 1, 5, 9, 2, 6, 7];
    input.iter().map(|&i| score[i]).sum()
}
