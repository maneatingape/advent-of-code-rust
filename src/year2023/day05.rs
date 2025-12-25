//! # If You Give A Seed A Fertilizer
use crate::util::iter::*;
use crate::util::parse::*;

pub struct Input {
    seeds: Vec<u64>,
    stages: Vec<Vec<[u64; 3]>>,
}

pub fn parse(input: &str) -> Input {
    let mut chunks = input.split("\n\n");
    let seeds = chunks.next().unwrap().iter_unsigned().collect();
    let stages = chunks
        .map(|chunk| {
            // Convert from start and length to start and end.
            chunk
                .iter_unsigned()
                .chunk::<3>()
                .map(|[dest, start, length]| [dest, start, start + length])
                .collect()
        })
        .collect();

    Input { seeds, stages }
}

/// Process each seed individually.
pub fn part1(input: &Input) -> u64 {
    let mut seeds = input.seeds.clone();

    for stage in &input.stages {
        for seed in &mut seeds {
            if let Some(&[dest, start, _]) =
                stage.iter().find(|&&[_, start, end]| start <= *seed && *seed < end)
            {
                *seed = *seed - start + dest;
            }
        }
    }

    seeds.into_iter().min().unwrap()
}

/// Process ranges.
pub fn part2(input: &Input) -> u64 {
    let mut current = Vec::new();
    let mut next = Vec::new();
    let mut next_stage = Vec::new();

    // Convert input pairs to ranges.
    for [start, length] in input.seeds.iter().copied().chunk::<2>() {
        current.push([start, start + length]);
    }

    for stage in &input.stages {
        for &[dest, s2, e2] in stage {
            for [s1, e1] in current.drain(..) {
                // Split ranges that overlap into 1, 2 or 3 new ranges.
                // x1 and x2 are the possible overlap.
                let x1 = s1.max(s2);
                let x2 = e1.min(e2);

                if x1 >= x2 {
                    // No overlap.
                    next.push([s1, e1]);
                } else {
                    // Move overlap to new destination. Only compare with next range.
                    next_stage.push([x1 - s2 + dest, x2 - s2 + dest]);

                    // Check remnants with remaining ranges.
                    if s1 < x1 {
                        next.push([s1, x1]);
                    }
                    if x2 < e1 {
                        next.push([x2, e1]);
                    }
                }
            }

            (current, next) = (next, current);
        }

        // Combine elements for the next stage.
        current.append(&mut next_stage);
    }

    current.iter().map(|r| r[0]).min().unwrap()
}
