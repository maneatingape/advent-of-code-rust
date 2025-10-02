//! # Custom Customs
//!
//! Part one is solved with a brute force search over every possible pair in the preamble, using a
//! sliding window to advance to each number. To allow testing with the sample data that uses a
//! preamble of 5 but preserve compile-time optimization, the `decrypt` method is
//! [const generic](https://doc.rust-lang.org/reference/items/generics.html#const-generics)
//! in the size of the preamble.
//!
//! Part two uses a sliding search over a variable size window of the input.
use crate::util::parse::*;

type Result = (u64, u64);

pub fn parse(input: &str) -> Result {
    decrypt::<25>(input)
}

pub fn part1(input: &Result) -> u64 {
    input.0
}

pub fn part2(input: &Result) -> u64 {
    input.1
}

pub fn decrypt<const N: usize>(input: &str) -> Result {
    let numbers: Vec<_> = input.iter_unsigned().collect();

    let invalid = numbers
        .windows(N + 1)
        .find(|w| {
            for i in 0..(N - 1) {
                for j in (i + 1)..N {
                    if w[i] + w[j] == w[N] {
                        return false;
                    }
                }
            }
            true
        })
        .map(|w| w[N])
        .unwrap();

    let mut start = 0;
    let mut end = 2;
    let mut sum = numbers[0] + numbers[1];

    while sum != invalid {
        if sum < invalid {
            sum += numbers[end];
            end += 1;
        } else {
            sum -= numbers[start];
            start += 1;
        }
    }

    let slice = &numbers[start..end];
    let min = slice.iter().min().unwrap();
    let max = slice.iter().max().unwrap();
    let weakness = min + max;

    (invalid, weakness)
}
