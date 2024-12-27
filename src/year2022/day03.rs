//! # Rucksack Reorganization
//!
//! The core idea of this puzzle is computing set intersection. We could use the built-in `HashSet`
//! but as the cardinality of the set is so small (52 maximum including both lowercase and upper
//! case letters) we can instead use a much faster approach of storing each set in a single `u128`
//! integer and using bit manipulation.
//!
//! If a letter is present in the set then the corresponding bit will be `1` otherwise `0`.
//! For example to add the letter "a", logical OR the set with 1 shifted left by 97
//!
//! `set | (1 << 97)`
//!
//! Set intersection is the logical AND of two integers which compiles to a single machine instruction.
//!
//! `a & b`
//!
//! To obtain the score we can use the [`trailing_zeroes`] method to find the first set bit. On most
//! architectures this also compiles down to a single instruction (`LZCNT` on x86 or `CLZ` on ARM)
//! that is blazing fast.
//!
//! Notes:
//! * We could get away with a `u64` for the set, but by using an `u128` we can shift directly by the
//!   raw ASCII codes and not bother computing offsets until the very end.
//!
//! [`trailing_zeroes`]: u128
use crate::util::iter::*;

/// Collect each line into a `vec` of string slices.
pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

/// Split each line into 2 equal halves, then compute the set intersection.
pub fn part1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|rucksack| {
            let (a, b) = rucksack.split_at(rucksack.len() / 2);
            priority(mask(a) & mask(b))
        })
        .sum()
}

/// Group lines into chunks of 3, then compute the mutual set intersection.
pub fn part2(input: &[&str]) -> u32 {
    input.iter().chunk::<3>().map(|[a, b, c]| priority(mask(a) & mask(b) & mask(c))).sum()
}

/// Build a set from a slice of ASCII characters, using the `fold` function to repeatedly OR
/// bit offsets into an accumulator.
fn mask(s: &str) -> u128 {
    s.bytes().fold(0, |acc, b| acc | (1 << b))
}

/// Find the lowest set bit (there should only be one) then convert to priority using the
/// given rules.
fn priority(mask: u128) -> u32 {
    let zeroes = mask.trailing_zeros();
    match zeroes {
        65..=90 => zeroes - 38,
        97..=122 => zeroes - 96,
        _ => unreachable!(),
    }
}
