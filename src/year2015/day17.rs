//! # No Such Thing as Too Much
//!
//! Given `n` items the number of possible subsets is `2ⁿ`. We could brute force through each
//! subset by iterating from 0 to 2ⁿ using the binary bits to indicate if a container is present.
//! This will work but is a little slow as there are 20 containers, giving 2²⁰ = 1048576
//! combinations to check.
//!
//! ## Part One
//!
//! Tackling this with dynamic programming provides a much faster approach. We build a table similar
//! to but not exactly the same as the [knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem).
//! This approach is built atop a solution provided by [`e_blake`](http://reddit.com/u/e_blake)
//! preserved in the [commit history](https://github.com/maneatingape/advent-of-code-rust/blob/4a08f73b40c5b93a59f0c595b97879192f986c31/src/year2015/day17.rs).
//!
//! The table has `target + 1` columns and `containers + 1` rows.
//! Each `table[row, col]` is computed row by row from the sum of:
//!
//! * Not taking the current item, using just the existing number of ways to make the target
//!   `table[row - 1, col]`
//! * Taking the current item, using the existing number of ways to make the target weight less
//!   the weight of the current item `table[row - 1, col - item]`.
//!
//! The table for the example looks like:
//!
//! ```none
//!     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] Initial
//!     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0] 20
//!     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0] 15
//!     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1] 10
//!     [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2] First 5
//!     [1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 4] Second 5
//! ```
//!
//! The answer `4` is the bottom right cell. Since only the previous row is needed to compute
//! the current row, we optimize by only storing a single row instead of the entire table
//! and updating in place.
//!
//! ## Part Two
//!
//! The key insight is to keep *two* tables. The first table tracks the number of combinations as
//! before. The second table tracks the minimum number of containers needed to store each volume
//! up to and including the target size.
//!
//! If taking the item results in fewer containers then we add that number of combinations.
//! Vice-versa if not taking the item results in fewer containers then we don't include it. If both
//! taking and not taking the item results in the same number of containers then we add both.
//!
//! The new minima table for the example using `u32::MAX` to represent `∞` is:
//!
//! ```none
//!     [0, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞] Initial
//!     [0, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, 1, ∞, ∞, ∞, ∞, ∞] 20
//!     [0, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, 1, ∞, ∞, ∞, ∞, 1, ∞, ∞, ∞, ∞, ∞] 15
//!     [0, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, ∞, 1, ∞, ∞, ∞, ∞, 1, ∞, ∞, ∞, ∞, 1, ∞, ∞, ∞, ∞, 2] 10
//!     [0, ∞, ∞, ∞, ∞, 1, ∞, ∞, ∞, ∞, 1, ∞, ∞, ∞, ∞, 1, ∞, ∞, ∞, ∞, 1, ∞, ∞, ∞, ∞, 2] First 5
//!     [0, ∞, ∞, ∞, ∞, 1, ∞, ∞, ∞, ∞, 1, ∞, ∞, ∞, ∞, 1, ∞, ∞, ∞, ∞, 1, ∞, ∞, ∞, ∞, 2] Second 5
//! ```
//!
//! The minimum number of containers needed to make the target is the bottom right cell (2).
//! The combinations table with the minimum restriction is:
//!
//! ```none
//!     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] Initial
//!     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0] 20
//!     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0] 15
//!     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1] 10
//!     [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 2] First 5
//!     [1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 3] Second 5
//! ```
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<usize> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[usize]) -> u32 {
    part1_testable(input, 150)
}

pub fn part2(input: &[usize]) -> u32 {
    part2_testable(input, 150)
}

pub fn part1_testable(input: &[usize], goal: usize) -> u32 {
    // There is one way to store 0 liters with 0 containers.
    let mut ways = vec![0; goal + 1];
    ways[0] = 1;

    for &item in input {
        // Iterating backwards enables us to update the same row in place
        // instead of storing the entire table.
        for i in (item..goal + 1).rev() {
            ways[i] += ways[i - item];
        }
    }

    ways[goal]
}

pub fn part2_testable(input: &[usize], goal: usize) -> u32 {
    let mut ways = vec![0; goal + 1];
    ways[0] = 1;

    // The minimum number of containers to store 0 litres is 0.
    let mut minimum = vec![u32::MAX; goal + 1];
    minimum[0] = 0;

    for &item in input {
        for i in (item..goal + 1).rev() {
            // Saturating add prevents overflow if the minimum is u32::MAX.
            let take = minimum[i - item].saturating_add(1);
            let not_take = minimum[i];

            if take < not_take {
                // Use only the number of combinations that result from taking the container.
                ways[i] = ways[i - item];
                minimum[i] = take;
            } else if take == not_take {
                // Use both combinations from taking and not taking the container.
                ways[i] += ways[i - item];
            }
        }
    }

    ways[goal]
}
