//! # Space Stoichiometry
//!
//! Sorting the reactions in [topological order](https://en.wikipedia.org/wiki/Topological_sorting)
//! from `FUEL` at the start to `ORE` at the end, allows us to process each reaction only once.
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;
use std::cmp::Ordering;

struct Ingredient {
    amount: u64,
    chemical: usize,
}

pub struct Reaction {
    amount: u64,
    chemical: usize,
    ingredients: Vec<Ingredient>,
}

/// To speed things up when processing, we use a temporary map to convert chemical names into
/// contiguous indices.
pub fn parse(input: &str) -> Vec<Reaction> {
    let lines: Vec<_> = input.lines().collect();

    let mut reactions: Vec<_> = (0..lines.len() + 1)
        .map(|_| {
            Reaction {
                amount: 0,
                chemical: 1, // Default to ORE, other chemicals will overwrite.
                ingredients: Vec::new(),
            }
        })
        .collect();

    // Assign FUEL and ORE known indices as we'll need to look them up later.
    let mut indices = FastMap::new();
    indices.insert("FUEL", 0);
    indices.insert("ORE", 1);

    for line in lines {
        let mut tokens = line
            .split(|c: char| !c.is_ascii_alphanumeric())
            .filter(|s| !s.is_empty())
            .rev()
            .chunk::<2>();

        // Assigns other indices in the arbitrary order that chemicals are encountered.
        let [kind, amount] = tokens.next().unwrap();
        let size = indices.len();
        let chemical = *indices.entry(kind).or_insert(size);

        let reaction = &mut reactions[chemical];
        reaction.amount = amount.unsigned();
        reaction.chemical = chemical;

        for [kind, amount] in tokens {
            let amount = amount.unsigned();
            let size = indices.len();
            let chemical = *indices.entry(kind).or_insert(size);
            reaction.ingredients.push(Ingredient { amount, chemical });
        }
    }

    // Sort reactions in topological order
    let mut order = vec![0; reactions.len()];
    topological(&reactions, &mut order, 0, 0);
    reactions.sort_unstable_by_key(|r| order[r.chemical]);
    reactions
}

/// Calculate the amount of ore needed for 1 fuel. This will be the most ore needed per unit of
/// fuel. Larger amounts of fuel can use some of the leftover chemicals from intermediate reactions.
pub fn part1(input: &[Reaction]) -> u64 {
    ore(input, 1)
}

/// Find the maximum amount of fuel possible from 1 trillion ore with an efficient binary search.
pub fn part2(input: &[Reaction]) -> u64 {
    let threshold = 1_000_000_000_000;
    let mut start = 1;
    let mut end = threshold;

    while start < end {
        let middle = (start + end) / 2;

        match ore(input, middle).cmp(&threshold) {
            Ordering::Less => start = middle + 1,
            Ordering::Equal => return middle,
            Ordering::Greater => end = middle - 1,
        }
    }

    start
}

/// Sort reactions in topological order from FUEL at the root to ORE at the leaves. Reactions may
/// occur more than once at different depths in the graph, so we take the maximum depth.
fn topological(reactions: &[Reaction], order: &mut [usize], chemical: usize, depth: usize) {
    order[chemical] = order[chemical].max(depth);

    for ingredient in &reactions[chemical].ingredients {
        topological(reactions, order, ingredient.chemical, depth + 1);
    }
}

/// Run the reactions to find ore needed. Each chemical is processed only once, so we don't need
/// to track excess values of intermediate chemicals.
fn ore(reactions: &[Reaction], amount: u64) -> u64 {
    let mut total = vec![0; reactions.len()];
    total[0] = amount;

    for reaction in &reactions[..reactions.len() - 1] {
        let multiplier = total[reaction.chemical].div_ceil(reaction.amount);

        for ingredient in &reaction.ingredients {
            total[ingredient.chemical] += multiplier * ingredient.amount;
        }
    }

    total[1]
}
