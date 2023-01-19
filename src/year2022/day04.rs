use crate::util::collection::*;
use crate::util::parse::*;

type Pairs = (u32, u32, u32, u32);

pub fn parse(input: &str) -> Vec<Pairs> {
    input.to_unsigned_iter().tupled4().collect()
}

pub fn part1(input: &[Pairs]) -> usize {
    input
        .iter()
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (c >= a && d <= b))
        .count()
}

pub fn part2(input: &[Pairs]) -> usize {
    input
        .iter()
        .filter(|(a, b, c, d)| a <= d && c <= b)
        .count()
}
