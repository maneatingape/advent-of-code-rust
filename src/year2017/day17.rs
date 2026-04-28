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
//! wraps around. For example, if the index is 1, the current value 10,000 and the step 300,
//! then we can insert 34 values at once. The [`div_ceil`] method is perfect for this computation.
//!
//! The number of skips grows geometrically, or `O(ln n)` overall effort, learning the answer in
//! just a few thousand iterations.
//!
//! [`div_ceil`]: usize::div_ceil
use crate::util::parse::*;

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    let step = input.unsigned::<usize>() + 1;

    // For part one, track the index every node had when inserted.
    let mut index = 0;
    let indices: Vec<_> = (1..=2017)
        .map(|n| {
            index = (index + step) % n;
            index
        })
        .collect();

    // Now back up to find the prior node that shares the same index, accounting for when
    // the index moved because an intermediate number was assigned an earlier index.
    let mut next = (index + 1) % 2017;
    let mut part_one = 0;

    for (i, &o) in indices.iter().enumerate().rev() {
        if o == next {
            part_one = i + 1;
            break;
        }
        if o < next {
            next -= 1;
        }
    }

    // For part two, we only need to focus on nodes inserted at index 0.
    let mut n = 2017;
    let mut part_two = 0;

    while n <= 50_000_000 {
        if index == 0 {
            part_two = n;
        }

        let skip = (n - index).div_ceil(step - 1);
        n += skip;
        // Here, n is larger than 2017, while step is on the order of 300 to 400. Therefore, step
        // wraps only once, and subtraction works instead of modulus.
        index = index + skip * step - n;
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}
