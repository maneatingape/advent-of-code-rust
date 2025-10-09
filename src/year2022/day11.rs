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
//! A neat trick is that each item can be treated individually. This allows the processing to be
//! parallelized over many threads. To speed things up even more, we notice that items form cycles,
//! repeating the same path through the monkeys. Once we find a cycle for an item, then we short
//! circuit the calculation early without having to calculate the entire 10,000 rounds.
//!
//! [`iter_unsigned`]: ParseOps::iter_unsigned
use crate::util::hash::*;
use crate::util::parse::*;
use crate::util::thread::*;

type Input = (Vec<Monkey>, Vec<Pair>);
type Pair = (usize, usize);

pub struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    yes: usize,
    no: usize,
}

enum Operation {
    Square,
    Multiply(usize),
    Add(usize),
}

#[derive(Clone, Copy)]
struct Business([usize; 8]);

impl Business {
    fn zero() -> Self {
        Business([0; 8])
    }

    fn inc(&mut self, from: usize) {
        self.0[from] += 1;
    }

    fn level(mut self) -> usize {
        self.0.sort_unstable();
        self.0.iter().rev().take(2).product()
    }

    fn add(mut self, rhs: Self) -> Self {
        self.0.iter_mut().zip(rhs.0).for_each(|(a, b)| *a += b);
        self
    }

    fn sub(mut self, rhs: Self) -> Self {
        self.0.iter_mut().zip(rhs.0).for_each(|(a, b)| *a -= b);
        self
    }

    fn mul(mut self, rhs: usize) -> Self {
        self.0.iter_mut().for_each(|a| *a *= rhs);
        self
    }
}

/// Extract each Monkey's info from the flavor text. With the exception of the lines starting
/// `Operation` we are only interested in the numbers on each line.
pub fn parse(input: &str) -> Input {
    let lines: Vec<_> = input.lines().collect();

    let monkeys: Vec<_> = lines
        .chunks(7)
        .map(|chunk: &[&str]| {
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
        })
        .collect();

    let pairs: Vec<_> = monkeys
        .iter()
        .enumerate()
        .flat_map(|(from, monkey)| monkey.items.iter().map(move |&item| (from, item)))
        .collect();

    (monkeys, pairs)
}

pub fn part1(input: &Input) -> usize {
    let (monkeys, pairs) = input;
    let mut business = Business::zero();

    for &(mut from, mut item) in pairs {
        let mut rounds = 0;

        while rounds < 20 {
            let worry = match monkeys[from].operation {
                Operation::Square => item * item,
                Operation::Multiply(y) => item * y,
                Operation::Add(y) => item + y,
            };
            item = worry / 3;

            let to = if item.is_multiple_of(monkeys[from].test) {
                monkeys[from].yes
            } else {
                monkeys[from].no
            };

            business.inc(from);

            // Only increase the round when the item is passes to a previous monkey
            // which will have to be processed in the next turn.
            rounds += usize::from(to < from);
            from = to;
        }
    }

    business.level()
}

pub fn part2(input: &Input) -> usize {
    let (monkeys, pairs) = input;

    // Use as many cores as possible to parallelize the calculation.
    let result = spawn_parallel_iterator(pairs, |iter| {
        iter.map(|&(from, item)| play(monkeys, from, item)).collect::<Vec<_>>()
    });

    // Merge results.
    result.into_iter().flatten().fold(Business::zero(), Business::add).level()
}

/// Play 10,000 rounds adjusting the worry level modulo the product of all the monkey's test values.
/// Look for cycles in each path so that we don't have to process the entire 10,000 rounds.
fn play(monkeys: &[Monkey], mut from: usize, mut item: usize) -> Business {
    let product: usize = monkeys.iter().map(|m| m.test).product();

    let mut round = 0;
    let mut business = Business::zero();

    let mut path = Vec::new();
    let mut seen = FastMap::new();

    path.push(business);
    seen.insert((from, item), path.len() - 1);

    while round < 10_000 {
        let worry = match monkeys[from].operation {
            Operation::Square => item * item,
            Operation::Multiply(y) => item * y,
            Operation::Add(y) => item + y,
        };
        item = worry % product;

        let to = if item.is_multiple_of(monkeys[from].test) {
            monkeys[from].yes
        } else {
            monkeys[from].no
        };

        business.inc(from);

        // Only increase the round when the item is passes to a previous monkey
        // which will have to be processed in the next turn.
        if to < from {
            round += 1;
            path.push(business);

            // If we have found a cycle, then short ciruit and return the final result.
            if let Some(previous) = seen.insert((to, item), path.len() - 1) {
                let cycle_width = round - previous;

                let offset = 10_000 - round;
                let quotient = offset / cycle_width;
                let remainder = offset % cycle_width;

                let full = business.sub(path[previous]).mul(quotient);
                let partial = path[previous + remainder].sub(path[previous]);
                return business.add(full).add(partial);
            }
        }

        from = to;
    }

    business
}
