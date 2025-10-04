//! # Handy Haversacks
//!
//! A hashtable would be a natural data structure for this problem but is a little slow.
//! To make things faster we implement the hashtable using an array and a combination of three
//! [perfect hash functions](https://en.wikipedia.org/wiki/Perfect_hash_function) that map from
//! each combination of bag descriptions to a unique index.
//!
//! There are 18 different adjectives, for example shiny, bright, dark. Taking only the first two
//! letters is enough to discriminate. For example the hash value for "shiny" is:
//!
//! ```none
//!     "sh" =>
//!     26 * 's' + 'h' =>
//!     26 * (115 - 97) + (104 - 97) =>
//!     26 * 115 + 104 - 2619 =>
//!     475
//! ```
//!
//! There are 33 different colors, for example white, gold, blue. The first two letters
//! result in some duplicates, however incrementing the second letter if the length is odd
//! is enough to discriminate.
//!
//! These first two hash values are then used to lookup consecutive indices from
//! 0 to 17 and 0 to 32 respectively, which are then combined into a *third* hash value from
//! 0 to 593 to form a perfect minimal hash function.
//!
//! Part one and part two are very similar. A recursive solution with memoization of previously
//! seen values computes the result efficiently.
use crate::util::iter::*;
use crate::util::parse::*;

const FIRST_HASH: [usize; 18] =
    [43, 63, 78, 86, 92, 95, 98, 130, 294, 320, 332, 390, 401, 404, 475, 487, 554, 572];
const SECOND_HASH: [usize; 33] = [
    16, 31, 37, 38, 43, 44, 59, 67, 70, 76, 151, 170, 173, 174, 221, 286, 294, 312, 313, 376, 381,
    401, 410, 447, 468, 476, 495, 498, 508, 515, 554, 580, 628,
];

#[derive(Clone, Copy)]
pub struct Rule {
    amount: u32,
    next: usize,
}

type Bag = [Option<Rule>; 4];

pub struct Haversack {
    shiny_gold: usize,
    bags: [Bag; 594],
}

pub fn parse(input: &str) -> Haversack {
    let mut first_indices = [0; 676];
    let mut second_indices = [0; 676];
    let mut bags = [[None; 4]; 594];

    FIRST_HASH.iter().enumerate().for_each(|(i, &h)| first_indices[h] = i);
    SECOND_HASH.iter().enumerate().for_each(|(i, &h)| second_indices[h] = i);

    let perfect_minimal_hash = |first: &str, second: &str| {
        let first = first.as_bytes();
        let a = first[0] as usize;
        let b = first[1] as usize;

        let second = second.as_bytes();
        let c = second[0] as usize;
        let d = (second[1] as usize) + (second.len() % 2);

        first_indices[26 * a + b - 2619] + 18 * second_indices[26 * c + d - 2619]
    };

    for line in input.lines() {
        let mut tokens = line.split_ascii_whitespace().chunk::<4>();
        let [first, second, _, _] = tokens.next().unwrap();
        let outer = perfect_minimal_hash(first, second);

        for (index, chunk) in tokens.enumerate() {
            let [amount, first, second, _] = chunk;
            let amount = amount.unsigned();
            let next = perfect_minimal_hash(first, second);
            bags[outer][index] = Some(Rule { amount, next });
        }
    }

    let shiny_gold = perfect_minimal_hash("shiny", "gold");
    Haversack { shiny_gold, bags }
}

pub fn part1(input: &Haversack) -> usize {
    fn helper(key: usize, haversack: &Haversack, cache: &mut [Option<bool>]) -> bool {
        if let Some(value) = cache[key] {
            return value;
        }

        let value =
            haversack.bags[key].iter().flatten().any(|rule| helper(rule.next, haversack, cache));

        cache[key] = Some(value);
        value
    }

    let mut cache = vec![None; input.bags.len()];
    cache[input.shiny_gold] = Some(true);
    (0..input.bags.len()).filter(|&key| helper(key, input, &mut cache)).count() - 1
}

pub fn part2(input: &Haversack) -> u32 {
    fn helper(key: usize, haversack: &Haversack, cache: &mut [Option<u32>]) -> u32 {
        if let Some(value) = cache[key] {
            return value;
        }

        let value = 1 + haversack.bags[key]
            .iter()
            .flatten()
            .map(|rule| rule.amount * helper(rule.next, haversack, cache))
            .sum::<u32>();

        cache[key] = Some(value);
        value
    }

    let mut cache = vec![None; input.bags.len()];
    helper(input.shiny_gold, input, &mut cache) - 1
}
