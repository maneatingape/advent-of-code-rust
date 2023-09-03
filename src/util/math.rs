//! Extended mathematical operations.
//!
//! * Greatest common divisor
//! * Modular exponentation
//! * Modular inverse
use crate::util::integer::*;

pub trait MathOps<T: Integer<T>> {
    fn gcd(self, b: T) -> T;
    fn mod_pow(self, e: T, m: T) -> T;
    fn mod_inv(self, m: T) -> T;
}

impl<T: Integer<T>> MathOps<T> for T {
    /// Greatest common divisor of 2 numbers using the
    /// [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm).
    fn gcd(self, mut b: T) -> T {
        let mut a = self;

        while b != T::ZERO {
            (a, b) = (b, a % b);
        }

        a
    }

    /// Calculates bᵉ mod m efficiently using
    /// [exponentiation by squaring](https://en.wikipedia.org/wiki/Exponentiation_by_squaring).
    fn mod_pow(self, mut e: T, m: T) -> T {
        let mut b = self;
        let mut c = T::ONE;

        while e > T::ZERO {
            if e & T::ONE == T::ONE {
                c = (c * b) % m;
            }
            b = (b * b) % m;
            e = e >> T::ONE;
        }

        c
    }

    /// [Modular multiplicative inverse](https://en.wikipedia.org/wiki/Modular_multiplicative_inverse)
    /// calculated using the [extended Euclidean algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm).
    fn mod_inv(self, m: T) -> T {
        let mut t = T::ZERO;
        let mut newt = T::ONE;
        let mut r = m;
        let mut newr = self;

        while newr != T::ZERO {
            let quotient = r / newr;
            (t, newt) = (newt, t - quotient * newt);
            (r, newr) = (newr, r - quotient * newr);
        }

        if t < T::ZERO {
            t = t + m;
        }
        t
    }
}
