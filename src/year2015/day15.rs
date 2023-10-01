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

        for b in 0..(101 - a) {
            let second: Ingredient = from_fn(|i| first[i] + b * recipe[1][i]);

            // Check if any ingredient can never be greater than zero.
            let check: Ingredient =
                from_fn(|i| second[i] + recipe[2][i].max(recipe[3][i]) * (100 - a - b));
            if check.iter().any(|&n| n <= 0) {
                continue;
            }

            for c in 0..(101 - a - b) {
                let d = 100 - a - b - c;
                let third: Ingredient = from_fn(|i| second[i] + c * recipe[2][i]);
                let fourth: Ingredient = from_fn(|i| third[i] + d * recipe[3][i]);

                let score = fourth.iter().take(4).map(|&n| n.max(0)).product();
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
