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
//! This means that each digit is the sum of itself and subsequent digits and can be computed
//! using a reverse rolling [prefix sum].
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
//! is the [tetrahedral number] sequence. More generally the `kth` coefficient of the `nth` phase
//! is the [binomial coefficient] `C(n, k)`.
//!
//! We compute the result for part two by finding the `C(100, k)` coefficient for each number using
//! the recursive method of summing previous coefficients from [Pascal's triangle]. This result in
//! complexity `O(100n)` where `n` is the number of digits after the starting index in the input.
//!
//! As a minor optimization we combine multiplying the digits of the input together with the
//! binomial coefficient calculation.
//!
//! [prefix sum]: https://en.wikipedia.org/wiki/Prefix_sum
//! [upper triangular matrix]: https://en.wikipedia.org/wiki/Triangular_matrix
//! [triangular number]: https://en.wikipedia.org/wiki/Triangular_number
//! [tetrahedral number]: https://en.wikipedia.org/wiki/Tetrahedral_number
//! [binomial coefficient]: https://en.wikipedia.org/wiki/Binomial_coefficient
//! [Pascal's triangle]: https://en.wikipedia.org/wiki/Pascal%27s_triangle
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<u8> {
    input.trim().bytes().map(u8::to_decimal).collect()
}

pub fn part1(input: &[u8]) -> i32 {
    let size = input.len();
    let mid = size / 2;
    let end = size - 1;
    let pattern = [0, 1, 0, -1];

    let mut current = &mut input.iter().copied().map(i32::from).collect::<Vec<_>>();
    let mut next = &mut vec![0; size];

    for _ in 0..100 {
        // Brute force the first half of the input.
        for i in 0..mid {
            let mut total = 0;

            // Skip the first `i` elements as the pattern is always zero.
            for j in i..size {
                total += current[j] * pattern[((j + 1) / (i + 1)) % 4];
            }

            next[i] = total.abs() % 10;
        }

        // Use a faster prefix sum approach similar to part two for the second half of the input.
        next[end] = current[end];

        for i in (mid..end).rev() {
            next[i] = (current[i] + next[i + 1]) % 10;
        }

        (current, next) = (next, current);
    }

    current.iter().take(8).fold(0, |acc, &b| 10 * acc + b)
}

pub fn part2(input: &[u8]) -> usize {
    let digits: Vec<_> = input.iter().copied().map(usize::from).collect();
    let start = fold_number(&digits[..7]);

    // This approach will only work if the index is in the second half of the input.
    let size = digits.len();
    let lower = size * 5_000;
    let upper = size * 10_000;
    assert!(lower <= start && start < upper);

    let mut current = &mut [0; 100];
    let mut next = &mut [0; 100];
    let mut result = [0; 8];

    for index in (start - 100..upper - 1).rev() {
        let offset = index + 100 - start;
        if offset < 8 {
            result[offset] = current[99];
        }

        // It's faster to turn the loop "inside out" and keep a window of the last
        // 100 coefficients pre-multiplied by the input. This means we only need a working array
        // of 100 items instead of a large subset of the input.
        next[0] = (digits[index % size] + current[0]) % 10;

        for j in 1..100 {
            next[j] = (current[j - 1] + current[j]) % 10;
        }

        (current, next) = (next, current);
    }

    fold_number(&result)
}

/// Folds a slice of digits into an integer.
fn fold_number(slice: &[usize]) -> usize {
    slice.iter().fold(0, |acc, &b| 10 * acc + b)
}
