//! # Camel Cards
//!
//! The types of each hand are computed from the frequency of the cards ordered in descending order.
//! For example a full house would have 1 card with a frequency of 3 and a second with a
//! frequency of 2, giving `[3, 2]`. Similarly two pair would be `[2, 2, 1]`.
//! Array comparisons will sort the hand types in order.
//!
//! To make comparisons faster the frequencies and the card ranks are packed into a `usize`:
//!
//! * `55222` => `0x3200055222`
//! * `32T3K` => `0x2111032a3d`
//!
//! For part two we add the numbers of jokers to the highest frequency (which could already be
//! jokers!).
//!
//! * `QQQJA` => `0x41000ccc1a`
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
    sort(input, 11)
}

pub fn part2(input: &[Hand]) -> usize {
    sort(input, 1)
}

fn sort(input: &[Hand], j: usize) -> usize {
    let mut hands: Vec<_> = input
        .iter()
        .map(|&Hand { cards, bid }| {
            let rank = cards.map(|b| match b {
                b'A' => 14,
                b'K' => 13,
                b'Q' => 12,
                b'J' => j,
                b'T' => 10,
                _ => b.to_decimal() as usize,
            });

            let mut freq = [0; 15];
            for r in rank {
                freq[r] += 1;
            }

            let jokers = freq[1];
            freq[1] = 0;
            freq.sort_unstable();
            freq.reverse();
            freq[0] += jokers;

            // To speed up comparisons, pack the frequencies and card ranks
            // into the nibbles of a `usize`.
            let mut key = 0;

            for &f in &freq[..5] {
                key = (key << 4) | f;
            }
            for r in rank {
                key = (key << 4) | r;
            }

            (key, bid)
        })
        .collect();

    hands.sort_unstable();
    hands.iter().enumerate().map(|(i, (_, bid))| (i + 1) * bid).sum()
}
