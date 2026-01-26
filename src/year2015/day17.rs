//! # No Such Thing as Too Much
//!
//! Given `n` items the number of possible subsets is `2ⁿ`. We could brute force through each
//! subset by iterating from 0 to 2ⁿ using the binary bits to indicate if a container is present.
//! This will work but is a little slow as there are 20 containers, giving 2²⁰ = 1048576
//! combinations to check.
//!
//! Tackle this with dynamic programming.  All we really have to compute is how
//! many ways a given group size can be reached for any given item index used as
//! the next item added to a smaller group whose item(s) are all smaller indices.
//! Part 1 is then the sum of all counted group sizes that summed to 150, while
//! Part 2 is the counter for the smallest size.
use crate::util::parse::*;

const GOAL: usize = 150;
const ITEMS: usize = 20;

pub fn parse(input: &str) -> [u32; ITEMS + 1] {
    // Initialize an array to collect counts of reachable group sizes.
    let mut counts = [[0; ITEMS + 1]; GOAL + 1];
    let mut cap = 0;
    counts[0][0] = 1;

    // For each container, update ways to reach group sizes, starting from largest sum.
    for (rank, size) in input.iter_unsigned().enumerate() {
        cap += size;
        let goal = if cap < GOAL { cap } else { GOAL };
        for i in (size..goal + 1).rev() {
            for j in 0..rank + 1 {
                counts[i][j + 1] += counts[i - size][j];
            }
        }
    }

    // Result is row 150.
    counts[GOAL]
}

/// We only care about the total combinations, so sum the entire vec.
pub fn part1(input: &[u32]) -> u32 {
    input.iter().sum()
}

/// We want the number of combinations with the fewest containers, so find first non-zero value.
pub fn part2(input: &[u32]) -> u32 {
    *input.iter().find(|&&n| n > 0).unwrap()
}
