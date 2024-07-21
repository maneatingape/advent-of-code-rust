//! # Memory Maneuver
//!
//! Recursive solution computing both parts at the same time, sharing a single mutable iterator.
//! A shared stack is used to store the scores for child nodes temporarily.
use crate::util::parse::*;

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    parse_node(&mut input.iter_unsigned(), &mut Vec::new())
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

fn parse_node(iter: &mut impl Iterator<Item = usize>, stack: &mut Vec<usize>) -> (usize, usize) {
    // Parse header
    let child_count = iter.next().unwrap();
    let metadata_count = iter.next().unwrap();

    let mut metadata = 0;
    let mut score = 0;

    // Parse child nodes, adding their metadata to current node and saving their score for
    // when metadata is processed.
    for _ in 0..child_count {
        let (first, second) = parse_node(iter, stack);
        metadata += first;
        stack.push(second);
    }

    // Process metadata.
    for _ in 0..metadata_count {
        let n = iter.next().unwrap();
        metadata += n;

        if child_count == 0 {
            score += n;
        } else if n > 0 && n <= child_count {
            score += stack[stack.len() - child_count + (n - 1)];
        }
    }

    // Pop child nodes from the stack.
    stack.truncate(stack.len() - child_count);

    (metadata, score)
}
