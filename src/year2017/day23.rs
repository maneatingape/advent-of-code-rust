//! # Coprocessor Conflagration
//!
//! Just like [`Day 18`] reverse engineering the code is essential. The entire input can be reduced
//! to only the very first number.
//!
//! ```none
//!     set b $NUMBER       if a == 0 {
//!     set c b                 b = $NUMBER;
//!     jnz a 2                 c = b;
//!     jnz 1 5             } else {
//!     mul b 100               b = 100000 + 100 * $NUMBER;
//!     sub b -100000           c = b + 17000;
//!     set c b             }
//!     sub c -17000
//!     set f 1             for b in (b..=c).step_by(17) {
//!     set d 2                 f = 1;
//!     set e 2                 for d in 2..b {
//!     set g d                     for e in 2..b {
//!     mul g e                         if d * e == b {
//!     sub g b                             f = 0;
//!     jnz g 2                         }
//!     set f 0
//!     sub e -1
//!     set g e
//!     sub g b
//!     jnz g -8                    }
//!     sub d -1
//!     set g d
//!     sub g b
//!     jnz g -13               }
//!     jnz f 2
//!     sub h -1                if f == 0 {
//!     set g b                     h += 1;
//!     sub g c                 }
//!     jnz g 2
//!     jnz 1 3
//!     sub b -17
//!     jnz 1 -23           }
//!  ```
//!
//! ## Part One
//!
//! The number of `mul` operations is the product of the two inner loops from 2 to `n` exclusive.
//!
//! ## Part Two
//!
//! Counts the number of composite numbers starting from `100,000 + 100 * n` checking the next
//! 1,000 numbers in steps of 17. The raw code take `O(n²)` complexity for each number so emulating
//! this directly would take at least 10⁵.10⁵.10³ = 10¹³ = 10,000,000,000,000 steps.
//!
//! [`Day 18`]: crate::year2017::day18

use crate::util::math::*;
use crate::util::parse::*;

/// We only need the vrey first number from the input.
pub fn parse(input: &str) -> u32 {
    input.unsigned()
}

/// The number of `mul` operations is `(n - 2)²`
pub fn part1(input: &u32) -> u32 {
    (input - 2) * (input - 2)
}

/// Count the number of composite numbers in a range calculated from the input number.
pub fn part2(input: &u32) -> usize {
    (0..=1000).filter_map(|n| composite(100_000 + 100 * input + 17 * n)).count()
}

/// Simple [prime number check](https://en.wikipedia.org/wiki/Primality_test)
/// of all factors from 2 to √n inclusive.
fn composite(n: u32) -> Option<u32> {
    for f in 2..=n.sqrt() {
        if n % f == 0 {
            return Some(n);
        }
    }

    None
}
