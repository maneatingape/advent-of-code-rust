//! # If You Give A Seed A Fertilizer
use crate::util::iter::*;
use crate::util::parse::*;

pub struct Input {
    seeds: Vec<u64>,
    stages: Vec<Vec<[u64; 3]>>,
}

pub fn parse(input: &str) -> Input {
    let chunks: Vec<_> = input.split("\n\n").collect();
    let seeds = chunks[0].iter_unsigned().collect();
    let stages = chunks[1..]
        .iter()
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
            for &[dest, start, end] in stage {
                if start <= *seed && *seed < end {
                    *seed = *seed - start + dest;
                    break;
                }
            }
        }
    }

    *seeds.iter().min().unwrap()
}

/// Process ranges.
pub fn part2(input: &Input) -> u64 {
    let mut current = &mut Vec::new();
    let mut next = &mut Vec::new();

    // Convert input pairs to ranges.
    for [start, length] in input.seeds.iter().copied().chunk::<2>() {
        current.push([start, start + length]);
    }

    for stage in &input.stages {
        'outer: for &[s1, e1] in current.iter() {
            // Split ranges that overlap into 1, 2 or 3 new ranges.
            // Assumes that seed ranges will only overlap with a single range in each stage.
            for &[dest, s2, e2] in stage {
                if s2 <= s1 && e1 <= e2 {
                    // Seed range is completely contained.
                    //      ssss
                    //    mmmmmmmm
                    next.push([s1 - s2 + dest, e1 - s2 + dest]);
                    continue 'outer;
                } else if s1 < s2 && s2 < e1 && e1 <= e2 {
                    // Upper half of seed range overlaps.
                    //      ssss
                    //        mmmm
                    next.push([s1, s2]);
                    next.push([dest, e1 - s2 + dest]);
                    continue 'outer;
                } else if s2 <= s1 && s1 < e2 && e2 < e1 {
                    // Lower half of seed range overlaps.
                    //      ssss
                    //    mmmm
                    next.push([s1 - s2 + dest, e2 - s2 + dest]);
                    next.push([e2, e1]);
                    continue 'outer;
                } else if s1 < s2 && e2 < e1 {
                    // Seed range contains other range.
                    //   sssssssss
                    //      mm
                    next.push([s1, s2]);
                    next.push([dest, e2 - s2 + dest]);
                    next.push([e2, e1]);
                    continue 'outer;
                }
            }
            // No intersection with any range so pass to next stage unchanged.
            next.push([s1, e1]);
        }

        (current, next) = (next, current);
        next.clear();
    }

    current.iter().map(|r| r[0]).min().unwrap()
}
