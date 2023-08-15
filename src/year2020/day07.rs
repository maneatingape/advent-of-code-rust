//! # Handy Haversacks
//!
//! A hashtable would be a natural data structure for this problem but is a little slow.
//! To make things faster we implement the hashtable using an array and a
//! [perfect hash function](https://en.wikipedia.org/wiki/Perfect_hash_function) that maps from
//! each combination of bag descriptions to a unique index.
//!
//! There are 18 different adjectives, for example shiny, bright, dark. Taking only the first two
//! letters in enough to discriminate.
//!
//! There are 33 different colors, for example white, gold, blue. The first two letters
//! result in some duplicates, however incrementing the second letter if the length is odd
//! is enough to discriminate.
//!
//! These 4 letter values are then combined with some constants determined using a brute force
//! search to give a unique index.
//!
//! Part one and part two are very similar. A recursive solution with memoization of previously
//! seen values computes the result efficiently.
use crate::util::iter::*;
use crate::util::parse::*;

#[derive(Clone, Copy)]
pub struct Rule {
    amount: u32,
    next: usize,
}

type Bag = [Option<Rule>; 4];

pub struct Haversack {
    shiny_gold: usize,
    bags: [Bag; 2522],
}

pub fn parse(input: &str) -> Haversack {
    let mut bags = [[None; 4]; 2522];

    for line in input.lines() {
        let mut tokens = line.split_ascii_whitespace().chunk::<4>();
        let [first, second, _, _] = tokens.next().unwrap();
        let outer = perfect_hash(first, second);

        for (index, chunk) in tokens.enumerate() {
            let [amount, first, second, _] = chunk;
            let amount = amount.unsigned();
            let next = perfect_hash(first, second);
            bags[outer][index] = Some(Rule { amount, next });
        }
    }

    let shiny_gold = perfect_hash("shiny", "gold");
    Haversack { shiny_gold, bags }
}

pub fn part1(input: &Haversack) -> usize {
    fn helper(key: usize, haversack: &Haversack, cache: &mut [Option<bool>]) -> bool {
        if let Some(value) = cache[key] {
            value
        } else {
            let mut value = false;
            let mut iter = haversack.bags[key].iter();

            while let Some(Some(Rule { next, .. })) = iter.next() {
                if helper(*next, haversack, cache) {
                    value = true;
                    break;
                }
            }

            cache[key] = Some(value);
            value
        }
    }

    let mut cache = vec![None; input.bags.len()];
    cache[input.shiny_gold] = Some(true);
    (0..input.bags.len()).filter(|&key| helper(key, input, &mut cache)).count() - 1
}

pub fn part2(input: &Haversack) -> u32 {
    fn helper(key: usize, haversack: &Haversack, cache: &mut [Option<u32>]) -> u32 {
        if let Some(value) = cache[key] {
            value
        } else {
            let mut value = 1;
            let mut iter = haversack.bags[key].iter();

            while let Some(Some(Rule { amount, next })) = iter.next() {
                value += amount * helper(*next, haversack, cache);
            }

            cache[key] = Some(value);
            value
        }
    }

    let mut cache = vec![None; input.bags.len()];
    helper(input.shiny_gold, input, &mut cache) - 1
}

fn perfect_hash(first_name: &str, second_name: &str) -> usize {
    let first_name = first_name.as_bytes();
    let a = first_name[0] as usize;
    let b = first_name[1] as usize;

    let second_name = second_name.as_bytes();
    let c = second_name[0] as usize;
    let d = (second_name[1] as usize) + (second_name.len() % 2);

    (a * 75 + b * 614 + c * 2137 + d * 1938) % 2522
}
