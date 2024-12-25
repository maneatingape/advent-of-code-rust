//! # Code Chronicle
const MASK: u64 = 0b_011111_011111_011111_011111_011111;

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> u32 {
    let mut slice = input.as_bytes();
    let mut locks = Vec::with_capacity(250);
    let mut keys = Vec::with_capacity(250);
    let mut result = 0;

    while !slice.is_empty() {
        let bits = slice[6..35].iter().fold(0, |bits, &n| (bits << 1) | (n & 1) as u64);

        if slice[0] == b'#' {
            locks.push(bits & MASK);
        } else {
            keys.push(bits & MASK);
        }

        slice = &slice[43.min(slice.len())..];
    }

    for lock in &locks {
        for key in &keys {
            result += (lock & key == 0) as u32;
        }
    }

    result
}

pub fn part2(_input: &str) -> &'static str {
    "n/a"
}
