//! # Safe Cracking
//!
//! Like [`Day 12`] this problem is all about *reading* code not writing code.
//!
//! We could implement a brute force virtual machine without understanding the underlying code
//! but it's much more efficient to analyze the code instead.
//!
//! The first thing we notice is that the following idiom is repeated several times:
//!
//! ```none
//!     inc x
//!     dec y
//!     jnz y -2
//! ```
//!
//! This is equivalent to `x += y` only much less efficient. The `tgl` instruction eventually
//! rewrites a `jnz` to `cpy` to allow the program loop to end.
//!
//! Analysis shows that the code is calculating the [factorial](https://en.wikipedia.org/wiki/Factorial)
//! of `a` plus some constant offset. We can replace the entire code with a single multiplication.
//! If we had emulated the raw instructions directly then it would have taken billions of
//! iterations to get the answer.
//!
//! [`Day 12`]: crate::year2016::day12
use crate::util::parse::*;

/// Extract the constant offset from the assembunny code.
pub fn parse(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let first: u32 = lines[19].unsigned();
    let second: u32 = lines[20].unsigned();
    first * second
}

/// 7! plus some constant.
pub fn part1(input: &u32) -> u32 {
    5040 + input
}

/// 12! plus some constant.
pub fn part2(input: &u32) -> u32 {
    479001600 + input
}
