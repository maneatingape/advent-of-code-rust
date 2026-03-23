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
//use crate::util::math::*;
use crate::util::parse::*;

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
    (start..end).step_by(17).filter(|&n| !is_prime(n)).count()
}

/// Simple but effective [prime number check](https://en.wikipedia.org/wiki/Primality_test)
/// trying to identify composite numbers quickly and to test as few factors as possible.
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    if n <= 3 {
        return true;
    }

    if n.is_multiple_of(2) || n.is_multiple_of(3) {
        return false;
    }

    let s = (n - 1).trailing_zeros();
    let d = n >> s;

    // https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Testing_against_small_sets_of_bases
    // "If n < 1,373,653, it is enough to test a = 2 and 3."
    miller_rabin(n, 2, s, d) && miller_rabin(n, 3, s, d)
}

/// Return true if odd n is probably prime, after testing against base a. n must equal 1 + 2**s * d.
#[inline]
fn miller_rabin(n: u32, a: u32, s: u32, d: u32) -> bool {
    let mut x = safe_mod_pow(a, d, n);
    let mut y = 0;
    for _ in 0..s {
        y = ((x as u64 * x as u64) % n as u64) as u32;
        if y == 1 && x != 1 && x != n - 1 {
            return false;
        }
        x = y;
    }
    y == 1
}

/// Custom reimplementation of `crate::util::math::*` that avoids overflow.
#[inline]
fn safe_mod_pow(mut base: u32, mut e: u32, m: u32) -> u32 {
    let mut result = 1;

    while e > 0 {
        if e & 1 == 1 {
            result = ((result as u64 * base as u64) % m as u64) as u32;
        }
        base = ((base as u64 * base as u64) % m as u64) as u32;
        e >>= 1;
    }

    result
}
