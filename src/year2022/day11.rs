//! # Monkey in the Middle
//!
//! This problem is the combination of 2 AoC classics, extracting numbers from a wall of
//! flavor text and modular arithmetic. For the first problem, our utility [`iter_unsigned`] method
//! comes in handy.
//!
//! For the second problem, the key insight for part 2 is that
//! `a % m` is the same as `(a % n) % m` if `m` is a factor of `n`.
//!
//! For example:
//! ```none
//!   a = 23
//!   m = 3
//!   n = 15
//!   23 % 3 = 2
//!   23 % 15 = 8
//!   8 % 3 = 2
//! ```
//!
//! To keep the worry level manageable we need to find a number such that each monkey's test is a
//! factor of that number. The smallest number that meets this criteria is the
//! [least common multiple](https://en.wikipedia.org/wiki/Least_common_multiple).
//!
//! However before you rush off to implement the LCM algorithm, it's worth examining the input. Each
//! monkey's test number is prime, so in this specific case the LCM is simply the product of all
//! monkey's test numbers.

//! For example if we also need to test modulo 5 then the previous factor of 15 will work for both
//! 3 and 5.
//!
//! ```none
//!   a = 23
//!   m = 5
//!   n = 15
//!   23 % 5 = 3
//!   23 % 15 = 8
//!   8 % 5 = 3
//! ```
//!
//! [`iter_unsigned`]: ParseOps::iter_unsigned
use crate::util::parse::*;

#[derive(Clone)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    yes: usize,
    no: usize,
}

#[derive(Copy, Clone)]
pub enum Operation {
    Square,
    Multiply(u64),
    Add(u64),
}

/// Extract each Monkey's info from the flavor text. With the exception of the lines starting
/// `Operation` we are only interested in the numbers on each line.
pub fn parse(input: &str) -> Vec<Monkey> {
    /// Inner helper function to keep the parsing logic readable.
    fn helper(chunk: &[&str]) -> Monkey {
        let items = chunk[1].iter_unsigned().collect();
        let tokens: Vec<&str> = chunk[2].split(' ').rev().take(2).collect();
        let operation = match tokens[..] {
            ["old", _] => Operation::Square,
            [y, "*"] => Operation::Multiply(y.unsigned()),
            [y, "+"] => Operation::Add(y.unsigned()),
            _ => unreachable!(),
        };
        let test = chunk[3].unsigned();
        let yes = chunk[4].unsigned();
        let no = chunk[5].unsigned();
        Monkey { items, operation, test, yes, no }
    }
    input.lines().collect::<Vec<&str>>().chunks(7).map(helper).collect()
}

/// Play 20 rounds dividing the worry level by 3 each inspection.
pub fn part1(input: &[Monkey]) -> usize {
    play(input, 20, |x| x / 3)
}

/// Play 10,000 rounds adjusting the worry level modulo the product of all the monkey's test values.
pub fn part2(input: &[Monkey]) -> usize {
    let product: u64 = input.iter().map(|m| m.test).product();
    play(input, 10000, |x| x % product)
}

/// Play an arbitrary number of rounds. The logic to adjust the worry level is passed via a closure
/// so that we can re-use the bulk of the same logic between part 1 and 2.
fn play(input: &[Monkey], rounds: u32, adjust: impl Fn(u64) -> u64) -> usize {
    let mut monkeys = input.to_vec();
    let mut business = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            business[i] += monkeys[i].items.len();

            while let Some(item) = monkeys[i].items.pop() {
                let worry = match monkeys[i].operation {
                    Operation::Square => item * item,
                    Operation::Multiply(y) => item * y,
                    Operation::Add(y) => item + y,
                };
                let next = adjust(worry);
                let to = if next % monkeys[i].test == 0 { monkeys[i].yes } else { monkeys[i].no };
                monkeys[to].items.push(next);
            }
        }
    }

    business.sort_unstable();
    business.iter().rev().take(2).product()
}
