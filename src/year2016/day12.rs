//! # Leonardo's Monorail
//!
//! This problem is interesting in that the solution is all about *reading* code not writing code.
//!
//! We could implement a brute force virtual machine without understanding the underlying code
//! but it's much more efficient to analyse the code instead.
//!
//! The first thing we notice is that the following idiom is repeated several times:
//!
//! ```none
//!     inc x
//!     dec y
//!     jnz y -2
//! ```
//!
//! This is equivalent to `x += y` only much less efficient. Replacing this in the code then
//! rewriting the remainder to Rust the program becomes:
//!
//! ```none
//!     let mut a = 1;
//!     let mut b = 1;
//!     let mut c = 0; // 1 in part two
//!     let d = if c == 0 { 26 } else { 33 };
//!     for _ in 0..d {
//!         c = a;
//!         a += b;
//!         b = c;
//!     }
//!     a += q * r // q and r are the constants on lines 17 and 18.
//! ```
//!
//! We can see that the code is calculating the 28th and 35th numbers in the Fibonacci sequence
//! plus some constant offset. We can replace the entire code with a single multiplication.
//! If we had emulated the raw instructions then it would have taken ~10,000,000 iterations to
//! obtain the answer.
use crate::util::parse::*;

/// Extract the constant offset from the assembunny code.
pub fn parse(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let first: u32 = lines[16].unsigned();
    let second: u32 = lines[17].unsigned();
    first * second
}

/// 28th Fibonacci number plus some constant.
pub fn part1(input: &u32) -> u32 {
    317811 + input
}

/// 35th Fibonacci number plus some constant.
pub fn part2(input: &u32) -> u32 {
    9227465 + input
}
