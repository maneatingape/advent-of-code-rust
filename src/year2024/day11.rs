//! # Plutonian Pebbles
//!
//! Each stone is independent and does not affect its neighbours. This means that we can
//! recursively split each stone, skipping trillions of calculations by memoizing the count for
//! each `(stone, blinks)` tuple.
//!
//! Interestingly the number of distinct stones is not too large, about 5000 for part two.
use crate::util::hash::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<u64> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[u64]) -> u64 {
    let cache = &mut FastMap::with_capacity(5_000);
    input.iter().map(|&stone| count(cache, stone, 25)).sum()
}

pub fn part2(input: &[u64]) -> u64 {
    let cache = &mut FastMap::with_capacity(150_000);
    input.iter().map(|&stone| count(cache, stone, 75)).sum()
}

fn count(cache: &mut FastMap<(u64, u64), u64>, stone: u64, blinks: u64) -> u64 {
    if blinks == 1 {
        if stone == 0 {
            return 1;
        }
        let digits = stone.ilog10() + 1;
        return if digits % 2 == 0 { 2 } else { 1 };
    }

    let key = (stone, blinks);
    if let Some(&value) = cache.get(&key) {
        return value;
    }

    let next = if stone == 0 {
        count(cache, 1, blinks - 1)
    } else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let power = 10_u64.pow(digits / 2);
            count(cache, stone / power, blinks - 1) + count(cache, stone % power, blinks - 1)
        } else {
            count(cache, stone * 2024, blinks - 1)
        }
    };

    cache.insert(key, next);
    next
}
