//! # Chronal Calibration
//!
//! The simplest approach to part two is to store previously seen numbers in a `HashSet` then
//! stop once a duplicate is found. However this approach requires scanning the input of ~1,000
//! numbers multiple times, around 150 times for my input.
//!
//! A much faster `O(nlogn)` approach relies on the fact that each frequency increases by the same
//! amount (the sum of all deltas) each time the list of numbers is processed. For example:
//!
//! ```none
//!    Deltas: +1, -2, +3, +1 =>
//!    0    1   -1    2
//!    3    4    2    5
//! ```
//!
//! Two frequencies that are a multiple of the sum will eventually repeat. First we group each
//! frequencies by its remainder modulo the sum, using `rem_euclid` to handle negative frequencies
//! correctly, Then we sort, first by the remainder to group frequencies that can repeat together,
//! then by the frequency increasing in order to help find the smallest gap between similar
//! frequencies, then lastly by index as this is needed in the next step.
//!
//! For the example this produces `[(0, 0, 0), (1, 1, 1), (2, -1, 2), (2, 2, 3)]`. Then we use
//! a sliding windows of size two to compare each pair of adjacent canditates, considering only
//! candidates with the same remainder. For each valid pair we then produce a tuple of
//! `(frequency gap, index, frequency)`.
//!
//! Finally we sort the tuples in ascending order, first by smallest frequency gap, breaking any
//! ties using the index to find frequencies that appear earlier in the list. The first tuple
//! in the list gives the result, in the example this is `[(3, 2, 2)]`.
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<i32> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

pub fn part2(input: &[i32]) -> i32 {
    // The frequencies increase by this amount each pass through the list of deltas.
    let total: i32 = input.iter().sum();

    // Calculate tuples of `(frequency gap, index, frequency)` then sort to group frequencies that
    // can collide together.
    let mut frequency: i32 = 0;
    let mut seen = Vec::with_capacity(input.len());

    for n in input {
        seen.push((frequency.rem_euclid(total), frequency, seen.len()));
        frequency += n;
    }

    seen.sort_unstable();

    // Compare each adjacent pair of tuples to find candidates, then find the first tuple
    // sorting by smallest gap first, tie breaking with index if needed.
    seen.windows(2)
        .filter_map(|window| {
            let (remainder0, freq0, index0) = window[0];
            let (remainder1, freq1, _) = window[1];

            (remainder0 == remainder1).then_some((freq1 - freq0, index0, freq1))
        })
        .min()
        .map(|(_, _, freq)| freq)
        .unwrap()
}
