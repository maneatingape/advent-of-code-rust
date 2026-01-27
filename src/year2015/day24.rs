//! # It Hangs in the Balance
//!
//! To simplify things the solution assumes that the remaining items after the first best
//! combination is found can be split evenly.
//!
//! This problem is the same as [`Day 17`] part two and we use the same dynamic programming approach
//! with two tables. This approach is described in detail in the day 17 solution.
//!
//! The key difference is that we store the partial quantum entanglement as we build the table.
//! If the number of items from a take/not take choice is fewer, then we just use that quantum
//! entanglement. If the number of items is the same then we use the smaller value. Otherwise
//! we do nothing.
//!
//! [`Day 17`]: crate::year2015::day17
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<usize> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[usize]) -> usize {
    arrangements(input, 3)
}

pub fn part2(input: &[usize]) -> usize {
    arrangements(input, 4)
}

fn arrangements(input: &[usize], groups: usize) -> usize {
    let goal = input.iter().sum::<usize>() / groups;

    // Zero weight needs zero packages.
    let mut minimum = vec![u32::MAX; goal + 1];
    minimum[0] = 0;

    // Define quantum entanglement for zero packages to be 1.
    let mut qe = vec![usize::MAX; goal + 1];
    qe[0] = 1;

    for &item in input {
        for i in (item..goal + 1).rev() {
            let take = minimum[i - item].saturating_add(1);
            let not_take = minimum[i];

            if take < not_take {
                // Taking the item result in fewer packages, use the new quantum entanglement even
                // if it's greater than the existing value.
                qe[i] = item.saturating_mul(qe[i - item]);
                minimum[i] = take;
            } else if take == not_take {
                // Number of packages is the same, so choose the minimum quantum entanglement.
                qe[i] = qe[i].min(item.saturating_mul(qe[i - item]));
            }
        }
    }

    qe[goal]
}
