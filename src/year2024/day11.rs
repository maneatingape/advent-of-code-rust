//! # Plutonian Pebbles
//!
//! The transformation rules have an interesting property that the total number of
//! distinct stone numbers is not very large, about 2000 for part one and 4000 for part two.
//!
//! This means that we can store the count of each distinct stone in a small contiguous array
//! that is much faster to process than a recursive memoization approach.
use crate::util::hash::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<u64> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[u64]) -> u64 {
    count(input, 25)
}

pub fn part2(input: &[u64]) -> u64 {
    count(input, 75)
}

fn count(input: &[u64], blinks: usize) -> u64 {
    // Allocate enough room to prevent reallocations.
    let mut stones = Vec::with_capacity(5000);
    // Maps number on stone to a much smaller contiguous range of indices.
    let mut indices = FastMap::with_capacity(5000);
    // Numbers of any new stones generated during the previous blink.
    let mut todo = Vec::new();
    let mut numbers = Vec::new();
    // Amount of each stone of a particular number.
    let mut current = Vec::new();

    // Initialize stones from input.
    for &number in input {
        if let Some(&index) = indices.get(&number) {
            current[index] += 1;
        } else {
            indices.insert(number, indices.len());
            todo.push(number);
            current.push(1);
        }
    }

    for _ in 0..blinks {
        // If a stone number has already been seen then return its index,
        // otherwise queue it for processing during the next blink.
        (numbers, todo) = (todo, numbers);

        let mut index_of = |number| {
            let size = indices.len();
            *indices.entry(number).or_insert_with(|| {
                todo.push(number);
                size
            })
        };

        // Apply the transformation logic to stones added in the previous blink.
        for number in numbers.drain(..) {
            let (first, second) = if number == 0 {
                (index_of(1), usize::MAX)
            } else {
                let digits = number.ilog10() + 1;
                if digits.is_multiple_of(2) {
                    let power = 10_u64.pow(digits / 2);
                    (index_of(number / power), index_of(number % power))
                } else {
                    (index_of(number * 2024), usize::MAX)
                }
            };

            stones.push((first, second));
        }

        // Add amount of each stone to either 1 or 2 stones in the next blink.
        let mut next = vec![0; indices.len()];

        for (&(first, second), amount) in stones.iter().zip(current) {
            next[first] += amount;
            if second != usize::MAX {
                next[second] += amount;
            }
        }

        current = next;
    }

    current.iter().sum()
}
