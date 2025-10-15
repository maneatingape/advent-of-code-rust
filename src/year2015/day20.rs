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
//! We get a list of all possible house numbers that have divisor sums that exceed the value.
//! Checking in ascending order, each house is broken down into its factors, including only those
//! where the second elf will actually deliver.
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
    // Differences from part one:
    // * Return all possible house numbers.
    // * Remove cache since each state is unique so it slows things down.
    fn divisor_sum(candidates: &mut Vec<u32>, primes: &[u32], house: u32, target: u32) -> u32 {
        if primes.is_empty() {
            if target == 1 {
                candidates.push(house);
            }
            return target;
        }

        // Try not including this prime.
        let mut result = divisor_sum(candidates, &primes[1..], house, target);
        let mut power = 1;
        let mut sum = 1;

        // Try increasing powers of this prime until the divisor sum exceeds the target.
        while sum < target {
            power *= primes[0];
            sum += power;

            let ds = divisor_sum(candidates, &primes[1..], house * power, target.div_ceil(sum));
            result = result.min(power * ds);
        }

        result
    }

    let target = input.div_ceil(11);
    let mut candidates = Vec::new();

    // Get list of all house numbers that meet or exceed the target value in ascending order.
    divisor_sum(&mut candidates, &PRIMES, 1, target);
    candidates.sort_unstable();

    // Find the first house taking into account the 50 present limit.
    candidates.into_iter().find(|&house| factor_sum(&PRIMES, house, 1) >= target).unwrap()
}

/// Combine prime factors into all factors, only counting those where the elf will still deliver.
fn factor_sum(primes: &[u32], house: u32, factor: u32) -> u32 {
    if primes.is_empty() {
        // Check if the elf reached this house.
        if 50 * factor >= house { factor } else { 0 }
    } else {
        (0..31)
            .map(|exp| primes[0].pow(exp))
            .take_while(|&prime_power| house.is_multiple_of(prime_power))
            .map(|prime_power| factor_sum(&primes[1..], house, factor * prime_power))
            .sum()
    }
}
