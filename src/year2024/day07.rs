//! # Bridge Repair
//!
//! The equations can be validated using a recursive solution that checks all possible combinations
//! of operators. However the number of states to check grows exponentially as 2ⁿ in part one
//! and 3ⁿ in part two.
//!
//! As much faster approach works in reverse from the end of the equation to prune as many states
//! as possible by checking which operations are possible. For example:
//!
//! ```none
//!     Test Value: 123456
//!     Equation: 2 617 56
//!     Addition is possible as 123456 >= 56
//!     Multiplication is not possible as 56 is not a factor of 123456
//!     Concatenation is possible as 1234 || 56 = 123456
//! ```
//!
//! Following the concatenation branch and applying an inverse concentation
//! (the full solution would also follow the addition branch):
//!
//! ```none
//!     Test Value: 1234
//!     Equation: 2 617
//!     Addition is possible as 1234 >= 617
//!     Multiplication is possible as 2 * 617 = 1234
//!     Concatenation is not possible as 1234 does not end in 617
//! ```
//!
//! Following the multiplication branch:
//!
//! ```none
//!     Test Value: 2
//!     Equation: 2
//!     Addition is possible
//!     Multiplication is possible
//!     Concatenation is possible
//! ```
//!
//! Following the addition or concatentation branches results in a test value of 0 which means
//! that all terms have been applied successfully and the equation is valid.
//!
//! Inverse concenation can be implemented without time consuming conversion to or from
//! strings by dividing the left term by the next power of ten greater than the right term.
//! For example:
//!
//! * 7 || 9 => 79 => 79 / 10 => 7
//! * 12 || 34 => 1234 => 1234 / 100 => 12
//! * 123 || 789 => 123789 => 123789 / 1000 => 123
use crate::util::parse::*;

type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let mut equation = Vec::new();
    let mut part_one = 0;
    let mut part_two = 0;

    for line in input.lines() {
        equation.extend(line.iter_unsigned::<u64>());

        // If an equation is valid for part one then it's also valid for part two.
        if valid(&equation, equation[0], equation.len() - 1, false) {
            part_one += equation[0];
            part_two += equation[0];
        } else if valid(&equation, equation[0], equation.len() - 1, true) {
            part_two += equation[0];
        }

        equation.clear();
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}

fn valid(terms: &[u64], test_value: u64, index: usize, concat: bool) -> bool {
    if test_value == 0 {
        return index == 0;
    }

    if index == 0 {
        return false;
    }

    (concat
        && test_value % next_power_of_ten(terms[index]) == terms[index]
        && valid(terms, test_value / next_power_of_ten(terms[index]), index - 1, concat))
        || (test_value % terms[index] == 0
            && valid(terms, test_value / terms[index], index - 1, concat))
        || (test_value >= terms[index]
            && valid(terms, test_value - terms[index], index - 1, concat))
}

fn next_power_of_ten(n: u64) -> u64 {
    let mut power = 10;

    while power <= n {
        power *= 10;
    }

    power
}
