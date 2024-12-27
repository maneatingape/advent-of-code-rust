//! # Custom Customs
//!
//! This is a disguised binary question like the previous [`day 5`].
//!
//! We can store each passenger's answers as an implicit set in a `u32` since the cardinality
//! is only 26. For each yes answer we set a bit, shifting left based on the letter. For example
//! `acf` would be represented as `100101`.
//!
//! For part one to find groups where any person answered yes, we reduce the group using
//! [bitwise OR](https://en.wikipedia.org/wiki/Bitwise_operation) then count the number of ones
//! for each group using the blazing fast [`count_ones`] intrinsic.
//!
//! Part two is very similar, except that we use a bitwise AND instead.
//!
//! [`day 5`]: crate::year2020::day05
//! [`count_ones`]: u32::count_ones

pub fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.bytes().fold(0, |acc, b| acc | (1 << (b - b'a')))).collect()
}

pub fn part1(input: &[u32]) -> u32 {
    let mut total = 0;
    let mut group = u32::MIN;

    for &passenger in input {
        if passenger == 0 {
            total += group.count_ones();
            group = u32::MIN;
        } else {
            group |= passenger;
        }
    }

    total + group.count_ones()
}

pub fn part2(input: &[u32]) -> u32 {
    let mut total = 0;
    let mut group = u32::MAX;

    for &passenger in input {
        if passenger == 0 {
            total += group.count_ones();
            group = u32::MAX;
        } else {
            group &= passenger;
        }
    }

    total + group.count_ones()
}
