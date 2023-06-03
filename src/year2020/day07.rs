//! # Handy Haversacks
//!
//! A hashtable of hashtables would be a natural data structure for this problem but is a little
//! slow. To make things faster we parse the input in two passes. During the first pass we assign
//! each bag an index. Then during the second pass each bag's children are converted to this index and
//! stored in a fixed size array large enough to represent the bag containing the highest amount of
//! different other bags. Each bag is then stored in a `vec` using the indices computed in the first
//! pass.
//!
//! Part one and part two are very similar. A recursive solution with memoization of previously
//! seen values computes the result efficiently.
use crate::util::hash::*;
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
    bags: Vec<Bag>,
}

pub fn parse(input: &str) -> Haversack {
    let lines: Vec<_> = input.lines().collect();
    let mut indices = FastMapBuilder::with_capacity(1_000);
    let mut bags = Vec::with_capacity(1_000);

    for line in lines.iter() {
        let mut tokens = line.split_ascii_whitespace().chunk::<2>();
        let [first_name, second_name] = tokens.next().unwrap();
        indices.insert((first_name, second_name), indices.len());
    }

    for line in lines.iter() {
        let tokens = line.split_ascii_whitespace().chunk::<4>().skip(1).enumerate();
        let mut bag = [None; 4];

        for (index, [amount, first_name, second_name, _]) in tokens {
            let amount = from(amount);
            let next = indices[&(first_name, second_name)];
            bag[index] = Some(Rule { amount, next });
        }

        bags.push(bag);
    }

    let shiny_gold = indices[&("shiny", "gold")];
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
