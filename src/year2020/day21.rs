//! # Allergen Assessment
//!
//! The rules can be expressed as:
//!
//! * If an ingredient is on a line, then it *may* contain the listed allergens.
//! * If an ingredient is *not* on a line, then it definitely *does not* contain the listed
//!   allergens, as some other food on the line must instead contain the allergen.
//!
//! ## Part One
//! To find the safe foods we build two sets, then subtract them to find out the remaining possible
//! allergens. It's important to only subtract the sets at the very end in order to prevent
//! re-adding a previously excluded allergen. Using `kfcds` from the example:
//!
//! | Line | Possible    | Impossible       |
//! | ---  | ----------- | ---------------- |
//! | 1    | Dairy, Fish | Ø                |
//! | 2    | Dairy, Fish | Dairy            |
//! | 3    | Dairy, Fish | Dairy, Soy       |
//! | 4    | Dairy, Fish | Dairy, Soy, Fish |
//!
//! Final result: Ø (the empty set)
//!
//! # Part Two
//! This is a [constraint satisfaction problem](https://en.wikipedia.org/wiki/Constraint_satisfaction_problem),
//! similar to [`day 16`]. Using `fvjkl` from the example:
//!
//! | Line | Possible   | Impossible  |
//! | ---  | ---------- | ----------- |
//! | 1    | Ø          | Dairy, Fish |
//! | 2    | Dairy      | Dairy, Fish |
//! | 3    | Dairy, Soy | Dairy, Fish |
//! | 4    | Dairy, Soy | Dairy, Fish |
//!
//! Final result: Soy
//!
//! To solve there must be at least one ingredient with only one allergen remaining.
//! As this allergen can only belong to this ingredient, we eliminate it from other ingredients.
//! This causes a chain reaction where a second ingredient will reduce to only one allergen,
//! continuing until all allergens have been resolved.
//!
//! As there are less than 64 lines and allergens we can speed things up by using bitwise logic
//! on a `usize` to compute set addition and subtraction. To add to a set use OR `|`,
//! to remove use AND `&` and to calculate the size use [`count_ones`].
//!
//! [`Day 16`]: crate::year2020::day16
//! [`count_ones`]: u32::count_ones
use crate::util::hash::*;

pub struct Input<'a> {
    ingredients: FastMap<&'a str, Ingredient>,
    allergens: FastMap<&'a str, usize>,
}

#[derive(Clone, Copy, Default)]
pub struct Ingredient {
    food: usize,
    candidates: usize,
}

pub fn parse(input: &str) -> Input {
    let mut ingredients: FastMap<&str, Ingredient> = FastMapBuilder::empty();
    let mut allergens = FastMapBuilder::empty();
    let mut allergens_per_food = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let (prefix, suffix) = line.rsplit_once(" (contains ").unwrap();

        for ingredient in prefix.split_ascii_whitespace() {
            let entry = ingredients.entry(ingredient).or_default();
            entry.food |= 1 << i;
        }

        let mut mask = 0;
        for allergen in suffix.split([' ', ',', ')']).filter(|a| !a.is_empty()) {
            let size = allergens.len();
            let entry = allergens.entry(allergen).or_insert(size);
            mask |= 1 << *entry;
        }
        allergens_per_food.push(mask);
    }

    for ingredient in ingredients.values_mut() {
        let mut possible = 0;
        let mut impossible = 0;

        for (i, allergens) in allergens_per_food.iter().enumerate() {
            if ingredient.food & (1 << i) == 0 {
                impossible |= allergens;
            } else {
                possible |= allergens;
            }
        }

        ingredient.candidates = possible & !impossible;
    }

    Input { ingredients, allergens }
}

pub fn part1(input: &Input) -> u32 {
    input.ingredients.values().filter(|i| i.candidates == 0).map(|i| i.food.count_ones()).sum()
}

pub fn part2(input: &Input) -> String {
    let mut ingredients = input.ingredients.clone();
    ingredients.retain(|_, v| v.candidates != 0);

    let inverse_allergens: FastMap<_, _> =
        input.allergens.iter().map(|(k, v)| (1 << v, k)).collect();

    // There must be at least one ingredient with only one allergen.
    let mut todo: Vec<_> = ingredients
        .iter()
        .filter(|(_, v)| v.candidates.count_ones() == 1)
        .map(|(k, v)| (*k, v.candidates))
        .collect();
    let mut done: Vec<_> = vec![];

    // Eliminate known allergens from other ingredients.
    while let Some(pair @ (next, allergen)) = todo.pop() {
        ingredients.remove(next);
        done.push(pair);

        for (name, ingredient) in &mut ingredients {
            ingredient.candidates &= !allergen;
            if ingredient.candidates.count_ones() == 1 {
                todo.push((name, ingredient.candidates));
            }
        }
    }

    // Sort by alphabetical order of the allergens.
    done.sort_by_cached_key(|(_, v)| inverse_allergens[v]);
    done.iter().map(|(k, _)| *k).collect::<Vec<_>>().join(",")
}
