use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<i32> {
    input.to_signed_iter().collect()
}

pub fn part1(input: &[i32]) -> i32 {
    let median = median(input);
    input.iter().map(|n| (n - median).abs()).sum()
}

pub fn part2(input: &[i32]) -> i32 {
    let mean = mean(input);
    let triangle = |x: i32, mean: i32| {
        let n = (x - mean).abs();
        (n * (n + 1)) / 2
    };

    let first: i32 = input.iter().map(|&x| triangle(x, mean)).sum();
    let second: i32 = input.iter().map(|&x| triangle(x, mean + 1)).sum();
    let third: i32 = input.iter().map(|&x| triangle(x, mean - 1)).sum();
    first.min(second).min(third)
}

fn median(input: &[i32]) -> i32 {
    let mut crabs = input.to_vec();
    crabs.sort_unstable();

    let half = input.len() / 2;
    let odd = crabs.len() % 2 == 1;

    if odd {
        crabs[half]
    } else {
        (crabs[half - 1] + crabs[half]) / 2
    }
}

fn mean(input: &[i32]) -> i32 {
    let sum: i32 = input.iter().sum();
    sum / (input.len() as i32)
}
