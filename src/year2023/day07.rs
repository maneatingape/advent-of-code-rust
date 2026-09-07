//! # Camel Cards
//!
//! The type of each hand is computed from the frequency of its cards in descending order.
//! For example, a full house has 1 card with a frequency of 3 and a second with a frequency of 2,
//! giving `[3, 2]`. Similarly, two pair is `[2, 2, 1]`. To make comparisons faster the frequencies
//! and the card ranks are packed into a `usize`, for example:
//!
//! * `55222` => `0x3200055222`
//! * `32T3K` => `0x2111032a3d`
//!
//! For part two, the strongest hand type is always made by adding the number of jokers to the
//! highest frequency card (which could also be jokers in the case of `JJJJJ`).
//!
//! * `QQQJA` => `0x41000ccc1a`
use std::cmp::Reverse;
use std::mem::replace;

use crate::util::parse::*;

pub struct Hand {
    cards: [u8; 5],
    bid: usize,
}

pub fn parse(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (prefix, suffix) = line.split_at(5);
            let cards = prefix.as_bytes().try_into().unwrap();
            let bid = suffix.unsigned();
            Hand { cards, bid }
        })
        .collect()
}

pub fn part1(input: &[Hand]) -> usize {
    winnings(input, 11)
}

pub fn part2(input: &[Hand]) -> usize {
    winnings(input, 1)
}

fn winnings(input: &[Hand], jack: usize) -> usize {
    let mut hands: Vec<_> = input
        .iter()
        .map(|&Hand { cards, bid }| {
            let ranks = cards.map(|b| match b {
                b'A' => 14,
                b'K' => 13,
                b'Q' => 12,
                b'J' => jack,
                b'T' => 10,
                _ => b.to_decimal(),
            });

            let mut frequency = [0; 15];
            for rank in ranks {
                frequency[rank] += 1;
            }

            // Set jokers aside so that they increase the biggest group.
            let jokers = replace(&mut frequency[1], 0);

            // Each card contributes its frequency once, then zero for any duplicates.
            let mut groups = ranks.map(|rank| replace(&mut frequency[rank], 0));
            groups.sort_unstable_by_key(|&count| Reverse(count));
            groups[0] += jokers;

            // To speed up comparisons, pack the groups and card ranks into hex nibbles.
            let key = groups.iter().chain(&ranks).fold(0, |key, &value| (key << 4) | value);
            (key, bid)
        })
        .collect();

    hands.sort_unstable();
    hands.into_iter().zip(1..).map(|((_, bid), rank)| bid * rank).sum()
}
