//! # Explosives in Cyberspace
//!
//! The only difference between part one and two is that we recursively decompress inner sequences.
use crate::util::parse::*;

pub fn parse(input: &str) -> &[u8] {
    input.trim().as_bytes()
}

pub fn part1(input: &[u8]) -> usize {
    decompress(input, false)
}

pub fn part2(input: &[u8]) -> usize {
    decompress(input, true)
}

fn decompress(mut slice: &[u8], part_two: bool) -> usize {
    let mut length = 0;

    // Find the next marker.
    while let Some(start) = slice.iter().position(|&b| b == b'(') {
        let (next, amount) = number(&slice[start + 1..]);
        let (next, repeat) = number(next);

        // For part two, recursively decompress data.
        let result = if part_two { decompress(&next[..amount], true) } else { amount };

        slice = &next[amount..];
        length += start + result * repeat;
    }

    // Add remaining plain data that doesn't contain any marker.
    length + slice.len()
}

fn number(slice: &[u8]) -> (&[u8], usize) {
    // Parse number digit by digit.
    let mut index = 0;
    let mut acc = 0;

    while slice[index].is_ascii_digit() {
        acc = 10 * acc + slice[index].to_decimal() as usize;
        index += 1;
    }

    // Skip over trailing delimeter.
    (&slice[index + 1..], acc)
}
