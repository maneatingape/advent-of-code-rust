//! # Science for Hungry People
//!
//! Brute force solution trying every possible combination of ingredients. The loop conditions are
//! calculated so that the ingredients always sum to 100. Solves part one and two simultaneously.
//!
//! As an optimization we check halfway through the loops to see if any ingredient will never be
//! be greater than zero to skip large numbers of combinations.
use crate::util::iter::*;
use crate::util::parse::*;
use std::array::from_fn;

type Ingredient = [i32; 5];
type Input = (i32, i32);

pub fn parse(input: &str) -> Input {
    let recipe: Vec<Ingredient> = input.iter_signed().chunk::<5>().collect();
    let mut part_one = 0;
    let mut part_two = 0;

    for a in 0..101 {
        let first: Ingredient = from_fn(|i| a * recipe[0][i]);

        'outer: for b in 0..(101 - a) {
            let second: Ingredient = from_fn(|i| first[i] + b * recipe[1][i]);

            // Check if any ingredient can never be greater than zero.
            // This makes the entire score zero, so we can skip.
            for ((x, y), z) in second.iter().zip(recipe[2]).zip(recipe[3]).take(4) {
                if x + y.max(z) * (100 - a - b) <= 0 {
                    continue 'outer;
                }
            }

            for c in 0..(101 - a - b) {
                let d = 100 - a - b - c;
                let third: Ingredient = from_fn(|i| second[i] + c * recipe[2][i]);
                let fourth: Ingredient = from_fn(|i| third[i] + d * recipe[3][i]);

                let score =
                    fourth[0].max(0) * fourth[1].max(0) * fourth[2].max(0) * fourth[3].max(0);
                let calories = fourth[4];

                part_one = part_one.max(score);
                if calories == 500 {
                    part_two = part_two.max(score);
                }
            }
        }
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}
