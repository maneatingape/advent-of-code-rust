//! # Full of Hot Air
//!
//! The SNAFU numbers are balanced quinary, similar to
//! [an actual base](https://en.wikipedia.org/wiki/Balanced_ternary)
//! used by some experimental computers.
pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> String {
    to_snafu(input.iter().map(from_snafu).sum())
}

pub fn part2(_input: &[&str]) -> &'static str {
    "n/a"
}

/// Converting from SNAFU to decimal is straightforward.
fn from_snafu(snafu: &&str) -> i64 {
    snafu.bytes().fold(0, |acc, c| {
        let digit = match c {
            b'=' => -2,
            b'-' => -1,
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            _ => unreachable!(),
        };
        5 * acc + digit
    })
}

/// Convert to decimal by first finding the result modulus 5 for each digit.
/// If the answer is 3 or 4 then we must add a carry to the next digit so account for the
/// subtraction.
fn to_snafu(mut n: i64) -> String {
    let mut digits = String::new();

    while n > 0 {
        let next = match n % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => unreachable!(),
        };
        digits.insert(0, next);
        // If the remainder of n is 3 or higher then this will add a carry digit to account
        // for the subtraction.
        n = (n + 2) / 5;
    }

    digits
}
