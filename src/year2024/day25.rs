//! # Code Chronicle
//!
//! Efficiently checks if locks and keys overlap using bitwise logic. The ASCII character
//! `#` (35) is odd and `.` (46) is even so bitwise AND with 1 results in either 1 or 0.
//! The newline character `\n` (10) is even so will result in 0 and not contribute to matches.
//! There are 25 bits plus 4 newline bits so each lock or key can be stored in an `u32`.
//! For example:
//!
//! ```none
//!    #####
//!    ##.##    11011
//!    .#.##    01011
//!    ...## => 00011 => 110110_010110_000110_000100_00010
//!    ...#.    00010
//!    ...#.    00010
//!    .....
//! ```
pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> u32 {
    let mut slice = input.as_bytes();
    let mut locks = Vec::with_capacity(250);
    let mut keys = Vec::with_capacity(250);
    let mut result = 0;

    while !slice.is_empty() {
        let bits = slice[6..35].iter().fold(0, |bits, &n| (bits << 1) | (n & 1) as u32);

        if slice[0] == b'#' {
            locks.push(bits);
        } else {
            keys.push(bits);
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
