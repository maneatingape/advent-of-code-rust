//! # The Sum of Its Parts
//!
//! Part one is a [topological sort](https://en.wikipedia.org/wiki/Topological_sorting)
//! of the steps based on the dependencies between them.
use crate::util::hash::*;
use std::cmp::Reverse;
use std::collections::BTreeMap;

type Input = FastMap<u8, Step>;

#[derive(Clone, Default)]
pub struct Step {
    remaining: u32,
    children: Vec<u8>,
}

pub fn parse(input: &str) -> Input {
    let mut steps: Input = FastMap::new();

    for line in input.lines().map(str::as_bytes) {
        // Each step is a single uppercase letter.
        let from = line[5];
        let to = line[36];

        // Add all steps that depend on this one to children vec.
        steps.entry(from).or_default().children.push(to);

        // Count how many steps must finish before this step is ready.
        // We only need the total count, the exact steps are not necessary.
        steps.entry(to).or_default().remaining += 1;
    }

    steps
}

pub fn part1(input: &Input) -> String {
    // Move all steps with no dependencies to the `ready` map. A `BTreeMap` is sorted by key
    // so will retrieve steps in alphabetical order.
    let (mut ready, mut blocked) = split_by_readiness(input);
    let mut done = String::new();

    while let Some((key, step)) = ready.pop_first() {
        // Keep track of the order of completed tasks.
        done.push(key as char);

        // For each dependent step, decrease the remaining count by one. Once a step reaches zero
        // then all its dependencies have been completed and we can move it to the `ready` map.
        for key in step.children {
            let mut step = blocked.remove(&key).unwrap();
            step.remaining -= 1;

            if step.remaining == 0 {
                ready.insert(key, step);
            } else {
                blocked.insert(key, step);
            }
        }
    }

    done
}

pub fn part2(input: &Input) -> u32 {
    part2_testable(input, 5, 60)
}

pub fn part2_testable(input: &Input, max_workers: usize, base_duration: u32) -> u32 {
    // Same as part one, move all tasks that are root nodes to the `ready` map.
    let (mut ready, mut blocked) = split_by_readiness(input);

    // Loop until there are no more steps available and all workers are idle.
    let mut time = 0;
    let mut workers = Vec::new();

    while !ready.is_empty() || !workers.is_empty() {
        // Assign any steps to available workers until one or the other runs out first.
        while !ready.is_empty() && workers.len() < max_workers {
            let (key, step) = ready.pop_first().unwrap();
            let finish = time + base_duration + (key - 64) as u32;

            // Sort workers in reverse order, so that the worker that will finish first is at
            // the end of the vec.
            workers.push((finish, step));
            workers.sort_unstable_by_key(|&(time, _)| Reverse(time));
        }

        // Fast forward time until the earliest available worker finishes their step.
        // This may not unblock a dependent step right away in which case the outer loop will
        // bring things back here for another worker to complete.
        let (finish, step) = workers.pop().unwrap();
        time = finish;

        // Update dependent tasks the same as part one.
        for key in step.children {
            let mut step = blocked.remove(&key).unwrap();
            step.remaining -= 1;

            if step.remaining == 0 {
                ready.insert(key, step);
            } else {
                blocked.insert(key, step);
            }
        }
    }

    time
}

fn split_by_readiness(input: &Input) -> (BTreeMap<u8, Step>, FastMap<u8, Step>) {
    let mut ready = BTreeMap::new();
    let mut blocked = FastMap::new();

    for (key, step) in input.clone() {
        if step.remaining == 0 {
            ready.insert(key, step);
        } else {
            blocked.insert(key, step);
        }
    }

    (ready, blocked)
}
