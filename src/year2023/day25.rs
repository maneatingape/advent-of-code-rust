use crate::util::hash::*;
use std::collections::VecDeque;

type Input<'a> = FastMap<&'a str, FastSet<&'a str>>;

pub fn parse(input: &str) -> Input<'_> {
    let mut edges = FastMap::new();

    for line in input.lines() {
        let tokens: Vec<_> = line.split_ascii_whitespace().collect();

        let key = &tokens[0][..3];
        let parent = edges.entry(key).or_insert(FastSet::new());

        for &child in &tokens[1..] {
            parent.insert(child);
        }

        for &child in &tokens[1..] {
            let entry = edges.entry(child).or_insert(FastSet::new());
            entry.insert(key);
        }
    }

    edges
}

pub fn part1(edges: &Input<'_>) -> usize {
    // Find the 3 connections with the highest count.
    // Assume these will be the links.
    let mut freq = FastMap::new();

    for &start in edges.keys() {
        let mut todo = VecDeque::new();
        todo.push_back(start);

        let mut seen = FastSet::new();
        seen.insert(start);

        while let Some(pos) = todo.pop_front() {
            for &next in &edges[pos] {
                if seen.insert(next) {
                    let key = if pos < next { [pos, next] } else { [next, pos] };

                    let entry = freq.entry(key).or_insert(0);
                    *entry += 1;

                    todo.push_back(next);
                }
            }
        }
    }

    let mut order: Vec<_> = freq.iter().collect();
    order.sort_unstable_by_key(|e| e.1);
    order.reverse();

    let cut: Vec<_> = order.iter().take(3).map(|p| *p.0).collect();
    let start = *edges.keys().next().unwrap();
    let mut size = 1;

    let mut todo = VecDeque::new();
    todo.push_back(start);

    let mut seen = FastSet::new();
    seen.insert(start);

    while let Some(pos) = todo.pop_front() {
        for &next in &edges[pos] {
            let key = if pos < next { [pos, next] } else { [next, pos] };

            if cut.contains(&key) {
                continue;
            }

            if seen.insert(next) {
                size += 1;
                todo.push_back(next);
            }
        }
    }

    size * (edges.len() - size)
}

pub fn part2(_input: &Input<'_>) -> &'static str {
    "n/a"
}
