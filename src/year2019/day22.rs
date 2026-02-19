//! # Slam Shuffle
//!
//! This problem requires `i128` integers to prevent overflow!
//!
//! ## Modular Arithmetic
//!
//! To solve we'll need some knowledge of [modular arithmetic](https://en.wikipedia.org/wiki/Modular_arithmetic).
//!
//! Some basic modular identities:
//! * (a + b) mod m = (a mod m) + (b mod m)
//! * (a * b) mod m = (a mod m) * (b mod m)
//! * The [modular inverse](https://en.wikipedia.org/wiki/Modular_multiplicative_inverse)
//!   is used instead of division.
//!
//! ## Linear Congruences
//!
//! The next required insight is that each of the shuffle operation is a *linear congruence*
//! of the form:
//!
//! `Xₙ₊₁ = (aXₙ + c) mod m`
//!
//! For example "deal into new stack" which reverses the deck can be represented as:
//!
//! `Xₙ₊₁ = ((m - 1) * Xₙ + (m - 1)) mod m`
//!
//! With a deck of 3 cards this is:
//! * 0 => (2 * 0 + 2) mod 3 = 2
//! * 1 => (2 * 1 + 2) mod 3 = 1
//! * 2 => (2 * 2 + 2) mod 3 = 0
//!
//! "cut N cards" is:
//!
//! `Xₙ₊₁ = 1 * Xₙ + (m - N)) mod m`
//!
//! For example "cut 3" with a deck of 5 cards is:
//! * 0 => (1 * 0 + (5 - 3)) mod 5 = 2
//! * 1 => (1 * 1 + (5 - 3)) mod 5 = 3
//! * 2 => (1 * 2 + (5 - 3)) mod 5 = 4
//! * 3 => (1 * 3 + (5 - 3)) mod 5 = 0
//! * 4 => (1 * 3 + (5 - 3)) mod 5 = 1
//!
//! If N is negative the cut works from the end. If N is greater than m then take N mod m.
//!
//! "deal with increment N" is:
//!
//! `Xₙ₊₁ = N * Xₙ + 0) mod m`
//!
//! For example "deal with increment 3" with a deck of 5 cards is:
//! * 0 => (3 * 0 + 0) mod 5 = 0
//! * 1 => (3 * 1 + 0) mod 5 = 3
//! * 2 => (3 * 2 + 0) mod 5 = 1
//! * 3 => (3 * 3 + 0) mod 5 = 4
//! * 4 => (3 * 4 + 0) mod 5 = 2
//!
//! ## Composition
//!
//! Congruences can be composed:
//!
//! `Xₙ₊₁ = a₂ * (a₁Xₙ + c₁) + c₂) mod m = (a₁a₂Xₙ + a₂c₁ + c₂) mod m`
//!
//! so we could combine the previous "cut 3" and deal with increment 3" as:
//!
//! `Xₙ₊₁ = 3 * (1Xₙ + 2) + 0) mod m = (3Xₙ + 6) mod m`.
//!
//! This allows us to take all the input techniques and then combine them into a *single*
//! technique with the same effect, providing an efficient solution for part one.
//!
//! ## Inverse
//!
//! Part two is trickier. To find the card that ends up at index 2020 we need to find an inverse
//! congruence such that when we apply a composition to the shuffle we get the identity congruence:
//!
//! `(a₁a₂Xₙ + a₂c₁ + c₂) mod m = (Xₙ + 0) mod m`
//!
//! This implies that `a₁a₂ mod m = 1` which is the definition of the modular inverse`a₂ = a₁⁻¹`.
//!
//! The constant term `(a₂c₁ + c₂) mod m = 0` implies `c₂ = m - a₂c₁`.
//!
//! ## Exponentiation
//!
//! To find of inverse of 101741582076661 shuffles we need to raise to our inverse to the same
//! power. Let's look at the first few powers
//! * ax + c
//! * a * (ax + c) + c = a²x + ac + c
//! * a * (a²x + ac + c) = a³x + a²c + ac + c
//! * a * (a³x + a²c + ac + c) = a⁴x + a³c + a²c + ac + c
//!
//! We notice that the constant terms are the sum of a [geometric series](https://en.wikipedia.org/wiki/Geometric_series)
//! which is given by the closed-form formula:
//!
//! `cₙ = c(1 - aⁿ)/(1 - a)`
//!
//! Multiplying both sides by -1 and remembering that modular division is the multiplicative
//! inverse yields:
//!
//! `cₙ = c(aⁿ - 1)((a - 1)⁻¹) mod m`
//!
//! We can then raise a congruence to any power, using only one modular exponentiation and
//! one modular inverse, allowing us to solve part two efficiently.
//!
//! `Xₙ₊₁ = (aⁿXₙ + c(aⁿ - 1)((a - 1)⁻¹)) mod m`
//! 
//! ## Why Const Generics?
//!
//! The deck size `m` is known at compile time for each part:
//! * Part 1 uses `m = 10007`
//! * Part 2 uses `m = 119315717514047`
//!
//! All arithmetic in this problem is performed modulo `m`, and every shuffle
//! technique is parameterized by that same modulus. Instead of storing `m`
//! as a runtime field inside the struct, we make it a **const generic
//! parameter**: `Technique<const M: i128>`.
//!
//! This has several advantages:
//!
//! * The modulus becomes part of the *type*, preventing accidental mixing
//!   of techniques built with different deck sizes.
//! * No runtime storage of `m` is required.
//! * The compiler can treat `M` as a true constant, enabling better
//!   optimization and inlining of modular arithmetic.
//! * It makes the mathematical intent explicit: a `Technique<M>` is a
//!   linear congruence defined over ℤ/Mℤ.
//!
//! In short, the modulus is a compile-time property of the shuffle,
//! not a runtime variable.
use crate::util::math::*;
use crate::util::parse::*;

struct Technique<const M: i128> {
    a: i128,
    c: i128,
}

impl<const M: i128> Technique<M> {
    fn compose(&self, other: &Technique<M>) -> Technique<M> {
        let a = (self.a * other.a) % M;
        let c = (self.c * other.a + other.c) % M;
        Technique::<M> { a, c }
    }

    fn inverse(&self) -> Technique<M> {
        let a = self.a.mod_inv(M).unwrap();
        let c = M - (a * self.c) % M;
        Technique::<M> { a, c }
    }

    fn power(&self, e: i128) -> Technique<M> {
        let a = self.a.mod_pow(e, M);
        let c = (((a - 1) * (self.a - 1).mod_inv(M).unwrap() % M) * self.c) % M;
        Technique::<M> { a, c }
    }

    fn shuffle(&self, index: i128) -> i128 {
        (self.a * index + self.c) % M
    }
}

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> i128 {
    deck::<10007>(input).shuffle(2019)
}

pub fn part2(input: &str) -> i128 {
    deck::<119315717514047>(input).inverse().power(101741582076661).shuffle(2020)
}

fn deck<const M: i128>(input: &str) -> Technique<M> {
    input
        .lines()
        .map(|line| {
            if line.ends_with("stack") {
                Technique::<M> { a: M - 1, c: M - 1 }
            } else if line.starts_with("cut") {
                let n: i128 = line.signed();
                let c = (M - n % M) % M;
                Technique::<M> { a: 1, c }
            } else {
                let n: i128 = line.signed();
                let a = (M + n % M) % M;
                Technique::<M> { a, c: 0 }
            }
        })
        .reduce(|a, b| a.compose(&b))
        .unwrap()
}
