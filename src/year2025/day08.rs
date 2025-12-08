//! # Playground
use crate::util::iter::*;
use crate::util::parse::*;

type Box = [u64; 3];
type Pair = (usize, usize, u64);
type Input = (Vec<Box>, Vec<Pair>);

struct Node {
    parent: usize,
    size: usize,
}

pub fn parse(input: &str) -> Input {
    let boxes: Vec<_> = input.iter_unsigned::<u64>().chunk::<3>().collect();
    let mut pairs = Vec::with_capacity(boxes.len() * (boxes.len() - 1));

    for (i, &v1) in boxes.iter().enumerate() {
        for (j, &v2) in boxes.iter().enumerate().skip(i + 1) {
            let dx = v1[0].abs_diff(v2[0]);
            let dy = v1[1].abs_diff(v2[1]);
            let dz = v1[2].abs_diff(v2[2]);
            let distance = dx * dx + dy * dy + dz * dz;
            pairs.push((i, j, distance));
        }
    }

    pairs.sort_unstable_by_key(|&(.., distance)| distance);
    (boxes, pairs)
}

pub fn part1(input: &Input) -> usize {
    part1_testable(input, 1000)
}

pub fn part1_testable(input: &Input, limit: usize) -> usize {
    let (boxes, pairs) = input;
    let mut nodes: Vec<_> = (0..boxes.len()).map(|i| Node { parent: i, size: 1 }).collect();

    for &(i, j, ..) in pairs.iter().take(limit) {
        union(&mut nodes, i, j);
    }

    nodes.sort_unstable_by_key(|node| node.size);
    nodes.iter().rev().take(3).map(|node| node.size).product()
}

pub fn part2(input: &Input) -> u64 {
    let (boxes, pairs) = input;
    let mut nodes: Vec<_> = (0..boxes.len()).map(|i| Node { parent: i, size: 1 }).collect();

    for &(i, j, ..) in pairs {
        if union(&mut nodes, i, j) == boxes.len() {
            return boxes[i][0] * boxes[j][0];
        }
    }

    unreachable!()
}

fn find(set: &mut [Node], mut x: usize) -> usize {
    while set[x].parent != x {
        let parent = set[x].parent;
        (x, set[x].parent) = (parent, set[parent].parent);
    }

    x
}

fn union(set: &mut [Node], mut x: usize, mut y: usize) -> usize {
    x = find(set, x);
    y = find(set, y);

    if x != y {
        if set[x].size < set[y].size {
            (x, y) = (y, x);
        }

        set[y].parent = x;
        set[x].size += set[y].size;
    }

    set[x].size
}
