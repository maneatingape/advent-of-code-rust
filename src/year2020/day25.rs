//! # Combo Breaker
//!
//! The puzzle description for today describes the [Diffie-Hellman-Merkle] handshake, which your
//! browser uses every day for encrypted traffic. It relies on the fact that if you use the
//! product of two prime numbers as your encryption key, where each of the two parties in
//! the transaction only knows one of the two prime numbers, then modular arithmetic allows
//! both sides to efficiently compute the same results, while an eavesdropper would have
//! exponentially more work to do to first factor the product into the original components. Here,
//! we are playing the role of the eavesdropper, so we are doing exponential work. But since
//! 2²⁵ is small, brute force is tolerable, when compared to more typical Diffie-Hellman parameters
//! of 2⁵¹² or larger (these days, anything less than 2048 bits is insecure).
//!
//! The common encryption key is then calculated efficiently by [modular exponentiation] using
//! [exponentiation by squaring].
//!
//! That said, we can still perform much faster than the potential of 20201227 steps needed for
//! brute force, by using the [Baby-step giant-step algorithm]. This takes only √20201227 = 4495
//! steps, similar to how listing all divisors of a number can stop after reaching the square
//! root of that number, since all the larger divisors will pair with a smaller one.
//!
//! See also [this alternative patch](https://github.com/maneatingape/advent-of-code-rust/pull/88)
//! that can further speed things up to less than one-tenth of the work by using the [Pohlig-Hellman]
//! algorithm (yes, the same Hellman that described secure key exchange also described how to speed
//! up the factoring of that key). However, the complexity required to exploit properties from
//! [Fermat's Little Theorem] and the [Chinese Remainder Theorem] to reduce the work to √116099 = 341
//! table entries (as one of the factors of `20201227-1`) is harder to maintain, when this
//! solution is already fast enough.
//!
//! [Diffie-Hellman-Merkle](https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange)
//! [modular exponentiation](https://en.wikipedia.org/wiki/Modular_exponentiation)
//! [exponentiation by squaring](https://en.wikipedia.org/wiki/Exponentiation_by_squaring)
//! [Baby-step giant-step algorithm](https://en.wikipedia.org/wiki/Baby-step_giant-step)
//! [Pohlig-Hellman](https://en.wikipedia.org/wiki/Pohlig-Hellman_algorithm)
//! [Fermat's Little Theorem](https://en.wikipedia.org/wiki/Fermat%27s_little_theorem)
//! [Chinese Remainder Theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem)
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::math::*;
use crate::util::parse::*;

const MOD: u64 = 20201227;

pub fn parse(input: &str) -> [u64; 2] {
    input.iter_unsigned().chunk::<2>().next().unwrap()
}

pub fn part1(input: &[u64; 2]) -> u64 {
    let [card_public_key, door_public_key] = *input;
    let card_loop_count = discrete_logarithm(card_public_key);
    door_public_key.mod_pow(card_loop_count, MOD)
}

pub fn part2(_input: &[u64; 2]) -> &'static str {
    "n/a"
}

/// Baby-step giant-step algorithm to compute discrete logarithm.
/// Constants are hardcoded to this specific problem.
/// * 4495 is the ceiling of √20201227
/// * 680915 is 7⁻⁴⁴⁹⁵, or the
///   [multiplicative modular inverse](https://en.wikipedia.org/wiki/Modular_multiplicative_inverse)
///   of 7 to modular exponent 4495.
fn discrete_logarithm(public_key: u64) -> u64 {
    let m = 4495;
    let mut map = FastMap::with_capacity(m as usize);

    let mut a = 1;
    for j in 0..m {
        map.insert(a, j);
        a = (a * 7) % MOD;
    }

    let mut b = public_key;
    for i in 0..m {
        if let Some(j) = map.get(&b) {
            return i * m + j;
        }
        b = (b * 680915) % MOD;
    }

    unreachable!()
}
