//! # Rucksack Reorganization
//!
//! The core idea of this puzzle is computing set intersection. We could use the built-in `HashSet`
//! but as the cardinality of the set is so small (52 maximum including both lowercase and
//! uppercase letters) we can instead use a much faster approach of storing each set in a single
//! `u64` integer and using bit manipulation.
//!
//! If a letter is present in the set then the corresponding bit will be `1` otherwise `0`.
//! For example, to add the letter "a", logical OR the set with 1 shifted left by 33.
//!
//! `set | (1 << (b'a' & 0x3f))`
//!
//! Set intersection is the logical AND of two integers which compiles to a single machine
//! instruction.
//!
//! `a & b`
//!
//! To obtain the score we can use the [`trailing_zeros`] method to find the first set bit. On most
//! architectures this also compiles down to a single instruction (`TZCNT` on x86 or `CTZ` on ARM)
//! that is blazing fast.
//!
//! Notes:
//! * We could use a `u128` to use raw ASCII codes, but it performs less efficiently than a `u64`
//!   combined with masked ASCII bytes. We can still not bother with computing offsets until the
//!   very end.
//!
//! [`trailing_zeros`]: u64::trailing_zeros
use crate::util::iter::*;

/// Collect each line into a `vec` of string slices.
pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

/// Split each line into 2 equal halves, then compute the set intersection.
pub fn part1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|&rucksack| {
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
fn mask(s: &str) -> u64 {
    s.bytes().fold(0, |acc, b| acc | (1 << (b & 0x3f)))
}

/// Find the lowest set bit (there should only be one) then convert to priority using the
/// given rules.
fn priority(mask: u64) -> u32 {
    let bit = mask.trailing_zeros();
    if bit > 32 { bit - 32 } else { bit + 26 }
}
