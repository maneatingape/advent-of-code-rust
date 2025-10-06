//! # Monkey in the Middle
//!
//! This problem is the combination of two Advent of Code classics, extracting numbers from a wall
//! of flavor text and modular arithmetic. For the first problem, our utility [`iter_unsigned`]
//! method comes in handy.
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
//! Each item can be treated individually. This allows the processing to be parallelized over
//! many threads, speeding things up in part two.
//!
//! [`iter_unsigned`]: ParseOps::iter_unsigned
use crate::util::parse::*;
use crate::util::thread::*;

type Pair = (usize, u64);

pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    yes: usize,
    no: usize,
}

pub enum Operation {
    Square,
    Multiply(u64),
    Add(u64),
}

type Business = [u64; 8];

/// Extract each Monkey's info from the flavor text. With the exception of the lines starting
/// `Operation` we are only interested in the numbers on each line.
pub fn parse(input: &str) -> Vec<Monkey> {
    /// Inner helper function to keep the parsing logic readable.
    fn helper(chunk: &[&str]) -> Monkey {
        let items = chunk[1].iter_unsigned().collect();
        let tokens: Vec<_> = chunk[2].split(' ').rev().take(2).collect();
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

pub fn part1(input: &[Monkey]) -> u64 {
    solve(input, sequential)
}

pub fn part2(input: &[Monkey]) -> u64 {
    solve(input, parallel)
}

/// Convenience wrapper to reuse common logic between part one and two.
fn solve(monkeys: &[Monkey], play: impl Fn(&[Monkey], &[Pair]) -> Business) -> u64 {
    let mut pairs = Vec::new();

    for (from, monkey) in monkeys.iter().enumerate() {
        for &item in &monkey.items {
            pairs.push((from, item));
        }
    }

    let mut business = play(monkeys, &pairs);
    business.sort_unstable();
    business.iter().rev().take(2).product()
}

/// Play 20 rounds dividing the worry level by 3 each inspection.
fn sequential(monkeys: &[Monkey], pairs: &[Pair]) -> Business {
    let mut business = [0; 8];

    for &pair in pairs {
        let extra = play(monkeys, 20, |x| x / 3, pair);
        business.iter_mut().enumerate().for_each(|(i, b)| *b += extra[i]);
    }

    business
}

/// Play 10,000 rounds adjusting the worry level modulo the product of all the monkey's test values.
fn parallel(monkeys: &[Monkey], pairs: &[Pair]) -> Business {
    // Use as many cores as possible to parallelize the calculation.
    let result = spawn_parallel_iterator(pairs, |iter| worker(monkeys, iter));

    let mut business = [0; 8];
    for extra in result.into_iter().flatten() {
        business.iter_mut().zip(extra).for_each(|(b, e)| *b += e);
    }
    business
}

/// Multiple worker functions are executed in parallel, one per thread.
fn worker(monkeys: &[Monkey], iter: ParIter<'_, Pair>) -> Vec<Business> {
    let product: u64 = monkeys.iter().map(|m| m.test).product();
    iter.map(|&pair| play(monkeys, 10000, |x| x % product, pair)).collect()
}

/// Play an arbitrary number of rounds for a single item.
///
/// The logic to adjust the worry level is passed via a closure
/// so that we can re-use the bulk of the same logic between part 1 and 2.
fn play(monkeys: &[Monkey], max_rounds: u32, adjust: impl Fn(u64) -> u64, pair: Pair) -> Business {
    let (mut from, mut item) = pair;
    let mut rounds = 0;
    let mut business = [0; 8];

    while rounds < max_rounds {
        let worry = match monkeys[from].operation {
            Operation::Square => item * item,
            Operation::Multiply(y) => item * y,
            Operation::Add(y) => item + y,
        };
        item = adjust(worry);

        let to = if item.is_multiple_of(monkeys[from].test) {
            monkeys[from].yes
        } else {
            monkeys[from].no
        };

        // Only increase the round when the item is passes to a previous monkey
        // which will have to be processed in the next turn.
        rounds += (to < from) as u32;
        business[from] += 1;
        from = to;
    }

    business
}
