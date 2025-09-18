//! # No Such Thing as Too Much
//!
//! Given `n` items the number of possible subsets is `2ⁿ`. We could brute force through each
//! subset by iterating from 0 to 2ⁿ using the binary bits to indicate if a container is present.
//! This will work but is a little slow as there are 20 containers, giving 2²⁰ = 1048576
//! combinations to check.
//!
//! We speed things up by noticing that some containers are the same size, so the total number
//! of combinations is fewer. For example if there are 3 containers of size `x` that is only
//! 4 possibilities rather than 8.
//!
//! We have to multiply the result by [nCr](https://en.wikipedia.org/wiki/Combination)
//! or the number of ways of choosing `r` items from `n`. For example if 2 containers out of 3
//! are used in a solution then there are 3 ways of selecting the 2 containers. This solution
//! hardcodes nCr tables for `n` up to 4.
//!
//! As an optimization containers are ordered from largest to smallest so that the
//! recursive checking can exit as early as possible if we exceed 150 litres.
use crate::util::parse::*;
use std::collections::BTreeMap;

/// nCr for `n` from 0 to 4 inclusive.
const NCR: [[u32; 5]; 5] =
    [[1, 0, 0, 0, 0], [1, 1, 0, 0, 0], [1, 2, 1, 0, 0], [1, 3, 3, 1, 0], [1, 4, 6, 4, 1]];

struct State {
    size: Vec<u32>,
    freq: Vec<usize>,
    result: Vec<u32>,
}

pub fn parse(input: &str) -> Vec<u32> {
    // Collect size and frequency of each container.
    let mut containers = BTreeMap::new();

    for size in input.iter_unsigned() {
        containers.entry(size).and_modify(|e| *e += 1).or_insert(1);
    }

    // Convenience struct to group size and frequency of each container, plus the number of
    // combinations grouped by total number of containers.
    let mut state = State {
        size: containers.keys().copied().collect(),
        freq: containers.values().copied().collect(),
        result: vec![0; containers.len()],
    };

    // As an optimization order largest containers first so that we can exit early if the total
    // size is greater than 150.
    state.size.reverse();
    state.freq.reverse();

    combinations(&mut state, 0, 0, 0, 1);
    state.result
}

/// We only care about the total combinations, so sum the entire vec.
pub fn part1(input: &[u32]) -> u32 {
    input.iter().sum()
}

/// We want the number of combination with the fewest containers, so find first non-zero value.
pub fn part2(input: &[u32]) -> u32 {
    *input.iter().find(|&&n| n > 0).unwrap()
}

/// Recursively try every possible combination, returning early if the size exceeds 150 litres.
///
/// `state`: Convenience struct to reduce parameters
/// `index`: Current container
/// `containers`: Number of containers used so far
/// `litres`: How many litres of eggnog stored so far
/// `factor`: The total different number of ways of selecting previous containers
#[expect(clippy::needless_range_loop)]
fn combinations(state: &mut State, index: usize, containers: usize, litres: u32, factor: u32) {
    let n = state.freq[index];
    let mut next = litres;

    for r in 0..(n + 1) {
        if next < 150 {
            if index < state.size.len() - 1 {
                combinations(state, index + 1, containers + r, next, factor * NCR[n][r]);
            }
        } else {
            if next == 150 {
                state.result[containers + r] += factor * NCR[n][r];
            }
            break;
        }
        next += state.size[index];
    }
}
