use crate::util::collection::*;
use crate::util::parse::*;

#[derive(Clone, Copy)]
pub enum Sub {
    Up(i32),
    Down(i32),
    Forward(i32),
}

pub fn parse(input: &str) -> Vec<Sub> {
    let helper = |[a, b]: [&str; 2]| match a {
        "up" => Sub::Up(from(b)),
        "down" => Sub::Down(from(b)),
        "forward" => Sub::Forward(from(b)),
        _ => unreachable!(),
    };
    input
        .split_ascii_whitespace()
        .chunked::<2>()
        .map(helper)
        .collect()
}

pub fn part1(input: &[Sub]) -> i32 {
    let helper = |(position, depth), next| match next {
        Sub::Up(n) => (position, depth - n),
        Sub::Down(n) => (position, depth + n),
        Sub::Forward(n) => (position + n, depth),
    };
    let (position, depth) = input.iter().copied().fold((0, 0), helper);
    position * depth
}

pub fn part2(input: &[Sub]) -> i32 {
    let helper = |(position, depth, aim), next| match next {
        Sub::Up(n) => (position, depth, aim - n),
        Sub::Down(n) => (position, depth, aim + n),
        Sub::Forward(n) => (position + n, depth + aim * n, aim),
    };
    let (position, depth, _) = input.iter().copied().fold((0, 0, 0), helper);
    position * depth
}
