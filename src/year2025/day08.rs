//! # Playground
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::thread::*;

type Box = [usize; 3];
type Pair = (u16, u16, usize);
type Input = (Vec<Box>, Vec<Vec<Vec<Pair>>>);

struct Node {
    parent: usize,
    size: usize,
}

const BUCKETS: usize = 5;
const SIZE: usize = 10_000 * 10_000;

pub fn parse(input: &str) -> Input {
    let boxes: Vec<_> = input.iter_unsigned::<usize>().chunk::<3>().collect();
    let items: Vec<_> = (0..boxes.len()).collect();
    let mut buckets = vec![vec![]; BUCKETS];

    for result in spawn_parallel_iterator(&items, |iter| worker(&boxes, iter)) {
        for (bucket, pairs) in buckets.iter_mut().zip(result) {
            bucket.push(pairs);
        }
    }

    (boxes, buckets)
}

pub fn part1(input: &Input) -> usize {
    part1_testable(input, 1000)
}

pub fn part1_testable(input: &Input, limit: usize) -> usize {
    let (boxes, buckets) = input;
    let mut nodes: Vec<_> = (0..boxes.len()).map(|i| Node { parent: i, size: 1 }).collect();

    for (i, j, ..) in flatten(buckets).take(limit) {
        union(&mut nodes, i as usize, j as usize);
    }

    nodes.sort_unstable_by_key(|node| node.size);
    nodes.iter().rev().take(3).map(|node| node.size).product()
}

pub fn part2(input: &Input) -> usize {
    let (boxes, buckets) = input;
    let mut nodes: Vec<_> = (0..boxes.len()).map(|i| Node { parent: i, size: 1 }).collect();

    for (i, j, ..) in flatten(buckets) {
        let (i, j) = (i as usize, j as usize);

        if union(&mut nodes, i, j) == boxes.len() {
            return boxes[i][0] * boxes[j][0];
        }
    }

    unreachable!()
}

fn worker(boxes: &[Box], iter: ParIter<'_, usize>) -> Vec<Vec<Pair>> {
    let mut buckets = vec![vec![]; BUCKETS];

    for &i in iter {
        let v1 = boxes[i];

        for (j, &v2) in boxes.iter().enumerate().skip(i + 1) {
            let dx = v1[0].abs_diff(v2[0]);
            let dy = v1[1].abs_diff(v2[1]);
            let dz = v1[2].abs_diff(v2[2]);
            let distance = dx * dx + dy * dy + dz * dz;

            let index = (distance / SIZE).min(BUCKETS - 1);
            buckets[index].push((i as u16, j as u16, distance));
        }
    }

    buckets
}

fn flatten(buckets: &[Vec<Vec<Pair>>]) -> impl Iterator<Item = Pair> {
    buckets.iter().flat_map(|pairs| {
        let mut merged = pairs.concat();
        merged.sort_unstable_by_key(|&(.., distance)| distance);
        merged
    })
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
