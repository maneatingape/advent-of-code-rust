//! # Rambunctious Recitation
//!
//! Hybrid solution that uses both a `vec` and [`FastMap`] to store previously seen values.
//! This approach is faster than using either data structure alone. The threshold is chosen so that
//! about 85% of values are stored in the `vec`.
//!
//! To save space the `vec` is `u32` instead of `usize`. Each difference is at least one so we can
//! use zero as a special value to indicate numbers not seen before.
//!
//! Accessing the map uses the [`Entry`] method as this reduces two key lookups to one.
//!
//! [`FastMap`]: crate::util::hash
//! [`Entry`]: std::collections::hash_map::Entry
use crate::util::hash::*;
use crate::util::parse::*;

const THRESHOLD: usize = 1_000_000;

pub fn parse(input: &str) -> Vec<usize> {
    input.iter_unsigned().collect()
}

pub fn part1(input: &[usize]) -> usize {
    play(input, 2020)
}

pub fn part2(input: &[usize]) -> usize {
    play(input, 30_000_000)
}

fn play(input: &[usize], rounds: usize) -> usize {
    let size = input.len() - 1;
    let mut last = input[size];

    let mut spoken_low = vec![0; rounds.min(THRESHOLD)];
    let mut spoken_high = FastMap::with_capacity(rounds / 5);

    for i in 0..size {
        spoken_low[input[i]] = (i + 1) as u32;
    }

    for i in input.len()..rounds {
        if last < THRESHOLD {
            let previous = spoken_low[last] as usize;
            spoken_low[last] = i as u32;
            last = if previous == 0 { 0 } else { i - previous };
        } else {
            spoken_high
                .entry(last as u32)
                .and_modify(|previous| {
                    last = i - *previous as usize;
                    *previous = i as u32;
                })
                .or_insert_with(|| {
                    last = 0;
                    i as u32
                });
        }
    }

    last
}
