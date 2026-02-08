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
use crate::util::integer::*;
use crate::util::parse::*;
use std::collections::VecDeque;

type Input = (u32, u32);
type Dest = (bool, u32);

struct Bot {
    low: Dest,
    high: Dest,
    chip: Option<u32>,
}

pub fn parse(input: &str) -> Input {
    let tokens: Vec<_> = input.split_ascii_whitespace().collect();
    let mut tokens = &tokens[..];

    let mut todo = VecDeque::with_capacity(500);
    let mut bots = FastMap::with_capacity(500);

    let mut part_one = u32::MAX;
    let mut part_two = 1;

    while !tokens.is_empty() {
        if tokens[0] == "value" {
            let value = tokens[1].unsigned();
            let dest = to_dest(tokens[4], tokens[5]);

            todo.push_back((dest, value));
            tokens = &tokens[6..];
        } else {
            let key: u32 = tokens[1].unsigned();
            let low = to_dest(tokens[5], tokens[6]);
            let high = to_dest(tokens[10], tokens[11]);

            bots.insert(key, Bot { low, high, chip: None });
            tokens = &tokens[12..];
        }
    }

    while let Some(((is_bot, index), value)) = todo.pop_front() {
        if is_bot {
            let bot = bots.get_mut(&index).unwrap();

            if let Some(previous) = bot.chip {
                let (min, max) = previous.minmax(value);
                if min == 17 && max == 61 {
                    part_one = index;
                }

                todo.push_back((bot.low, min));
                todo.push_back((bot.high, max));
            } else {
                bot.chip = Some(value);
            }
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

fn to_dest(first: &str, second: &str) -> Dest {
    (first == "bot", second.unsigned())
}
