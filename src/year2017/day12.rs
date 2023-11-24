//! # Digital Plumber
//!
//! This problem is the classic [union-find](https://en.wikipedia.org/wiki/Disjoint-set_data_structure).
//! A variant of [flood fill](https://en.wikipedia.org/wiki/Flood_fill) is used to find the
//! connected groups or cliques.
//!
//! For each program we [depth first search](https://en.wikipedia.org/wiki/Depth-first_search)
//! from each of its neighbors that we have not already visited. If a neighbor has been visited
//! then it must be either already in this clique or in another clique.
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<u32> {
    let lines: Vec<_> = input.lines().collect();
    let size = lines.len();

    let mut visited = vec![false; size];
    let mut groups = Vec::new();

    for start in 0..size {
        // DFS from each unvisited program.
        if !visited[start] {
            visited[start] = true;
            let size = dfs(&lines, &mut visited, start);
            groups.push(size);
        }
    }

    groups
}

pub fn part1(input: &[u32]) -> u32 {
    input[0]
}

pub fn part2(input: &[u32]) -> usize {
    input.len()
}

fn dfs(lines: &[&str], visited: &mut [bool], index: usize) -> u32 {
    let mut size = 1;

    // At least the first 6 characters of each line can be skipped as it only contains the index
    // that we already know.
    for next in (&lines[index][6..]).iter_unsigned::<usize>() {
        if !visited[next] {
            visited[next] = true;
            size += dfs(lines, visited, next);
        }
    }

    size
}
