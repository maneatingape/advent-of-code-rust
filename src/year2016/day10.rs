//! # Balance Bots
//!
//! Performs a [topological sort](https://en.wikipedia.org/wiki/Topological_sorting) of the bots,
//! starting from raw values, passing through some number of bots then ending in an output.
//!
//! We maintain a [`VecDeque`] of chips and destinations starting with raw inputs.
//! Once each robot receives 2 chips then its low and high outputs are added to the queue.
//!
//! As a minor optimization we only need to store the product of outputs 0, 1 and 2.
use crate::util::hash::*;
use crate::util::parse::*;
use std::collections::VecDeque;

type Input = (u32, u32);
type Dest<'a> = (&'a str, &'a str);

struct Bot<'a> {
    low: Dest<'a>,
    high: Dest<'a>,
    chips: [u32; 2],
    amount: usize,
}

pub fn parse(input: &str) -> Input {
    let tokens: Vec<_> = input.split_ascii_whitespace().collect();
    let mut tokens = &tokens[..];

    let mut todo = VecDeque::new();
    let mut bots = FastMap::new();

    let mut part_one = u32::MAX;
    let mut part_two = 1;

    while !tokens.is_empty() {
        if tokens[0] == "value" {
            let value = tokens[1].unsigned();
            let dest = (tokens[4], tokens[5]);

            tokens = &tokens[6..];
            todo.push_back((dest, value));
        } else {
            let key = tokens[1].unsigned();
            let low = (tokens[5], tokens[6]);
            let high = (tokens[10], tokens[11]);

            tokens = &tokens[12..];
            bots.insert(key, Bot { low, high, chips: [0; 2], amount: 0 });
        }
    }

    while let Some(((kind, index), value)) = todo.pop_front() {
        let index = index.unsigned();

        if kind == "bot" {
            bots.entry(index).and_modify(|bot| {
                bot.chips[bot.amount] = value;
                bot.amount += 1;

                if bot.amount == 2 {
                    let min = bot.chips[0].min(bot.chips[1]);
                    let max = bot.chips[0].max(bot.chips[1]);

                    todo.push_back((bot.low, min));
                    todo.push_back((bot.high, max));

                    if min == 17 && max == 61 {
                        part_one = index;
                    }
                }
            });
        } else if index <= 2 {
            part_two *= value;
        }
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
