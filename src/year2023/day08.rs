//! # Haunted Wasteland
//!
//! We rely on the input having a very specific structure. Each node ending in `A` has a
//! corresponding node ending in `Z` that forms a *cycle*. The period of this cycle reaching the
//! node ending in `Z` is the [LCM](https://en.wikipedia.org/wiki/Least_common_multiple) of the
//! length of the directions with the length of the cycle. This
//! [visualization](https://www.reddit.com/r/adventofcode/comments/18did3d/2023_day_8_part_1_my_input_maze_plotted_using/)
//! shows the special structure.
//!
//! A [BFS](https://en.wikipedia.org/wiki/Breadth-first_search) from each start node finds the
//! length of each cycle. We only need the total length of the directions.
//!
//! Part one is then a special case of the nodes named `AAA` and `ZZZ`. The answer for part two is
//! the combined LCM of each individual cycle.
//! To combine the list of LCMs from each path we use the identity:
//!
//! `lcm(a, b, c) = lcm(lcm(a, b), c)`
use crate::util::hash::*;
use crate::util::math::*;
use std::collections::VecDeque;

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    let lines: Vec<_> = input.lines().collect();
    let mut nodes = FastMap::with_capacity(lines.len());

    for line in &lines[2..] {
        nodes.insert(&line[0..3], [&line[7..10], &line[12..15]]);
    }

    let mut part_one = lines[0].len();
    let mut part_two = lines[0].len();
    let mut todo = VecDeque::new();
    let mut seen = FastSet::new();

    for &start in nodes.keys().filter(|k| k.ends_with('A')) {
        // Find the length of the cycle using a BFS from each start node.
        todo.push_back((start, 0));
        seen.insert(start);

        while let Some((node, cost)) = todo.pop_front() {
            if node.ends_with('Z') {
                if start == "AAA" {
                    part_one = part_one.lcm(cost);
                }
                part_two = part_two.lcm(cost);
                break;
            }

            for next in nodes[node] {
                if seen.insert(next) {
                    todo.push_back((next, cost + 1));
                }
            }
        }

        todo.clear();
        seen.clear();
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}
