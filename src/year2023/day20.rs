//! # Pulse Propagation
//!
//! Similar to [`Day 8`] the input has a very specific structure. The flip-flops form 4 rows
//! of 12 columns, followed by 2 conjunctions (in square brackets):
//!
//! ```none
//!            / aa ab ac ad ae af ag ah ai aj aj al [ax] [ay] \
//!           /  ba bb bc bd be bf bg bh bi bj bj bl [bx] [by]  \
//!     () - ()                                                 [zz] -> [rx]
//!           \  ca cb dc cd ce cf cg ch ci cj cj cl [cx] [cy]  /
//!            \ da db dc dd de df dg dh di dj dj dl [dx] [dy] /
//! ```
//!
//! The penultimate conjunction in each row, for example `ax` both takes input and delivers output
//! to the flip-flops. This follows a pattern, for example using `v` to indicate input from the
//! conjunction and `v` to indicate output:
//!
//! ```none
//!     v     v        v              v
//!     aa ab ac ad ae af ag ah ai aj aj al
//!     v  v     v  v     v  v  v   v     v
//! ```
//!
//! The flip-flops form a binary counter. When the counter reaches a specific value the conjunction
//! will pulse low and reset the counter to zero. When all 4 counters hit their limit at the
//! same time then a low pulse will be sent to `rx`. The answer is the
//! [LCM](https://en.wikipedia.org/wiki/Least_common_multiple) of the 4 limit values.
//!  For my input the numbers were co-prime so the LCM simplified to a product.
//!
//! For part one, as long as all numbers are greater than 1000, then the counting pulses follow
//! a predictable pattern that we can calculate with some bitwise logic.
//!
//! [`Day 8`]: crate::year2023::day08
use crate::util::hash::*;

type Input = [u32; 4];

pub fn parse(input: &str) -> Input {
    // Build the graph
    let mut node = FastMap::with_capacity(100);
    let mut kind = FastMap::with_capacity(100);

    for line in input.lines() {
        let mut tokens = line.split(|c: char| !c.is_ascii_lowercase()).filter(|s| !s.is_empty());

        let key = tokens.next().unwrap();
        let children: Vec<_> = tokens.collect();

        node.insert(key, children);
        kind.insert(key, !line.starts_with('&'));
    }

    // Follow the nodes from the broadcaster node building each binary number.
    let mut todo = Vec::new();
    let mut numbers = Vec::new();

    for &start in &node["broadcaster"] {
        todo.push((start, 0, 1));
    }

    while let Some((key, mut value, bit)) = todo.pop() {
        let children = &node[key];

        if let Some(next) = children.iter().find(|&&k| kind[k]) {
            if children.len() == 2 {
                value |= bit;
            }
            todo.push((next, value, bit << 1));
        } else {
            numbers.push(value | bit);
        }
    }

    numbers.try_into().unwrap()
}

/// Use bitwise logic to count pulses.
pub fn part1(input: &Input) -> u32 {
    // Counting only works correctly if there are no resets from 1 to 1000
    // so that we can assume all rows increment exactly the same.
    assert!(input.iter().all(|&n| n > 1000));

    // Each conjunction feeds back into the chained flip-flops in the inverse pattern
    // to the flip-flops feeding into the conjunction, except for the least significant
    // flip-flop which is always set. Thus the total is 12 - count_ones + 1.
    let pairs: Vec<_> = input.iter().map(|n| (n, 13 - n.count_ones())).collect();

    // The button and broadcaster contribute 5 low pulses each press.
    let mut low = 5000;
    let mut high = 0;

    for n in 0..1000 {
        // Flip flop changing from off to on emits a high pulse.
        let rising: u32 = !n & (n + 1);
        high += 4 * rising.count_ones();

        // Flip flop changing from on to off emits a low pulse.
        let falling: u32 = n & !(n + 1);
        low += 4 * falling.count_ones();

        for &(number, feedback) in &pairs {
            // Factor is the number of high pulses sent to the conjunction.
            // For each pulse the conjunction feeds a high pulse back to "feedback" flip-flops.
            // In addition the penultimate conjunction in each row receives "factor" high pulses,
            // resulting in "factor" low pulses the final conjunction and finally "factor" high
            // pulses to "rx".
            let factor = (rising & number).count_ones();
            high += factor * (feedback + 3);
            low += factor;

            // Factor is the number of low pulses sent to the conjunction.
            // For each pulse the conjunction feeds a high pulse back to "feedback" flip-flops.
            // In addition the penultimate conjunction in each row receives "factor" high pulses,
            // resulting in "factor" low pulses the final conjunction and finally "factor" high
            // pulses to "rx".
            let factor = (falling & number).count_ones();
            high += factor * (feedback + 2);
            low += 2 * factor;
        }
    }

    low * high
}

/// Assume all numbers are prime (or co-prime) so that the LCM is equal to the product.
pub fn part2(input: &Input) -> u64 {
    input.iter().map(|&n| n as u64).product()
}
