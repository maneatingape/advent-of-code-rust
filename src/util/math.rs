//! Extended mathematical operations.
//!
//! * [Greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor) of 2 numbers using
//!   the [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm).
//!
//! * [Least common multiple](https://en.wikipedia.org/wiki/Least_common_multiple)
//!
//! * [Modular exponentiation](https://en.wikipedia.org/wiki/Modular_exponentiation). Calculates bᵉ mod
//!   m efficiently using [exponentiation by squaring](https://en.wikipedia.org/wiki/Exponentiation_by_squaring).
use crate::util::integer::*;

pub trait MathOps<T: Integer> {
    #[must_use]
    fn gcd(self, b: T) -> T;
    #[must_use]
    fn lcm(self, b: T) -> T;
    #[must_use]
    fn mod_pow(self, e: T, m: T) -> T;
}

impl<T: Integer> MathOps<T> for T {
    /// Greatest common divisor.
    #[inline]
    fn gcd(self, mut b: T) -> T {
        let mut a = self;

        while b != T::ZERO {
            (a, b) = (b, a % b);
        }

        a
    }

    /// Least common multiple.
    #[inline]
    fn lcm(self, b: T) -> T {
        self * (b / self.gcd(b))
    }

    /// Modular exponentiation.
    #[inline]
    fn mod_pow(self, mut e: T, m: T) -> T {
        let mut base = self;
        let mut result = T::ONE;

        while e > T::ZERO {
            if e & T::ONE == T::ONE {
                result = (result * base) % m;
            }
            base = (base * base) % m;
            e = e >> 1;
        }

        result
    }
}
