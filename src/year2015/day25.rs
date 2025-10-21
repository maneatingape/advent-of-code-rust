//! # Let It Snow
//!
//! There are two parts to solving this problem.
//!
//! The first is converting the row and column to a *zero-based* index. Using the example of
//! the 12th code at row 4 column 2:
//!
//! ```none
//!        | 1   2   3   4   5   6
//!     ---+---+---+---+---+---+---+
//!      1 |  1   3   6  10  15  21
//!      2 |  2   5   9  14  20
//!      3 |  4   8  13  19
//!      4 |  7  12  18
//!      5 | 11  17
//!      6 | 16
//! ```
//!
//! First we observe that the numbers on the top row are the
//! [triangular numbers](https://en.wikipedia.org/wiki/Triangular_number) that can be calculated
//! with the formula `(n * (n + 1)) / 2` for the `nth` number.
//!
//! Starting at the chosen number 12 and moving diagonally upwards to the right we intersect
//! the top row at column `column + row - 1 = 2 + 4 - 1 = 5`. This gives the triangular number
//! `5 * (5 + 1) / 2 = 15`. Then we count backward by `row` elements to get the one less
//! zero-based index `15 - 4 = 11`.
//!
//! The second part is realizing that the description of the code generation is
//! [modular exponentiation](https://en.wikipedia.org/wiki/Modular_exponentiation). The exponent
//! of the first code is zero, which is the reason for using a zero-based index.
use crate::util::iter::*;
use crate::util::math::*;
use crate::util::parse::*;

type Input = [u64; 2];

pub fn parse(input: &str) -> Input {
    input.iter_unsigned().chunk::<2>().next().unwrap()
}

pub fn part1(input: &Input) -> u64 {
    let [row, column] = *input;

    let n = column + row - 1;
    let triangle = (n * (n + 1)) / 2;
    let index = triangle - row;

    (20151125 * 252533.mod_pow(index, 33554393)) % 33554393
}

pub fn part2(_input: &Input) -> &'static str {
    "n/a"
}
