//! # Combo Breaker
//!
//! The card loop size is found using the
//! [Baby-step giant-step algorithm](https://en.wikipedia.org/wiki/Baby-step_giant-step).
//! This takes only √20201227 = 4495 steps, compared to potentially up to 20201227 steps
//! for the brute force approach.
//!
//! The common encryption key is then calculated efficiently by
//! [modular exponentiation](https://en.wikipedia.org/wiki/Modular_exponentiation) using
//! [exponentiation by squaring](https://en.wikipedia.org/wiki/Exponentiation_by_squaring).
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::math::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> [u64; 2] {
    input.iter_unsigned().chunk::<2>().next().unwrap()
}

pub fn part1(input: &[u64; 2]) -> u64 {
    let [card_public_key, door_public_key] = *input;
    let card_loop_count = discrete_logarithm(card_public_key);
    door_public_key.mod_pow(card_loop_count, 20201227)
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
    let mut map = FastMapBuilder::with_capacity(m as usize);

    let mut a = 1;
    for j in 0..m {
        map.insert(a, j);
        a = (a * 7) % 20201227;
    }

    let mut b = public_key;
    for i in 0..m {
        if let Some(j) = map.get(&b) {
            return i * m + j;
        }
        b = (b * 680915) % 20201227;
    }

    unreachable!()
}
