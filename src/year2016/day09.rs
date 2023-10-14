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

fn decompress(mut slice: &[u8], recurse: bool) -> usize {
    let mut length = 0;

    while !slice.is_empty() {
        if slice[0] == b'(' {
            let (next, amount) = number(slice);
            let (next, repeat) = number(next);

            let start = 1;
            let end = start + amount;
            let result = if recurse { decompress(&next[start..end], true) } else { amount };

            slice = &next[end..];
            length += result * repeat;
        } else {
            slice = &slice[1..];
            length += 1;
        }
    }

    length
}

fn number(slice: &[u8]) -> (&[u8], usize) {
    // Parse number digit by digit, skipping over the delimeter at the start but leaving the
    // delimeter at the end.
    let mut index = 2;
    let mut acc = slice[1].to_decimal() as usize;

    while slice[index].is_ascii_digit() {
        acc = 10 * acc + slice[index].to_decimal() as usize;
        index += 1;
    }

    (&slice[index..], acc)
}
