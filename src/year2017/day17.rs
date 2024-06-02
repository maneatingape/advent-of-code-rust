//! # Spinlock
//!
//! There are two insights that speed up part two.
//!
//! The first is that we don't need a buffer. We only need to preserve the last value inserted
//! whenever the index becomes zero. Once 50 million values have been inserted then this value
//! is the final result.
//!
//! The second trick is realizing that we can insert multiple values at a time before the index
//! wraps around. For example if the index is 1, the current value 10,000 and the step 300,
//! then we can insert 34 values at once. The [`div_ceil`] method is perfect for this computation.
//!
//! This reduces the number of loops needed to approximately √50000000 = 7071.
//!
//! [`div_ceil`]: usize::div_ceil
use crate::util::parse::*;

pub fn parse(input: &str) -> usize {
    input.unsigned()
}

pub fn part1(input: &usize) -> u16 {
    let step = input + 1;
    let mut index = 0;
    let mut buffer = vec![0];

    for n in 0..2017 {
        index = (index + step) % buffer.len();
        buffer.insert(index, n + 1);
    }

    buffer[(index + 1) % buffer.len()]
}

pub fn part2(input: &usize) -> usize {
    let step = input + 1;
    let mut n: usize = 1;
    let mut index = 0;
    let mut result = 0;

    while n <= 50_000_000 {
        if index == 0 {
            result = n;
        }

        let skip = (n - index).div_ceil(step);
        n += skip;
        index = (index + skip * step) % n;
    }

    result
}
