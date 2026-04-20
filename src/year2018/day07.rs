//! # The Sum of Its Parts
//!
//! Part one is a [topological sort](https://en.wikipedia.org/wiki/Topological_sorting)
//! of the steps based on the dependencies between them. As there are only 26 possible different
//! steps, we can use bitmasks to store the dependency graph, enabling extremely quick lookup.
use crate::util::bitset::*;
use std::cmp::Reverse;

type Input = [Step; 26];

#[derive(Clone, Copy, Default)]
pub struct Step {
    todo: bool,
    from: u32,
    to: u32,
}

pub fn parse(input: &str) -> Input {
    let mut steps = [Step::default(); 26];

    for line in input.as_bytes().chunks(49) {
        // Each step is a single uppercase letter.
        let from = to_index(line[5]);
        let to = to_index(line[36]);

        // Track dependencies as bitmask.
        steps[from].todo = true;
        steps[from].to |= 1 << to;

        steps[to].todo = true;
        steps[to].from |= 1 << from;
    }

    steps
}

pub fn part1(input: &Input) -> String {
    let mut steps = *input;
    let mut done = String::new();

    // Find next available step in alphabetical order.
    while let Some(i) = next_ready(&steps) {
        // Prevent this step being considered again.
        steps[i].todo = false;

        // Keep track of the order of completed tasks.
        done.push(from_index(i));

        // For each dependent step, remove this step from the remaining required steps.
        for j in steps[i].to.biterator() {
            steps[j].from ^= 1 << i;
        }
    }

    done
}

pub fn part2(input: &Input) -> usize {
    part2_testable(input, 5, 60)
}

pub fn part2_testable(input: &Input, max_workers: usize, base_duration: usize) -> usize {
    let mut steps = *input;
    let mut time = 0;
    let mut workers = Vec::new();

    // Loop until there are no more steps available and all workers are idle.
    while next_ready(&steps).is_some() || !workers.is_empty() {
        // Assign any steps to available workers until one or the other runs out first.
        while let Some(i) = next_ready(&steps)
            && workers.len() < max_workers
        {
            // Prevent this step being considered again.
            steps[i].todo = false;

            // Add task duration based on step.
            let finish = time + base_duration + i + 1;

            // Sort workers in reverse order, so that the worker that will finish first is at
            // the end of the vec.
            workers.push((finish, i));
            workers.sort_unstable_by_key(|&(finish, _)| Reverse(finish));
        }

        // Fast forward time until the earliest available worker finishes their step.
        // This may not unblock a dependent step right away in which case the outer loop will
        // bring things back here for another worker to complete.
        let (finish, i) = workers.pop().unwrap();
        time = finish;

        // Update dependent tasks the same as part one.
        for j in steps[i].to.biterator() {
            steps[j].from ^= 1 << i;
        }
    }

    time
}

fn to_index(b: u8) -> usize {
    usize::from(b - b'A')
}

fn from_index(i: usize) -> char {
    char::from(i as u8 + b'A')
}

fn next_ready(steps: &[Step]) -> Option<usize> {
    steps.iter().position(|step| step.todo && step.from == 0)
}
