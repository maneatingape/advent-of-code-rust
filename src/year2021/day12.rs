use crate::util::iter::*;
use std::collections::HashMap;

const START: usize = 0;
const END: usize = 1;

pub struct Input {
    small: u32,
    edges: Vec<u32>,
}

#[derive(Hash, PartialEq, Eq)]
struct State {
    from: usize,
    visited: u32,
    twice: bool,
}

pub fn parse(input: &str) -> Input {
    let tokens: Vec<_> = input
        .split(|c: char| !c.is_ascii_alphabetic())
        .filter(|s| !s.is_empty())
        .collect();

    let mut indices = HashMap::from([("start", START), ("end", END)]);
    for token in &tokens {
        if !indices.contains_key(token) {
            indices.insert(token, indices.len());
        }
    }

    let mut edges = vec![0; indices.len()];
    for [a, b] in tokens.iter().chunk::<2>() {
        edges[indices[a]] |= 1 << indices[b];
        edges[indices[b]] |= 1 << indices[a];
    }
    let not_start = !(1 << START);
    edges.iter_mut().for_each(|edge| *edge &= not_start);

    let mut small = 0;
    for (key, value) in indices.iter() {
        if key.chars().next().unwrap().is_ascii_lowercase() {
            small |= 1 << value;
        }
    }

    Input { small, edges }
}

pub fn part1(input: &Input) -> u32 {
    explore(input, false)
}

pub fn part2(input: &Input) -> u32 {
    explore(input, true)
}

fn explore(input: &Input, twice: bool) -> u32 {
    let state = State {
        from: START,
        visited: 0,
        twice,
    };
    let mut cache = HashMap::new();
    paths(input, state, &mut cache)
}

fn paths(input: &Input, state: State, cache: &mut HashMap<State, u32>) -> u32 {
    let State { from, visited, twice } = state;

    if from == END {
        return 1;
    }
    if let Some(total) = cache.get(&state) {
        return *total;
    }

    let mut caves = input.edges[from];
    let mut total = 0;

    while caves != 0 {
        let to = caves.trailing_zeros() as usize;
        let mask = 1 << to;
        caves ^= mask;

        let once = input.small & mask == 0 || visited & mask == 0;
        if once || twice {
            let next = State {
                from: to,
                visited: visited | mask,
                twice: once && twice,
            };
            total += paths(input, next, cache);
        }
    }

    cache.insert(state, total);
    total
}
