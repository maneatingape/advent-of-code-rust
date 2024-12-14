//! Extended mathematical operations.
//!
//! * [Greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
//!   of 2 numbers using the
//!   [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm).
//!
//! * [Least common multiple](https://en.wikipedia.org/wiki/Least_common_multiple)
//!
//! * [Modular exponentation](https://en.wikipedia.org/wiki/Modular_exponentiation).
//!   Calculates bᵉ mod m efficiently using
//!   [exponentiation by squaring](https://en.wikipedia.org/wiki/Exponentiation_by_squaring).
//!
//! * [Modular multiplicative inverse](https://en.wikipedia.org/wiki/Modular_multiplicative_inverse)
//!   calculated using the [extended Euclidean algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm).
//!
//! * [Integer square root](https://en.wikipedia.org/wiki/Integer_square_root).
use crate::util::integer::*;

pub trait IntegerMathOps<T: Integer<T>> {
    fn gcd(self, b: T) -> T;
    fn lcm(self, b: T) -> T;
    fn mod_pow(self, e: T, m: T) -> T;
}

pub trait UnsignedMathOps<T: Unsigned<T>> {
    fn sqrt(self) -> T;
}

pub trait SignedMathOps<T: Signed<T>> {
    fn mod_inv(self, m: T) -> Option<T>;
}

impl<T: Integer<T>> IntegerMathOps<T> for T {
    /// Greatest common divisor
    fn gcd(self, mut b: T) -> T {
        let mut a = self;

        while b != T::ZERO {
            (a, b) = (b, a % b);
        }

        a
    }

    // Least common multiple
    fn lcm(self, b: T) -> T {
        self * (b / self.gcd(b))
    }

    // Modular exponentation
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
}

impl<T: Unsigned<T>> UnsignedMathOps<T> for T {
    // Integer square root. Once [`isqrt`] is stablized then this function can be removed.
    fn sqrt(self) -> T {
        let mut bit = T::ONE << (self.ilog2() >> T::ONE);
        let mut root = bit;

        while bit > T::ONE {
            bit = bit >> T::ONE;
            let next = root | bit;
            if next * next <= self {
                root = next;
            }
        }

        root
    }
}

impl<T: Signed<T>> SignedMathOps<T> for T {
    // Modular multiplicative inverse
    fn mod_inv(self, m: T) -> Option<T> {
        let mut t = T::ZERO;
        let mut newt = T::ONE;
        let mut r = m;
        let mut newr = self;

        while newr != T::ZERO {
            let quotient = r / newr;
            (t, newt) = (newt, t - quotient * newt);
            (r, newr) = (newr, r - quotient * newr);
        }

        if r > T::ONE {
            return None;
        }
        if t < T::ZERO {
            t = t + m;
        }

        Some(t)
    }
}
