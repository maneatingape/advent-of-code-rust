//! Extended mathematical operations.
use std::ops::Rem;

/// Greatest common divisor of 2 numbers using the
/// [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm).
pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy,
    T: Eq,
    T: Default,
    T: Rem + Rem<Output = T>,
{
    let mut a = a;
    let mut b = b;

    while b != Default::default() {
        (a, b) = (b, a.rem(b));
    }

    a
}
