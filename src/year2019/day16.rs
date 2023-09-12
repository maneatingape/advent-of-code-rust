//! # Flawed Frequency Transmission
//!
//! ## Part One
//!
//! For each phase we split the input into two halves. The first half is processed using the
//! pattern "stretched" for the current phase. For the second half we notice that
//! the coefficients before are always zero and after always one, so the result can only depend
//! on later digits. Using the first example:
//!
//! ```none
//!     5*1  + 6*1  + 7*1  + 8*1
//!     5*0  + 6*1  + 7*1  + 8*1
//!     5*0  + 6*0  + 7*1  + 8*1
//!     5*0  + 6*0  + 7*0  + 8*1
//! ```
//!
//! Each digit is the sum of itself and subsequent digits and can be computed using a reverse
//! rolling [prefix sum].
//!
//! ## Part Two
//!
//! If the index from the first 7 digits lies in the second half of the input then we only need to
//! consider coefficients that form an [upper triangular matrix], for example:
//!
//! ```none
//!   1  1  1  1
//!   0  1  1  1
//!   0  0  1  1
//!   0  0  0  1
//! ```
//!
//! After the first phase:
//!
//! ```none
//!   1  2  3  4
//!   0  1  2  3
//!   0  0  1  2
//!   0  0  0  1
//! ```
//!
//! After the second phase:
//!
//! ```none
//!   1  3  6 10
//!   0  1  3  6
//!   0  0  1  3
//!   0  0  0  1
//! ```
//!
//! After the third phase:
//!
//! ```none
//!   1  4 10 20
//!   0  1  4 10
//!   0  0  1  6
//!   0  0  0  1
//! ```
//!
//! We can see that the third phase is the [triangular number] sequence and that the fourth phase
//! is the [tetrahedral number] sequence. More generally the `i`th coefficient of the 100th phase
//! is the [binomial coefficient] `(i + 99, i)`.
//!
//! We could compute the coefficient using the formula `nᵏ/k!` however this [grows rather large]
//! and quickly will overflow even a `u128`.
//!
//! However we only need to coefficient modulo 10. [Lucas's theorem] allow us to computer binomial
//! coefficients modulo some prime number. If we compute the coefficients modulo 2 and modulo 5
//! then we can use the [Chinese remainder theorem] to find the result modulo 10.
//!
//! [prefix sum]: https://en.wikipedia.org/wiki/Prefix_sum
//! [upper triangular matrix]: https://en.wikipedia.org/wiki/Triangular_matrix
//! [triangular number]: https://en.wikipedia.org/wiki/Triangular_number
//! [tetrahedral number]: https://en.wikipedia.org/wiki/Tetrahedral_number
//! [binomial coefficient]: https://en.wikipedia.org/wiki/Binomial_coefficient
//! [grows rather large]: https://oeis.org/A017763/b017763.txt
//! [Lucas's theorem]: https://en.wikipedia.org/wiki/Lucas%27s_theorem
//! [Chinese remainder theorem]: https://en.wikipedia.org/wiki/Chinese_remainder_theorem
use crate::util::integer::*;
use crate::util::parse::*;

/// Lookup table for first five rows of
/// [Pascal's triangle](https://en.wikipedia.org/wiki/Pascal%27s_triangle).
/// A convention specifically for Lukas's theorem is that if `n` < `k` then the value is 0.
const PASCALS_TRIANGLE: [[usize; 5]; 5] =
    [[1, 0, 0, 0, 0], [1, 1, 0, 0, 0], [1, 2, 1, 0, 0], [1, 3, 3, 1, 0], [1, 4, 6, 4, 1]];

pub fn parse(input: &str) -> Vec<u8> {
    input.trim().bytes().map(u8::to_decimal).collect()
}

pub fn part1(input: &[u8]) -> i32 {
    let size = input.len();
    let mid = size / 2;
    let end = size - 1;

    let mut current = &mut input.iter().copied().map(i32::from).collect::<Vec<_>>();
    let mut next = &mut vec![0; size];

    for _ in 0..100 {
        // Brute force the first half of the input.
        for i in 0..mid {
            let phase = i + 1;
            let skip = 2 * phase;
            let mut remaining = &current[i..];
            let mut total: i32 = 0;

            while !remaining.is_empty() {
                let take = phase.min(remaining.len());
                total += &remaining[..take].iter().sum();

                if remaining.len() <= skip {
                    break;
                }
                remaining = &remaining[skip..];

                let take = phase.min(remaining.len());
                total -= &remaining[..take].iter().sum();

                if remaining.len() <= skip {
                    break;
                }
                remaining = &remaining[skip..];
            }

            next[i] = total.abs() % 10;
        }

        // Use a faster reverse prefix sum approach for the second half of the input.
        next[end] = current[end];

        for i in (mid..end).rev() {
            next[i] = (current[i] + next[i + 1]) % 10;
        }

        (current, next) = (next, current);
    }

    fold_number(&current[..8])
}

pub fn part2(input: &[u8]) -> usize {
    let digits: Vec<_> = input.iter().copied().map(usize::from).collect();
    let start = fold_number(&digits[..7]);

    // This approach will only work if the index is in the second half of the input.
    let size = digits.len();
    let lower = size * 5_000;
    let upper = size * 10_000;
    assert!(lower <= start && start < upper);

    let mut coefficients = [0; 8];
    let mut result = [0; 8];

    for (k, index) in (start..upper).enumerate() {
        coefficients.rotate_right(1);
        coefficients[0] = binomial_mod_10(k + 99, k);

        let next = digits[index % size];
        result.iter_mut().zip(coefficients).for_each(|(r, c)| *r += next * c);
    }

    result.iter_mut().for_each(|r| *r %= 10);
    fold_number(&result)
}

/// Computes C(n, k) % 2
///
/// This collapses to a special case of a product of only 4 possible values:
///
/// * `C(0, 0) = 1`
/// * `C(1, 0) = 1`
/// * `C(1, 1) = 1`
/// * `C(0, 1) = 0`
///
/// So the final value will always be one or zero. The fourth zero case happens when `k` has a
/// bit not present in `n` so we can compute the final value using bitwise logic.
#[inline]
fn binomial_mod_2(n: usize, k: usize) -> usize {
    (k & !n == 0) as usize
}

/// Computes C(n, k) % 5
///
/// If `k` is zero then the remaining coefficients are 1 so we can exit early.
/// If `r` is zero then the total result is also zero so we can exit early.
/// To save some time we only take the result modulo 5 at the end.
#[inline]
fn bimonial_mod_5(mut n: usize, mut k: usize) -> usize {
    let mut r = 1;

    while k > 0 && r > 0 {
        r *= PASCALS_TRIANGLE[n % 5][k % 5];
        n /= 5;
        k /= 5;
    }

    r % 5
}

/// Computes C(n, k) % 10
///
/// Solving the Chinese remainder theorem for the special case of two congruences:
///
/// ```none
///     x ​≡ a₁ (mod n₁) ​≡ a₁ (mod 2)
///     x ​≡ a₂ (mod n₂) ≡ a₂ (mod 5)
///     N = n₁n₂ = 10
///     y₁ = N / n₁ = 5
///     y₂ = N / n₂ = 2
///     z₁ = y₁⁻¹ mod n₁ = 5⁻¹ mod 2 = 1
///     z₂ = y₂⁻¹ mod n₂ = 2⁻¹ mod 5 = 3
///     x ≡ a₁y₁z₁ + a₂y₂z₂ (mod 10) ≡ 5a₁ + 6a₂ (mod 10)
/// ```
#[inline]
fn binomial_mod_10(n: usize, k: usize) -> usize {
    5 * binomial_mod_2(n, k) + 6 * bimonial_mod_5(n, k)
}

/// Folds a slice of digits into an integer.
#[inline]
fn fold_number<T: Integer<T>>(slice: &[T]) -> T {
    slice.iter().fold(T::ZERO, |acc, &b| T::TEN * acc + b)
}
