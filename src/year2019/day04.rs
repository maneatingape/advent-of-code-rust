//! # Secure Container
//!
//! We speed things up by only checking numbers that have digits in non-decreasing order for pairs.
//! These numbers become rapidly less dense as the password value increases and there
//! are only 3003 total of these numbers with 6 digits.
use crate::util::parse::*;
use crate::util::slice::*;

pub fn parse(input: &str) -> Vec<u32> {
    input.iter_unsigned().collect()
}

/// Password must contain at least one pair.
pub fn part1(input: &[u32]) -> u32 {
    let predicate = |first: bool, second: bool, third: bool, fourth: bool, fifth: bool| {
        first || second || third || fourth || fifth
    };
    passwords(input, predicate)
}

/// Password must contain at least one pair that's not part of a larger group.
pub fn part2(input: &[u32]) -> u32 {
    let predicate = |first: bool, second: bool, third: bool, fourth: bool, fifth: bool| {
        (first && !second)
            || (!first && second && !third)
            || (!second && third && !fourth)
            || (!third && fourth && !fifth)
            || (!fourth && fifth)
    };
    passwords(input, predicate)
}

fn passwords(input: &[u32], predicate: impl Fn(bool, bool, bool, bool, bool) -> bool) -> u32 {
    let start = input[0];
    let end = input[1];

    // Split into six digits.
    let mut digits = [
        start / 100000,
        (start / 10000) % 10,
        (start / 1000) % 10,
        (start / 100) % 10,
        (start / 10) % 10,
        start % 10,
    ];

    // Increase the starting number to the next number that has all digits in non-decreasing order
    // to ensure that the incrementing logic in the search loop works correctly.
    // For example 223450 => 223455, 120568 => 122222 and 439999 => 444444.
    for i in 1..6 {
        if digits[i] < digits[i - 1] {
            for j in i..6 {
                digits[j] = digits[i - 1];
            }
            break;
        }
    }

    let mut n = 0;
    let mut count = 0;

    while n <= end {
        // Check current number
        let first = digits[0] == digits[1];
        let second = digits[1] == digits[2];
        let third = digits[2] == digits[3];
        let fourth = digits[3] == digits[4];
        let fifth = digits[4] == digits[5];

        if predicate(first, second, third, fourth, fifth) {
            count += 1;
        }

        // Find the next number with all digits in non-decreasing order.
        let mut i = 5;
        while digits[i] == 9 {
            i -= 1;
        }

        let next = digits[i] + 1;
        while i <= 5 {
            digits[i] = next;
            i += 1;
        }

        // Convert number to `u32`.
        n = digits.fold_decimal();
    }

    count
}
