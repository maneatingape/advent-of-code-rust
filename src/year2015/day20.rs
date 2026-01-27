//! # Infinite Elves and Infinite Houses
//!
//! ## Part One
//!
//! The amount of presents that each house receives is 10 times the
//! [divisor function](https://en.wikipedia.org/wiki/Divisor_function) `σ`.
//! For example the divisors of 6 are 1, 2, 3 and 6, so house 6 receives
//! 10 + 20 + 30 + 60 = 120 presents. The answer will be a
//! [superabundant number](https://en.wikipedia.org/wiki/Superabundant_number).
//!
//! If `n` has the prime factorization `n = p₁^a₁ × p₂^a₂ × ... × pₖ^aₖ` then the sum of divisors is
//! `σ(n) = [(p₁^(a₁+1) - 1)/(p₁ - 1)] × [(p₂^(a₂+1) - 1)/(p₂ - 1)] × ... × [(pₖ^(aₖ+1) - 1)/(pₖ - 1)]`
//! or more compactly `σ(n) = ∏ᵢ₌₁ᵏ [(pᵢ^(aᵢ+1) - 1)/(pᵢ - 1)]`
//!
//! For example `n = 12 = 2² × 3¹`
//!
//! * `σ(12) = [(2³ - 1)/(2 - 1)] × [(3² - 1)/(3 - 1)]`
//! * `[(8 - 1)/1] × [(9 - 1)/2] = 7 × 4 = 28`
//!
//! Starting from 41 (the largest possible prime encountered in houses up to 50 billion presents)
//! we recursively try smaller and smaller prime powers, finding the smallest house number that
//! exceeds the target.
//!
//! ## Part Two
//!
//! For n>50, it is O(1) work per candidate to check whether the first 50 elves had an integer
//! divisor to visit n, which is faster than the complexity of trying to build up candidates by
//! prime factors.  What's more, an offline brute force search showed that for house numbers
//! between 1008 and 5 billion, all new record-holder sums are created from n divisible by 60.
use crate::util::hash::*;
use crate::util::parse::*;

/// Covers all possible scenarios up to 50 billion presents for part one.
/// Checked by brute forcing all solutions that the highest prime factor is 41.
const PRIMES: [u32; 13] = [41, 37, 31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2];

pub fn parse(input: &str) -> u32 {
    input.unsigned()
}

pub fn part1(input: &u32) -> u32 {
    // Recursively compute the divisor sum greater than the target.
    fn divisor_sum(cache: &mut FastMap<(u32, u32), u32>, primes: &[u32], target: u32) -> u32 {
        if primes.is_empty() {
            return target;
        }

        // Cache previously seen states.
        let key = (primes[0], target);
        if let Some(&value) = cache.get(&key) {
            return value;
        }

        // Try not including this prime.
        let mut result = divisor_sum(cache, &primes[1..], target);
        let mut power = 1;
        let mut sum = 1;

        // Try increasing powers of this prime until the divisor sum exceeds the target.
        while sum < target {
            power *= primes[0];
            sum += power;

            let next = power * divisor_sum(cache, &primes[1..], target.div_ceil(sum));
            result = result.min(next);
        }

        cache.insert(key, result);
        result
    }

    divisor_sum(&mut FastMap::new(), &PRIMES, input.div_ceil(10))
}

pub fn part2(input: &u32) -> u32 {
    fn divisor_sum(house: u32) -> u32 {
        let mut result = 0;
        for i in 1..51 {
            if house.is_multiple_of(i) {
                result += house / i;
            }
        }
        result
    }
    // There is an upper bound presents(n) <= 11 * n * H_(50), where H_ is the k-th harmonic
    // number.  The 50th harmonic number is approximately 4.499, which lets us set a reasonable
    // lower boundto start searching at; using integer math to approximate is fine (it's okay
    // if we check a few extra houses on the low end due to rounding errors).  A brute force
    // exploration up to a target of 50 billion presents determined that we only need to care
    // about a house number that is a multiple of 60.
    let mut candidate = *input / (11 * 4499) * 1000;
    candidate = candidate.div_ceil(60) * 60;
    let target = input.div_ceil(11);
    while divisor_sum(candidate) < target {
        candidate += 60;
    }
    candidate
}
