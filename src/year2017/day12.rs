//! # Digital Plumber
//!
//! This problem is the classic [union-find](https://en.wikipedia.org/wiki/Disjoint-set_data_structure).
//! A variant of [flood fill](https://en.wikipedia.org/wiki/Flood_fill) is used to find the
//! connected groups or cliques.
//!
//! For each program we [depth-first search](https://en.wikipedia.org/wiki/Depth-first_search)
//! from each of its neighbors that we have not already seen. If a neighbor has been seen
//! then it must be either already in this clique or in another clique.
use crate::util::parse::*;

type Input = (u32, usize);

pub fn parse(input: &str) -> Input {
    let lines: Vec<_> = input.lines().collect();
    let size = lines.len();

    let mut seen = vec![false; size];
    let part_one = dfs(&lines, &mut seen, 0);
    let part_two = 1 + (1..size).filter(|&i| dfs(&lines, &mut seen, i) > 0).count();

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

#[inline]
fn dfs(lines: &[&str], seen: &mut [bool], index: usize) -> u32 {
    if seen[index] {
        0
    } else {
        seen[index] = true;
        // Skip the first 6 characters of each line as it contains the index that we already know.
        1 + (&lines[index][6..]).iter_unsigned::<usize>().map(|i| dfs(lines, seen, i)).sum::<u32>()
    }
}
