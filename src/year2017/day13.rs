//! # Packet Scanners
//!
//! This problem is similar to [`Year 2016 Day 15`]. However the key difference is that we need
//! to *avoid* the scanners, so the [Chinese Remainder Theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem)
//! is not applicable.
//!
//! Part one checks that we can calculate the period for each scanner which is `2 * (range - 1)`.
//! For example a scanner with range 3 will be at the top position at time 0, 4, 8 and so on.
//!
//! To avoid a brute force approach for part two we sieve possible values for each scanner
//! sorted in ascending order of period. To combine the previous sieved values with the next
//! scanner, we find the [least common multiple](https://en.wikipedia.org/wiki/Least_common_multiple)
//! of their ranges. Then we "stretch" the sieved values to cover the new range, by adding
//! multiples of the factor between the previous range and the new lcm. Finally, any values
//! that would collide with the new scanner are filtered out.
//!
//! Using the sample data:
//!
//! ```none
//!    0: 3       1  2
//!    1: 2  =>   0  4
//!    4: 4       4  6
//!    6: 4       6  6
//! ```
//!
//! Starting value is `[1]`. First scanner:
//!
//! * Lcm of 1 and 2 => 2
//! * Stretch `[1] => [1+0 1+1] => [1 2]`
//! * Filter `[1 2] => [2]`
//!
//! Second scanner:
//!
//! * Lcm of 2 and 4 => 4
//! * Stretch `[2] => [2+0 2+2] => [2 4]`
//! * Filter `[2 4] => [2]`
//!
//! Third scanner:
//!
//! * Lcm of 4 and 6 => 12
//! * Stretch `[2] => [2+0 2+4 2+8] => [2 6 10]`
//! * Filter `[2 6 10] => [6 10]`
//!
//! Fourth scanner:
//!
//! * Lcm of 12 and 6 => 12
//! * Stretch `[6 10] => [6+0 10+0] => [6 10]`
//! * Filter `[6 10] => [10]`
//!
//! The lowest remaining value is our answer `10`.
//!
//! [`Year 2016 Day 15`]: crate::year2016::day15
use crate::util::iter::*;
use crate::util::math::*;
use crate::util::parse::*;

type Input = Vec<[u32; 2]>;

/// Sorts scanners in ascending order of range.
pub fn parse(input: &str) -> Input {
    let mut scanners: Vec<_> = input.iter_unsigned().chunk::<2>().collect();
    scanners.sort_unstable_by_key(|s| s[1]);
    scanners
}

/// Leaving at time zero the packet will encounter each scanner at time `depth`.
pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .filter_map(|&[depth, range]| {
            let period = 2 * (range - 1);
            depth.is_multiple_of(period).then_some(depth * range)
        })
        .sum()
}

/// Sieves possible values at each scanner stage to reduce the number of possible values.
pub fn part2(input: &Input) -> u32 {
    let mut lcm = 1;
    let mut current = Vec::new();
    let mut next = Vec::new();

    current.push(1);

    for &[depth, range] in input {
        // Find the least common multiple of the existing lcm and the new scanner period.
        let period = 2 * (range - 1);
        let next_lcm = lcm.lcm(period);

        // Check each multiple of the current `end` against the new scanner.
        for extra in (0..next_lcm).step_by(lcm as usize) {
            for &delay in &current {
                if !(delay + extra + depth).is_multiple_of(period) {
                    next.push(delay + extra);
                }
            }
        }

        lcm = next_lcm;
        (current, next) = (next, current);
        next.clear();
    }

    current[0]
}
