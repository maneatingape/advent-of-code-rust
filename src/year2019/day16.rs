//! # Flawed Frequency Transmission
//!
//! ## Part One
//!
//! For each phase we first compute the prefix sum of the digits. This allows us to compute
//! the sum of any contiguous range of digits with only 2 lookups. For example the sum of the
//! 5 to 8 is `36 - 10 = 26`.
//!
//! ```none
//!                               -----------
//!     Digits:     1  2  3   4   5  6  7   8   9
//!     Prefix sum: 1  3  6 [10] 15 21 28 [36] 45
//! ```
//!
//! The complexity of each phase is the [harmonic series](https://en.wikipedia.org/wiki/Harmonic_series_(mathematics))
//! so the total complexity is `n` for the prefix sum calculation and `log(n)` for the next digits
//! for a total of `nlog(n)`.
//!
//! As a minor optimization once the phase is greater than ⅓ of the digits, then the pattern
//! simplifies to a sum of a single range. For example with 11 digits on phase 4 the pattern is:
//!
//! ```none
//!   0 0 0 1 1 1 1 0 0 0 0
//! ```
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
//! However we only need the coefficient modulo 10. [Lucas's theorem] allows us to compute binomial
//! coefficients modulo some prime number. If we compute the coefficients modulo 2 and modulo 5
//! then we can use the [Chinese remainder theorem] to find the result modulo 10.
//!
//! Two further empirical insights from [Askalski](https://www.reddit.com/user/askalski/)
//! speed up part two even more. The first is that the coefficients modulo 2 form a cycle of
//! length 128 and the coefficients of modulo 5 form a cycle of length 125. Since the digits also
//! form a cycle of length 650 then we only need to process the
//! [least common multiple](https://en.wikipedia.org/wiki/Least_common_multiple) of each cycle.
//! This is 41600 for coefficients modulo 2 and 3250 for coefficients modulo 5.
//!
//! The second insight is that both of the cycles are very sparse. Only 8 out of 128 modulo 2 values
//! and 2 out of 125 modulo 5 values respectively are non-zero. By storing the values as a
//! compressed list of `(coefficient, skip value)` we only need to process a small fraction of
//! the total digits. In total we need to compute `41600 * (8 / 128) + 3250 * (2 / 125) = 2652`
//! values per digit. This is much less than the approximately 500,000 coefficients in the
//! complete range.
//!
//! [prefix sum]: https://en.wikipedia.org/wiki/Prefix_sum
//! [upper triangular matrix]: https://en.wikipedia.org/wiki/Triangular_matrix
//! [triangular number]: https://en.wikipedia.org/wiki/Triangular_number
//! [tetrahedral number]: https://en.wikipedia.org/wiki/Tetrahedral_number
//! [binomial coefficient]: https://en.wikipedia.org/wiki/Binomial_coefficient
//! [grows rather large]: https://oeis.org/A017763/b017763.txt
//! [Lucas's theorem]: https://en.wikipedia.org/wiki/Lucas%27s_theorem
//! [Chinese remainder theorem]: https://en.wikipedia.org/wiki/Chinese_remainder_theorem
use crate::util::math::*;
use crate::util::parse::*;
use std::array::from_fn;

/// `C(n, k) % 2` This collapses to a special case of a product of only 4 possible values
/// which are cyclic with a length of 128.
///
/// * `C(0, 0) = 1`
/// * `C(1, 0) = 1`
/// * `C(1, 1) = 1`
/// * `C(0, 1) = 0`
const BINOMIAL_MOD_2: [(i32, usize); 8] =
    [(1, 4), (1, 4), (1, 4), (1, 4), (1, 4), (1, 4), (1, 4), (1, 100)];
/// `C(n, k) % 5` Cyclic with a length of 125.
const BINOMIAL_MOD_5: [(i32, usize); 2] = [(1, 25), (4, 100)];

pub fn parse(input: &str) -> Vec<i32> {
    input.trim().bytes().map(|b| b.to_decimal() as i32).collect()
}

pub fn part1(input: &[i32]) -> i32 {
    let size = input.len();
    let limit = size.div_ceil(3);

    let mut digits = input.to_vec();
    let mut prefix_sum = vec![0; size + 1];

    for _ in 0..100 {
        // Prefix sum for fast computation of arbitrary contiguous ranges.
        let mut sum = 0;

        for (i, digit) in digits.iter().enumerate() {
            sum += digit;
            prefix_sum[i + 1] = sum;
        }

        // The first third of the phases can contain the complete alternating pattern.
        for (i, digit) in digits.iter_mut().enumerate().take(limit) {
            let phase = i + 1;
            let mut total = 0;
            let mut sign = 1;

            for start in (phase - 1..size).step_by(2 * phase) {
                let end = (start + phase).min(size);
                total += sign * (prefix_sum[end] - prefix_sum[start]);
                sign *= -1;
            }

            *digit = total.abs() % 10;
        }

        // The remaining phases simplify to the sum of a single range.
        for (i, digit) in digits.iter_mut().enumerate().skip(limit) {
            let phase = i + 1;
            let start = phase - 1;
            let end = (start + phase).min(size);
            *digit = (prefix_sum[end] - prefix_sum[start]).abs() % 10;
        }
    }

    fold_decimal(&digits[..8])
}

pub fn part2(input: &[i32]) -> i32 {
    let size = input.len();
    let lower = size * 5_000;
    let upper = size * 10_000;

    // This approach will only work if the index is in the second half of the input.
    let start = fold_decimal(&input[..7]) as usize;
    assert!(lower <= start && start < upper);

    let first = compute(input, start, upper, BINOMIAL_MOD_2.iter().copied().cycle(), 128);
    let second = compute(input, start, upper, BINOMIAL_MOD_5.iter().copied().cycle(), 125);

    // Computes C(n, k) % 10
    // Solving the Chinese remainder theorem for the special case of two congruences:
    //
    //     x ​≡ a₁ (mod n₁) ​≡ a₁ (mod 2)
    //     x ​≡ a₂ (mod n₂) ≡ a₂ (mod 5)
    //     N = n₁n₂ = 10
    //     y₁ = N / n₁ = 5
    //     y₂ = N / n₂ = 2
    //     z₁ = y₁⁻¹ mod n₁ = 5⁻¹ mod 2 = 1
    //     z₂ = y₂⁻¹ mod n₂ = 2⁻¹ mod 5 = 3
    //     x ≡ a₁y₁z₁ + a₂y₂z₂ (mod 10) ≡ 5a₁ + 6a₂ (mod 10)
    //
    let result: Vec<_> = first.into_iter().zip(second).map(|(f, s)| (5 * f + 6 * s) % 10).collect();
    fold_decimal(&result)
}

/// Quickly computes a digit taking advantage of the fact
/// that the LCM is much smaller than the whole range.
fn compute<I>(input: &[i32], start: usize, upper: usize, mut nck: I, size: usize) -> [i32; 8]
where
    I: Iterator<Item = (i32, usize)>,
{
    from_fn(|offset| {
        let start = start + offset;
        let total = upper - start;

        // Compute LCM, number of complete ranges and the remaining partial range.
        let lcm = input.len().lcm(size);
        let quotient = (total / lcm) as i32;
        let remainder = total % lcm;

        // Sum partial range first.
        let mut index = start;
        let mut partial = 0;

        while index < start + remainder {
            let (coefficient, skip) = nck.next().unwrap();
            partial += input[index % input.len()] * coefficient;
            index += skip;
        }

        // Then the full range.
        let mut full = partial;

        while index < start + lcm {
            let (coefficient, skip) = nck.next().unwrap();
            full += input[index % input.len()] * coefficient;
            index += skip;
        }

        // Calculate sum for the entire range.
        quotient * full + partial
    })
}

#[inline]
fn fold_decimal(slice: &[i32]) -> i32 {
    slice.iter().fold(0, |acc, &b| 10 * acc + b)
}
