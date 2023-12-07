//! # Haunted Wasteland
//!
//! To speed things up, each node name is first converted into a unique index. Then we can navigate
//! the nodes quickly using two `vec`s, one for the left direction and another for the
//! right direction.
//!
//! For part two, the answer is the combined [LCM](https://en.wikipedia.org/wiki/Least_common_multiple)
//! of each individual path. To combine the list of LCMs from each path we use the identity:
//!
//! `lcm(a, b, c) = lcm(lcm(a, b), c)`
use crate::util::hash::*;
use crate::util::math::*;

pub struct Input<'a> {
    directions: &'a [u8],
    left: Vec<usize>,
    right: Vec<usize>,
    part_one_start: usize,
    part_one_end: usize,
    part_two_start: Vec<usize>,
    part_two_end: Vec<bool>,
}

pub fn parse(input: &str) -> Input<'_> {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
    let size = lines.len() - 2;

    let mut indices = FastMap::with_capacity(size);
    let mut left = Vec::with_capacity(size);
    let mut right = Vec::with_capacity(size);
    let mut part_two_start = Vec::with_capacity(size);
    let mut part_two_end = Vec::with_capacity(size);

    for (i, line) in lines[2..].iter().enumerate() {
        let key = &line[0..3];
        indices.insert(key, i);

        if key[2] == b'A' {
            part_two_start.push(i);
        }

        part_two_end.push(key[2] == b'Z');
    }

    // Convert 3 letter node name to index.
    for line in &lines[2..] {
        left.push(indices[&line[7..10]]);
        right.push(indices[&line[12..15]]);
    }

    let directions = lines[0];
    let part_one_start = indices["AAA".as_bytes()];
    let part_one_end = indices["ZZZ".as_bytes()];

    Input { directions, left, right, part_one_start, part_one_end, part_two_start, part_two_end }
}

pub fn part1(input: &Input<'_>) -> u32 {
    let Input { left, right, part_one_start, .. } = input;
    let mut node = *part_one_start;
    let mut directions = input.directions.iter().copied().cycle();
    let mut steps = 0;

    while node != input.part_one_end {
        node = if directions.next().unwrap() == b'L' { left[node] } else { right[node] };
        steps += 1;
    }

    steps
}

pub fn part2(input: &Input<'_>) -> u64 {
    let Input { left, right, part_two_start, part_two_end, .. } = input;
    let mut result = 1;

    for &start in part_two_start {
        let mut node = start;
        let mut directions = input.directions.iter().copied().cycle();
        let mut steps = 0;

        while !part_two_end[node] {
            node = if directions.next().unwrap() == b'L' { left[node] } else { right[node] };
            steps += 1;
        }

        // Use LCM identity to combine each path.
        result = result.lcm(steps);
    }

    result
}
