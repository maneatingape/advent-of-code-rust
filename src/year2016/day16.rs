//! # Dragon Checksum
//!
//! We solve efficiently with a key insight that the checksum is simply the
//! [odd parity bit](https://en.wikipedia.org/wiki/Parity_bit) for each block. If the total number
//! of ones is even then the result is one, if the total is odd then the result is zero.
//!
//! This means that only the *total number of ones is important* not the pattern itself. Each
//! checksum bit is computed over the largest power of two divisible into the output size. For part
//! one this is 2⁴ or 16 and for part this is 2²¹ or 2097152. If we can calculate the number of
//! ones for any arbitrary length then we can find the number at the start and end of each block,
//! subtract from each other to get the total in the range then find the checksum bit.
//!
//! We find the number of ones for a pattern of length `n` in `log(n)` complexity as follows:
//! * Start with a known pattern `abcde` and let the reversed bit inverse of this pattern be
//!   `EDCBA`.
//! * Calculate the [prefix sum](https://en.wikipedia.org/wiki/Prefix_sum) of the known sequence.
//! * If the requested length is within the known sequence (in this example from 0 to 5 inclusive)
//!   then we're done, return the number of ones directly.
//! * Else after one repetition this becomes `abcde0EDCBA`.
//! * If the length is at or to the right of the middle `0`,
//!   for example `length` is 8 then the number of ones is:
//!    * Let `half` = 5 the length of the left hand known sequence.
//!    * Let `full` = 11 the length of the entire sequence.
//!    * Ones in `abcde` => x
//!    * Ones in `EDCBA` => the number of zeroes in `abcde`
//!      => 5 - x => half - x
//!    * Ones in `abc` => y
//!    * Ones in `CBA` => the number of zeroes in `abc`
//!      => 3 - y => 11 - 8 - y => full - length - y => next - y
//!    * The total number of ones in `abcde0ED` is
//!      x + (half - x) - (next - y) => half - next + y
//!
//! Now for the really neat part. We can recursively find the number of ones in `y` by repeating
//! the same process by setting the new `length` to `next`. We keep recursing until the length
//! is less the size of the initial input and we can lookup the final count from the prefix sum.
//!
//! Note that it is also possible to compute the parity of any prefix of the Dragon Curve in
//! O(1) time; the formula is available on [OEIS A255070](https://oeis.org/A255070), and there
//! are a [couple](https://www.reddit.com/r/adventofcode/comments/5ititq/2016_day_16_c_how_to_tame_your_dragon_in_under_a/)
//! of [posts](https://www.reddit.com/r/adventofcode/comments/1r642oc/2016_day_16_in_review_dragon_checksum/)
//! showing how to utilize that approach.  However, the logarithmic solution shown here is
//! fast enough to not need to worry about askalski's comment "I have no idea why it works,
//! only that it does work."
use crate::util::parse::*;

/// Build a prefix sum of the number of ones at each length in the pattern
/// including zero at the start.
pub fn parse(input: &str) -> Vec<usize> {
    let mut sum = 0;
    let mut ones = vec![0];

    for b in input.trim().bytes() {
        sum += b.to_decimal() as usize;
        ones.push(sum);
    }

    ones
}

/// 272 is 17 * 2⁴
pub fn part1(input: &[usize]) -> String {
    checksum(input, 272)
}

/// 35651584 is 17 * 2²¹
pub fn part2(input: &[usize]) -> String {
    checksum(input, 35651584)
}

/// Collect the ones count at each `step_size` then subtract in pairs to calculate the number of
/// ones in each interval to give the checksum.
pub fn checksum(input: &[usize], disk_size: usize) -> String {
    // Determine how many blocks and how big each one is, by lowest 1-bit in disk_size
    let step_size = disk_size & (!disk_size + 1);
    let blocks = disk_size / step_size;

    let counts: Vec<_> = (0..blocks + 1).map(|i| count(input, i * step_size)).collect();
    counts.windows(2).map(|w| if (w[1] - w[0]) % 2 == 0 { '1' } else { '0' }).collect()
}

/// Counts the number of ones from the start to the index (inclusive).
fn count(ones: &[usize], mut length: usize) -> usize {
    let mut half = ones.len() - 1;
    let mut full = 2 * half + 1;

    // Find the smallest pattern size such that the index is on the right hand side
    // (greater than or to) the middle `0` character.
    while full < length {
        half = full;
        full = 2 * half + 1;
    }

    let mut result = 0;

    while length >= ones.len() {
        // Shrink the pattern size until the index is on the right side once more.
        while length <= half {
            half /= 2;
            full /= 2;
        }

        // "Reflect" the index then add the extra number of ones to the running total.
        let next = full - length;
        result += half - next;
        length = next;
    }

    result + ones[length]
}
