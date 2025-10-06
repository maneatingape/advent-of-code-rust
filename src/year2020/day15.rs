//! # Rambunctious Recitation
//!
//! For efficiency the `vec` storing the last previously spoken turn of numbers is `u32`.
//! Each difference is at least one so zero is used as a special value to indicate numbers not
//! seen before.
//!
//! To speed things up even more, we notice that most large numbers over a certain threshold are
//! spoken only once. Storing if numbers have been seen before in a compact bitset prevents
//! expensive reads to main memory and halves the time needed for the solution.
//!
//! Zero occurs the most so storing it as a dedicated variable saves another 2% of execution time.
use crate::util::parse::*;

const THRESHOLD: usize = 0x10000;

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
    let mut zeroth = 0;
    let mut spoken = vec![0; rounds];
    let mut seen = vec![0_u64; rounds / 64];

    for i in 0..size {
        if input[i] == 0 {
            zeroth = i + 1;
        } else {
            spoken[input[i]] = (i + 1) as u32;
        }
    }

    for i in input.len()..rounds {
        if last == 0 {
            // Handle zero specially as it occurs the most.
            let previous = zeroth;
            zeroth = i;
            last = if previous == 0 { 0 } else { i - previous };
        } else if last < THRESHOLD {
            // Smaller numbers occur frequently so skip previously seen bitset check.
            let previous = spoken[last] as usize;
            spoken[last] = i as u32;
            last = if previous == 0 { 0 } else { i - previous };
        } else {
            // An array of 30 million `u32`s needs 120 MB of memory which exceeds most caches.
            // Writing and reading to random locations in this large array goes to main memory
            // which is slow. Store if a number has been seen before in a compact bitset,
            // needing only a more cache friendly 4 MB.
            let base = last / 64;
            let mask = 1 << (last % 64);

            if seen[base] & mask == 0 {
                seen[base] |= mask;
                spoken[last] = i as u32;
                last = 0;
            } else {
                let previous = spoken[last] as usize;
                spoken[last] = i as u32;
                last = i - previous;
            }
        }
    }

    last
}
