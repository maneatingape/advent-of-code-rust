//! # Seven Segment Search
//!
//! Listing each digit and the number of segments that are lit when that digit is displayed:
//!
//! | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
//! |---|---|---|---|---|---|---|---|---|---|
//! | 6 | 2 | 5 | 5 | 4 | 5 | 6 | 3 | 7 | 6 |
//!
//! shows that 3 digits share 5 segments and another 3 share 6 segments so we don't have enough
//! information just yet. Listing the total occurrences of each segment summing across all 10 digits:
//!
//! | a | b | c | d | e | f | g |
//! |---|---|---|---|---|---|---|
//! | 8 | 6 | 8 | 7 | 4 | 9 | 7 |
//!
//! shows that 2 segments share 7 occurrences and 2 share 8 occurrences so this is still not quite enough
//! information. However if we combine these 2 tables by *summing* the segment occurrences for each
//! digit, for example `1` has segments `c` and `f` for a total of 17, then the table looks like:
//!
//! | 0  |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |  9 |
//! |----|----|----|----|----|----|----|----|----|----|
//! | 42 | 17 | 34 | 39 | 30 | 37 | 41 | 25 | 49 | 45 |
//!
//! Now each digit can be uniquely identified. Our algorithm is as follows:
//! * Calculate the occurrences of each scrambled segment letter before the `|` symbol. Since the
//!   cardinality of the set is fixed, we can use an array instead of a `HashMap` for speed.
//! * Add the occurrences of each scrambled segment for each digit after the `|` symbol, then
//!   lookup the total and map directly to the unscrambled digit.
use crate::util::iter::*;
use crate::util::slice::*;

type Input = Vec<[u32; 4]>;

pub fn parse(input: &str) -> Input {
    input.lines().map(descramble).collect()
}

pub fn part1(input: &Input) -> usize {
    input.iter().flatten().filter(|&&d| d == 1 || d == 4 || d == 7 || d == 8).count()
}

pub fn part2(input: &Input) -> u32 {
    input.iter().map(|digits| digits.fold_decimal()).sum()
}

fn descramble(line: &str) -> [u32; 4] {
    let mut frequency = [0_u8; 104];
    let bytes = line.as_bytes();
    bytes[0..58].iter().for_each(|&b| frequency[b as usize] += 1);
    bytes[61..]
        .split(|&b| b == b' ')
        .map(|scrambled| to_digit(scrambled.iter().map(|&b| frequency[b as usize]).sum()))
        .chunk::<4>()
        .next()
        .unwrap()
}

fn to_digit(total: u8) -> u32 {
    match total {
        42 => 0,
        17 => 1,
        34 => 2,
        39 => 3,
        30 => 4,
        37 => 5,
        41 => 6,
        25 => 7,
        49 => 8,
        45 => 9,
        _ => unreachable!(),
    }
}
