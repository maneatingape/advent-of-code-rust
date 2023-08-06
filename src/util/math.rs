//! Extended mathematical operations.
//!
//! * Greatest common divisor
//! * Modular exponentation
//! * Modular inverse
pub trait MathOps {
    fn gcd(&self, b: u64) -> u64;
    fn mod_pow(&self, e: u64, m: u64) -> u64;
    fn mod_inv(&self, m: u64) -> u64;
}

impl MathOps for u64 {
    /// Greatest common divisor of 2 numbers using the
    /// [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm).
    fn gcd(&self, mut b: u64) -> u64 {
        let mut a = *self;

        while b != 0 {
            (a, b) = (b, a % b);
        }

        a
    }

    /// Calculates bᵉ mod m efficiently using
    /// [exponentiation by squaring](https://en.wikipedia.org/wiki/Exponentiation_by_squaring).
    fn mod_pow(&self, mut e: u64, m: u64) -> u64 {
        let mut b = *self;
        let mut c = 1;

        while e > 0 {
            if e & 1 == 1 {
                c = (c * b) % m;
            }
            b = (b * b) % m;
            e >>= 1;
        }

        c
    }

    /// [Modular multiplicative inverse](https://en.wikipedia.org/wiki/Modular_multiplicative_inverse)
    /// calculated using the [extended Euclidean algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm).
    fn mod_inv(&self, m: u64) -> u64 {
        let mut t = 0;
        let mut newt = 1;
        let mut r = m as i64;
        let mut newr = *self as i64;

        while newr != 0 {
            let quotient = r / newr;
            (t, newt) = (newt, t - quotient * newt);
            (r, newr) = (newr, r - quotient * newr);
        }

        if t < 0 {
            t += m as i64;
        }
        t as u64
    }
}
