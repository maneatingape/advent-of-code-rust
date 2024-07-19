//! # Repose Record
use crate::util::hash::*;
use crate::util::parse::*;

type Input = FastMap<usize, [u32; 60]>;

pub fn parse(input: &str) -> Input {
    // Records need to be in chronological order.
    let mut records: Vec<_> = input.lines().map(str::as_bytes).collect();
    records.sort_unstable();

    // Build each sleep schedule
    let mut id = 0;
    let mut start = 0;
    let mut guards = FastMap::new();

    for record in records {
        match record.len() {
            31 => start = to_index(&record[15..17]),
            27 => {
                let end = to_index(&record[15..17]);
                let minutes = guards.entry(id).or_insert_with(|| [0; 60]);
                (start..end).for_each(|i| minutes[i] += 1);
            }
            _ => id = to_index(&record[26..record.len() - 13]),
        }
    }

    guards
}

/// Find the guard with the greatest total minutes asleep.
pub fn part1(input: &Input) -> usize {
    choose(input, |(_, m)| m.iter().sum())
}

/// Find the guard with the highest single minute asleep.
pub fn part2(input: &Input) -> usize {
    choose(input, |(_, m)| *m.iter().max().unwrap())
}

fn choose(input: &Input, strategy: impl Fn(&(&usize, &[u32; 60])) -> u32) -> usize {
    // Find the guard using a specific strategy.
    let (id, minutes) = input.iter().max_by_key(strategy).unwrap();
    // Find the minute spent asleep the most
    let (minute, _) = minutes.iter().enumerate().max_by_key(|(_, m)| **m).unwrap();
    // Return result
    id * minute
}

fn to_index(slice: &[u8]) -> usize {
    slice.iter().fold(0, |acc, n| 10 * acc + n.to_decimal() as usize)
}
