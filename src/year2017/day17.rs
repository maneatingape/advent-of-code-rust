//! # Spinlock
//!
//! For part one, we simulate 2017 rounds and record the position (index) at which each number
//! is inserted. We then find the index after the number 2017. Finally, we iterate backwards
//! through the stored indexes to find the first (i.e. last) number inserted at that index.
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

pub fn part1(input: &usize) -> usize {
    let step = input + 1;
    let mut index = 0;
    let mut indexes = vec![0; 2017];
    for len in 1..=2017 {
        index = (index + step) % len;
        indexes[len - 1] = index;
    }
    let mut next = (indexes[2016] + 1) % 2017;

    let mut result = 0;
    for (i, &o) in indexes.iter().enumerate().rev() {
        if o == next {
            result = i + 1;
            break;
        }
        if o < next {
            next -= 1;
        }
    }

    result
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
