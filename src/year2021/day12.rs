//! # Passage Pathing
//!
//! Our basic approach is a [DFS](https://en.wikipedia.org/wiki/Depth-first_search) through the cave
//! system, exploring all possible permutations of the paths and finishing whenever we reach
//! the `end` cave.
//!
//! To speed things up, 2 strategies are used, one high level and one low level:
//! * [Memoization](https://en.wikipedia.org/wiki/Memoization) (or caching) of the possible paths
//!   from each position, taking into account previously visited caves is the high level strategy
//!   to re-use work and save time.
//! * [Bit Manipulation](https://en.wikipedia.org/wiki/Bit_manipulation) to store both the graph of
//!   cave connections as an [adjacency matrix](https://en.wikipedia.org/wiki/Adjacency_matrix)
//!   and the list of visited caves compressed into a single `u32` is the low level strategy to
//!   quickly and efficiently store the small cardinality set of caves.
use crate::util::iter::*;
use std::collections::HashMap;

const START: usize = 0;
const END: usize = 1;

pub struct Input {
    small: u32,
    edges: Vec<u32>,
}

struct State {
    from: usize,
    visited: u32,
    twice: bool,
}

/// Parse the input into an adjency matrix of edges compressed into `u32` bitfields.
///
/// First, each cave is assigned a unique index, with `0` reserved for the `start` cave and `1`
/// reserved for the `end` cave. For example the sample input caves are:
///
/// | start | end | A | b | c | d |
/// | :---: | :-: | - | - | - | - |
/// |   0   |  1  | 2 | 3 | 4 | 5 |
///
/// Next a `vec` of `u32` with an entry for each cave at the corresponding index is created with
/// a bit set for each other cave reachable at `2^n` where n is the cave index. The start cave
/// can only be visited once at the beginning, so it is removed from all edges.
/// For example the sample start cave `vec` looks like:
///
/// | cave  | index | edges  |
/// | ----- | ----- | ------ |
/// | start | 0     |   1100 |
/// | end   | 1     |   1100 |
/// | A     | 2     |  11010 |
/// | b     | 3     | 100110 |
/// | c     | 4     |    100 |
/// | d     | 5     |   1000 |
///
/// Finally all small caves are added to a single `u32`, for example the
/// sample data looks like `111011`.
pub fn parse(input: &str) -> Input {
    let tokens: Vec<_> =
        input.split(|c: char| !c.is_ascii_alphabetic()).filter(|s| !s.is_empty()).collect();

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

/// Explore the cave system visiting all small caves only once.
pub fn part1(input: &Input) -> u32 {
    explore(input, false)
}

/// Explore the cave system visiting a single small cave twice and the other small caves only once.
pub fn part2(input: &Input) -> u32 {
    explore(input, true)
}

/// Convenience method to create initial state.
fn explore(input: &Input, twice: bool) -> u32 {
    // Calculate the needed size of the cache as the product of:
    // * 2 states for boolean "twice".
    // * n states for the number of caves including start and end.
    // * 2^(n-2) states for the possible visited combinations, not including start and end cave.
    let size = 2 * input.edges.len() * (1 << (input.edges.len() - 2));
    let mut cache = vec![0; size];

    let state = State { from: START, visited: 0, twice };
    paths(input, state, &mut cache)
}

/// Core recursive DFS logic.
///
/// First we check if we have either reached the `end` cave or seen this state before,
/// returning early in either case with the respective result.
///
/// Next we use bit manipulation to quickly iterate through the caves connnected to our current
/// location. The [`trailing_zeros`] method returns the next set bit. This instrinsic compiles to
/// a single machine code instruction on x86 and ARM and is blazing fast. We remove visited caves
/// using a `^` XOR instruction.
///
/// The nuance is re-using the same code for both part 1 and part 2. First we check if we can visit
/// a cave using the rules for part 1. If not, then we also check if the `twice` variable is
/// still `true`. This variable allows a single second visit to a small cave. The expression
/// `once && twice` sets this value to `false` whenever we need to use it to visit a small cave.
///
/// [`trailing_zeros`]: u32::trailing_zeros
fn paths(input: &Input, state: State, cache: &mut [u32]) -> u32 {
    let State { from, visited, twice } = state;

    // Calculate index by converting "twice" to either 1 or 0, then multiplying "from" by 2
    // (the cardinality of "twice") and "visited" by "edges.len()".
    // Subtle nuance, by not multiplying "visited" by 2 and also dividing by 2 we ignore the
    // two least significant bits for start and end cave, as these will always be 0 and 1
    // respectively.
    let index =
        state.twice as usize + 2 * (state.from) + (input.edges.len() * (state.visited as usize / 2));
    let total = cache[index];
    if total > 0 {
        return total;
    }

    let mut caves = input.edges[from];
    let mut total = 0;
    let mut mask = 1 << END;

    if caves & mask != 0 {
        caves ^= mask;
        total += 1;
    }

    while caves != 0 {
        let to = caves.trailing_zeros() as usize;
        mask = 1 << to;
        caves ^= mask;

        let once = input.small & mask == 0 || visited & mask == 0;
        if once || twice {
            let next = State { from: to, visited: visited | mask, twice: once && twice };
            total += paths(input, next, cache);
        }
    }

    cache[index] = total;
    total
}
