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
//! 1,000 numbers in steps of 17. The raw code takes `O(n²)` complexity for each number so emulating
//! this directly would take at least 10⁵.10⁵.10³ = 10¹³ = 10,000,000,000,000 steps.
//!
//! [`Day 18`]: crate::year2017::day18
use crate::util::parse::*;

/// Pre-compile a list of the primes under 400; adequate for all two-digit seeds.
const PRIMES: [u32; 78] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
];

/// We only need the very first number from the input.
pub fn parse(input: &str) -> u32 {
    input.unsigned()
}

/// The number of `mul` operations is `(n - 2)²`
pub fn part1(input: &u32) -> u32 {
    let n = input - 2;
    n * n
}

/// Count the number of composite numbers in a range calculated from the input number.
pub fn part2(input: &u32) -> usize {
    let start = 100_000 + 100 * input;
    let end = start + 17001;
    let last_prime = PRIMES.partition_point(|&p| p * p < end);
    (start..end).step_by(17).filter(|&n| !is_prime(n, &PRIMES[0..last_prime])).count()
}

/// Simple but effective [prime number check](https://en.wikipedia.org/wiki/Primality_test)
/// trying to identify composite numbers quickly and to test as few factors as possible.
fn is_prime(n: u32, primes: &[u32]) -> bool {
    for &p in primes {
        if n.is_multiple_of(p) {
            return false;
        }
    }
    true
}
