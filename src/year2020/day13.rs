//! # Shuttle Search
//!
//! Part two is the [Chinese Remainder Theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem).
//! The integers n₁, n₂, ... nₖ map to the bus ids which happen to be prime. This satisfies the
//! requirement that the integers are [pairwise coprime](https://en.wikipedia.org/wiki/Coprime_integers#Coprimality_in_sets).
//!
//! This is similar to [`Year 2016 Day 15`]. However, in that year, the primes were all small,
//! so that the number of iterations to solve by sieving was less effort than performing
//! modular multiplication. For today's puzzle, however, a couple of primes are three digits,
//! such that the number of iterations for sieving outweighs the effort for performing modular
//! multiplications, especially when we can use `u128` for intermediate products to avoid
//! numeric overflow.
//!
//! [`Year 2016 Day 15`]: crate::year2016::day15
use crate::util::parse::*;

pub struct Input {
    timestamp: usize,
    buses: Vec<(usize, usize)>,
}

pub fn parse(input: &str) -> Input {
    let (first, second) = input.split_once('\n').unwrap();
    let timestamp = first.unsigned();
    let buses = second
        .split(',')
        .enumerate()
        .filter(|&(_, id)| id != "x")
        .map(|(offset, id)| (offset, id.unsigned()))
        .collect();
    Input { timestamp, buses }
}

pub fn part1(input: &Input) -> usize {
    let (id, next) = input
        .buses
        .iter()
        .map(|(_, id)| (id, id - input.timestamp % id))
        .min_by_key(|&(_, next)| next)
        .unwrap();

    id * next
}

pub fn part2(input: &Input) -> usize {
    let (mut time, mut step) = input.buses[0];

    for (offset, id) in &input.buses[1..] {
        // Scale id by enough to ensure a positive goal, then solve for x using the Chinese
        // Remainder Theorem.
        //     x ≡ a₁ (mod n₁) ≡ time (mod step)
        //     x ≡ a₂ (mod n₂) ≡ goal (mod id)
        //     N = n₁n₂ = product
        let goal = (1000 * id - *offset) % id;
        let product = id * step;

        // Use extended Euclidean division to obtain the respective modular inverses.
        //     z₁ = n₂⁻¹ mod n₁
        //     z₂ = n₁⁻¹ mod n₂
        let (z1, z2) = extended_euclid(step, *id);

        // Combine the two constituent parts. For this problem, time, step, z1, and z2 can reach
        // around 50 bits, while id and goal are less than 10.
        //     x ≡ a₁n₂z₁ + a₂n₁z₂ (mod n₁n₂)
        let rem1 = modular_multiply(time, z2 * *id, product);
        let rem2 = modular_multiply(goal * z1, step, product);
        (time, step) = ((rem1 + rem2) % product, product);
    }

    time
}

// Given two coprime numbers, return their corresponding positive modular inverses using
// extended Euclidean division to determine Bézout coefficients.
fn extended_euclid(a: usize, b: usize) -> (usize, usize) {
    let mut r1 = a as i64;
    let mut r2 = b as i64;
    let (mut m, mut s) = (1, 0);
    let (mut n, mut t) = (0, 1);
    while r2 != 0 {
        let q = r1 / r2;
        (r1, r2) = (r2, r1 - q * r2);
        (m, s) = (s, m - q * s);
        (n, t) = (t, n - q * t);
    }
    if m < 0 {
        m += (a * b) as i64;
    }
    if n < 0 {
        n += (a * b) as i64;
    }
    (m as usize, n as usize)
}

fn modular_multiply(a: usize, b: usize, modulus: usize) -> usize {
    let product = (a as u128) * (b as u128);
    (product % (modulus as u128)) as usize
}
