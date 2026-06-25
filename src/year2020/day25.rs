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
//! This can further be sped up by using the [Pohlig-Hellman] algorithm (yes, the same Hellman that
//! described secure key exchange also described how to speed up the factoring of that key). This
//! algorithm exploits properties from [Fermat's Little Theorem] and the [Chinese Remainder Theorem]
//! to show that instead of having to compute the discrete logarithm of `n`, it is possible
//! to compute the discrete logarithm of each of the prime factors of `n-1` and then combine
//! those to the final answer. The factors of `20201227` are `2 * 3 * 29 * 116099`, which means
//! we can use Baby-step-giant-step on √116099 = 341 table entries, plus solving for 2, 3, and 29
//! (all small enough to hard-code their lookup tables), then combine those four results in less
//! effort than the 4495 iterations required over 20201227 proper.
//!
//! [Diffie-Hellman-Merkle](https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange)
//! [modular exponentiation](https://en.wikipedia.org/wiki/Modular_exponentiation)
//! [exponentiation by squaring](https://en.wikipedia.org/wiki/Exponentiation_by_squaring)
//! [Baby-step giant-step algorithm](https://en.wikipedia.org/wiki/Baby-step_giant-step)
//! [Pohlig-Hellman](https://en.wikipedia.org/wiki/Pohlig-Hellman_algorithm)
//! [Fermat's Little Theorem](https://en.wikipedia.org/wiki/Fermat%27s_little_theorem)
//! [Chinese Remainder Theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem)
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
/// * 341 is the ceiling of √116099
/// * 174 is 20201226/116099
/// * 19372176 is 7⁻¹⁷⁴ mod 20201227, for the baby step
/// * 16585664 is 7¹⁷⁴*⁽¹¹⁶⁰⁹⁹⁻³⁴¹⁾ mod 20201227, for the giant step
fn dlog_116099(public_key: u64) -> u64 {
    /// Precompute the baby-step table for 116099 at compile time.
    const M: usize = 341;
    const fn precompute_baby_steps() -> [(u64, u64); M] {
        let mut table = [(0, 0); 341];
        let mut a = 1;

        let mut j = 0;
        while j < M {
            table[j] = (a, j as u64);
            a = (a * 19372176) % MOD;
            j += 1;
        }

        let mut i = 1;
        while i < M {
            let key = table[i];
            let mut k = i;
            while k > 0 && table[k - 1].0 > key.0 {
                table[k] = table[k - 1];
                k -= 1;
            }
            table[k] = key;
            i += 1;
        }

        table
    }

    const BABY_STEPS: [(u64, u64); M] = precompute_baby_steps();

    let mut b = public_key.mod_pow(174, MOD);
    for i in 0..M {
        if let Ok(idx) = BABY_STEPS.binary_search_by_key(&b, |&(key, _)| key) {
            let j = BABY_STEPS[idx].1 as usize;
            return (i * M + j) as u64;
        }
        b = (b * 16585664) % MOD;
    }

    unreachable!()
}

/// Constants are hardcoded to this specific problem.
/// 696594 = 20201226/29
/// 303116 = 7⁶⁹⁶⁵⁹⁴ mod 20201227
/// The discrete logarithms of powers of 696594 mod 20201227 form a cycle of length 29, small
/// enough to list in a hard-coded table; each row takes the previous row * 303116 % 20201227.
fn dlog_29(public_key: u64) -> u64 {
    match public_key.mod_pow(696594, MOD) {
        1 => 0,
        303116 => 1,
        4129060 => 2,
        17132175 => 3,
        7938545 => 4,
        10650888 => 5,
        15675230 => 6,
        3621372 => 7,
        1522426 => 8,
        15051055 => 9,
        10884154 => 10,
        18037586 => 11,
        18830426 => 12,
        7322247 => 13,
        1612389 => 14,
        12619313 => 15,
        13346858 => 16,
        7081919 => 17,
        20176130 => 18,
        8560327 => 19,
        5275690 => 20,
        16920720 => 21,
        11038036 => 22,
        17500755 => 23,
        17648515 => 24,
        19948416 => 25,
        12396162 => 26,
        6416338 => 27,
        1378556 => 28,
        _ => unreachable!(),
    }
}

/// Constants are hardcoded to this specific problem.
/// 6733742 = 20201226/3
/// 15303599 = 7⁶⁷³³⁷⁴² mod 20201227
/// The discrete logarithms of powers of 6733742 mod 20201227 form a cycle of length 3, small
/// enough to call out directly.
fn dlog_3(public_key: u64) -> u64 {
    match public_key.mod_pow(6733742, MOD) {
        1 => 0,
        15303599 => 1,
        4897627 => 2,
        _ => unreachable!(),
    }
}

/// Constants are hardcoded to this specific problem.
/// 10100613 = 20201226/2
/// The discrete logarithms of powers of 10100613 mod 20201227 form a cycle of length 2, small
/// enough to call out directly.
fn dlog_2(public_key: u64) -> u64 {
    match public_key.mod_pow(10100613, MOD) {
        1 => 0,
        20201226 => 1,
        _ => unreachable!(),
    }
}

/// Use Pohlig-Hellman computation of the discrete logarithms of each of the proime factors of
/// 20201227-1, then combine them through pre-computed multiplicative modular inverses needed
/// to satisfy the Chinese Remainder Theorem.
/// Constants are hardcoded to this specific problem.
/// 18227544 = ((20201226/116099)⁻¹ mod 116099) * 174 mod 20201226
/// 18808038 = ((20201226/29)⁻¹ mod 29) * 696594 mod 20201226
/// 13467484 = ((20201226/3)⁻¹ mod 3) * 6733742 mod 20201226
/// 10100613 = ((20201226/2)⁻¹ mod 2) * 10100613 mod 20201226
fn discrete_logarithm(public_key: u64) -> u64 {
    (18227544 * dlog_116099(public_key) % (MOD - 1)
        + 18808038 * dlog_29(public_key) % (MOD - 1)
        + 13467484 * dlog_3(public_key) % (MOD - 1)
        + 10100613 * dlog_2(public_key) % (MOD - 1))
        % (MOD - 1)
}
