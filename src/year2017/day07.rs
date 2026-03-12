//! # Recursive Circus
//!
//! Tree structures are tricky to implement in Rust, requiring wrapping the pointer in a [`Rc`].
//! To avoid this we store the tree "upside down" with each node containing a single index to its
//! parent, stored in a flat `vec`.
//!
//! We rely on a special structure of the input that the unbalanced node requiring change will
//! always be the lowest node in the tree and have at least two other balanced siblings
//! so that we can disambiguate.
//!
//! [`Rc`]: std::rc::Rc
use crate::util::hash::*;
use crate::util::parse::*;
use std::collections::VecDeque;

#[derive(Clone, Copy, Default)]
struct Node {
    has_parent: bool,
    parent: usize,
    children: usize,
    processed: usize,
    weight: i32,
    total: i32,
    sub_weights: [i32; 2],
    sub_totals: [i32; 2],
}

type Input<'a> = (&'a str, i32);

pub fn parse(input: &str) -> Input<'_> {
    // Split each line into the program name then the rest of the information.
    let pairs: Vec<_> = input.lines().map(|line| line.split_once(' ').unwrap()).collect();
    // Convert each program name into a fixed index so that we can use faster vec lookups
    // later on when processing the tree.
    let indices: FastMap<_, _> = pairs.iter().enumerate().map(|(i, &(key, _))| (key, i)).collect();
    // Create a vec of the correct size with default values.
    let mut nodes = vec![Node::default(); indices.len()];
    // We'll process nodes from leaf to root.
    let mut todo = VecDeque::new();

    for (i, &(_, suffix)) in pairs.iter().enumerate() {
        // Remove delimiters.
        let mut iter = suffix.split(|c: char| !c.is_ascii_alphanumeric()).filter(|s| !s.is_empty());

        let weight = iter.next().unwrap().signed();
        nodes[i].weight = weight;
        nodes[i].total = weight;

        for edge in iter {
            nodes[i].children += 1;
            let child = indices[edge];
            nodes[child].parent = i;
            nodes[child].has_parent = true;
        }

        // Start with leaf nodes.
        if nodes[i].children == 0 {
            todo.push_back(i);
        }
    }

    // The root is the only node without a parent. Start from any node, and walk up the
    // tree until finding the root.
    let mut candidate = 0;
    while nodes[candidate].has_parent {
        candidate = nodes[candidate].parent;
    }
    let part_one = pairs[candidate].0;
    let mut part_two = 0;

    while let Some(index) = todo.pop_front() {
        let Node { parent, weight, total, .. } = nodes[index];
        let node = &mut nodes[parent];

        if node.processed < 2 {
            // Fill out the first two children in any order.
            node.sub_weights[node.processed] = weight;
            node.sub_totals[node.processed] = total;
        } else {
            // Representing the balanced nodes as `b` and the unbalanced node as `u`,
            // there are 4 possibilities:
            // b3 + [b1 b2] => [b2 b1] Swap, keep accumulating
            // b3 + [b1 u2] => [u2 b1] Swap, unbalanced node identified
            // u3 + [b1 b2] -> [u3 b2] Overwrite, unbalanced node identified
            // b3 + [u1 b2] => [u1 b2] Do nothing, unbalanced node identified
            // The unbalanced node will always be first (if it exists).
            if node.sub_totals[0] == total {
                node.sub_weights.swap(0, 1);
                node.sub_totals.swap(0, 1);
            } else if node.sub_totals[1] != total {
                node.sub_weights[0] = weight;
                node.sub_totals[0] = total;
            }

            // If the unbalanced node was identified, it is now first, and we can short-circuit
            // summing the weights of the rest of the tree.
            let [x, y] = node.sub_totals;

            if x != y {
                part_two = node.sub_weights[0] - x + y;
                break;
            }
        }

        // Total is a node's weight plus the sum of all children recursively.
        node.total += total;
        node.processed += 1;

        // If we've processed all children then add to the queue and check balance.
        if node.processed == node.children {
            todo.push_back(parent);
        }
    }

    (part_one, part_two)
}

pub fn part1<'a>(input: &Input<'a>) -> &'a str {
    input.0
}

pub fn part2(input: &Input<'_>) -> i32 {
    input.1
}
