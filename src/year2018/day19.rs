//! # Go With The Flow
//!
//! There are two parts to this problem:
//! * Reverse engineering the assembly in order to figure out what the program is doing.
//! * Implementing the program more efficiently in Rust.
//!
//! ## Reverse Engineering
//!
//! ```none
//!     Raw              | Pseudo-Assembly                  | Pseudo-Rust
//!     -----------------+----------------------------------+-----------------------------------
//!     #ip 1            | # a = 0 b = 2 c = 3 d = 4 e = 5  |
//!     addi 1 16 1      |           goto hotel             |
//!     seti 1 8 2       | alfa:     b = 1                  | for b in 1..=e {
//!     seti 1 5 4       | bravo:    d = 1                  |     for d in 1..=e {
//!     mulr 2 4 3       | charlie:  c = b * d              |
//!     eqrr 3 5 3       |           c = (c == e) ? 1: 0    |
//!     addr 3 1 1       |           if c == 1 goto delta   |         if b * d == e {
//!     addi 1 1 1       |           goto echo              |
//!     addr 2 0 0       | delta:    a += b                 |             a += b
//!     addi 4 1 4       | echo:     d += 1                 |
//!     gtrr 4 5 3       |           c = (d > e) ? 1: 0     |         }
//!     addr 1 3 1       |           if c == 1 goto foxtrot |
//!     seti 2 8 1       |           goto charlie           |     }
//!     addi 2 1 2       | foxtrot:  b += 1                 |
//!     gtrr 2 5 3       |           c = (b > e) ? 1: 0     |
//!     addr 3 1 1       |           if c == 1 goto golf    |
//!     seti 1 8 1       |           goto bravo             | }
//!     mulr 1 1 1       | golf:     goto end               |
//!     addi 5 2 5       | hotel:    e = 2                  |
//!     mulr 5 5 5       |           e = e * e              |
//!     mulr 1 5 5       |           e *= 19                |
//!     muli 5 11 5      |           e *= 11                |
//!     addi 3 $FIRST 3  |           c = $FIRST             |
//!     mulr 3 1 3       |           c *= 22                |
//!     addi 3 $SECOND 3 |           c += $SECOND           |
//!     addr 5 3 5       |           e += c                 | e = (22 * $FIRST + $SECOND) + 836
//!     addr 1 0 1       |           if a == 1 goto india   |
//!     seti 0 7 1       |           goto alfa              |
//!     setr 1 1 3       | india:    c = 27                 |
//!     mulr 3 1 3       |           c *= 28                |
//!     addr 1 3 3       |           c += 29                |
//!     mulr 1 3 3       |           c *= 30                |
//!     muli 3 14 3      |           c *= 14                |
//!     mulr 3 1 3       |           c *= 32                |
//!     addr 5 3 5       |           e += c                 | if a == 1 { e += 10550400 }
//!     seti 0 9 0       |           a = 0                  |
//!     seti 0 0 1       |           goto alfa              |
//!                      | end:                             |
//! ```
//!
//! The decoded assembly shows that the program is computing the
//! [sum of the divisors](https://en.wikipedia.org/wiki/Divisor_summatory_function) of a number `n`,
//! using two nested loops for a total complexity in part two of `O(n²) = O(10¹⁴)`.
//!
//! Clearly there is some room for performance improvements. The interesting part is that we only
//! need the two numbers `$FIRST` and `$SECOND` and can discard the rest of the input.
//!
//! ## Rust Implementation
//!
//! We compute the divisor sum using [trial division](https://en.wikipedia.org/wiki/Trial_division).
//! As we want the prime factors (instead of checking that `n` is prime) the asymptotic complexity
//! is slightly lower in practice, being the square root of the largest prime factor of `n`
//! instead of the square root of `n` itself.
//!
//! As `n` is on the order of 10,000,000 this gives a worst case upper bound of `√10000000 = 3162`
//! when `n` is prime. However for most composite numbers the largest prime factor will be much
//! smaller, on the order of 100,000 for an approximate complexity of `√100000 = 316`.
use crate::util::parse::*;

type Input = (u32, u32);

/// Extracts the two unique numbers from the input then calculates the composite numbers
/// needed for both parts.
pub fn parse(input: &str) -> Input {
    let tokens: Vec<u32> = input.iter_unsigned().collect();
    let base = 22 * tokens[65] + tokens[71];
    (base + 836, base + 10551236)
}

pub fn part1(input: &Input) -> u32 {
    divisor_sum(input.0)
}

pub fn part2(input: &Input) -> u32 {
    divisor_sum(input.1)
}

/// Returns the sum of the divisors of an integer `n`, including 1 and `n` itself.
/// For example `20 => 1 + 2 + 4 + 5 + 10 + 20 = 42`.
fn divisor_sum(mut n: u32) -> u32 {
    let mut f = 2;
    let mut sum = 1;

    // We only need to check factors less than or equal to the square root of the greatest prime
    // factor of the input. This loop will only consider prime numbers since we will have sieved
    // out smaller primes. For example `n = 20 = 2 * 2 * 5`. When we check `f = 4`, `n` will
    // already be reduced to 5.
    while f * f <= n {
        // `g` is the next term in the geometric series
        // representing the sum of a repeated prime factor.
        let mut g = sum;

        // `n` could have more than one of the same prime factor.
        while n % f == 0 {
            n /= f;
            g *= f;
            sum += g;
        }

        f += 1;
    }

    // If `n` is one then the greatest prime factor was repeated so has already been included in
    // the sum and we can just return it directly. Otherwise `n` is the unique greatest prime
    // factor and must be added to the sum.
    if n == 1 { sum } else { sum * (1 + n) }
}
