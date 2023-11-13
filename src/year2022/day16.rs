//! # Proboscidea Volcanium
//!
//! ## Parsing
//!
//! First we simplify the graph formed by the valves. With the exception of `AA` there's no need to
//! stop at any zero valve, so we're only interested in the distance between non-zero valves.
//! This significantly reduces the complexity of the solution space as there are only around
//! 15 non-zero valves versus around 60 valves total.
//!
//! For each valve we find the distance to its immediate non-zero neighbors. Then we use the
//! [Floyd Warshall algorithm](https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm) to
//! find the distance between any two non-zero valves, storing this information in an
//! [adjacency matrix](https://en.wikipedia.org/wiki/Adjacency_matrix) for fast lookup.
//!
//! ## Part One
//!
//! The approach is [branch and bound](https://en.wikipedia.org/wiki/Branch_and_bound) enumerating
//! every possible combination combined with a heuristic to prune those combinations in order to
//! achieve a reasonable running time.
//!
//! The heuristic assumes that we can visit all remaining valves in descending order of flow,
//! taking only the minimum possible time to reach each valve. As this will always be better
//! than the actual maximum possible we can immediately prune any branch that would still be less
//! than the current high score.
//!
//! ## Part Two
//!
//! Part two uses an ingenious approach from [@korreman](https://github.com/korreman/aoc2022).
//!
//! First calculate the maximum value for any possible combination of valves reachable in
//! 26 minutes by a single entity. Then calculate a second score from the remaining unopened
//! valves.
//!
//! The neat part is using this second score as the heuristic threshold for a search over all
//! possible valve combinations. This works as the sum of the first two searches provides a
//! minimum baseline. If a branch can't do better then it can be pruned.
//!
//! Then we check every possible pair formed by those values, considering only the pairs
//! where the sets of valves are [disjoint](https://en.wikipedia.org/wiki/Disjoint_sets),
//! which is when you and the elephant have visited different sets of valves.
use crate::util::hash::*;
use crate::util::parse::*;
use std::cmp::Ordering;

/// Simplified graph of valves. Valves are stored in descending order of flow so the valve at
/// index 0 has the highest flow, valve at index 1 the second highest and so on.
/// This descending order is used by the heuristic to prune branches.
///
/// * `size` Number of non-zero valves plus 1 for `AA`
/// * `todo` Bitmask with a `1` for each initial unopened non-zero valve. For example if there
///   are 5 valves this would be binary `11111`.
/// * `flow` Stores the flow for each valve
/// * `distance` Adjacency matrix of distances between each pair of valves.
pub struct Input {
    size: usize,
    aa: usize,
    all_valves: usize,
    flow: Vec<u32>,
    distance: Vec<u32>,
    closest: Vec<u32>,
}

/// State of a single exploration path through the valves.
///
/// * `todo` Binary mask of unopened valves. For example if there are 3 unopened valves left this
///    could look like `11100`.
/// * `from` Index of current valve.
/// * `time` The *remaining* time left.
/// * `pressure` Total pressure released from all opened valves including future extrapolated flow.
struct State {
    todo: usize,
    from: usize,
    time: u32,
    pressure: u32,
}

/// Intermediate struct for parsing only.
struct Valve<'a> {
    name: &'a str,
    flow: u32,
    edges: Vec<&'a str>,
}

impl Valve<'_> {
    /// We're only interested in uppercase valve names and digits for the flow.
    fn parse(line: &str) -> Valve<'_> {
        let mut tokens: Vec<_> = line
            .split(|c: char| !c.is_ascii_uppercase() && !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .collect();
        let name = tokens[1];
        let flow = tokens[2].unsigned();
        tokens.drain(..3);
        Valve { name, flow, edges: tokens }
    }

    /// Order valves is descending order of flow then ascending alpabetical order of names.
    /// This places all non-zero valves at the start followed immediately by valve `AA`.
    fn cmp(&self, other: &Valve<'_>) -> Ordering {
        other.flow.cmp(&self.flow).then(self.name.cmp(other.name))
    }
}

pub fn parse(input: &str) -> Input {
    // Sort valves so that non-zero valves are at the start
    let mut valves: Vec<_> = input.lines().map(Valve::parse).collect();
    valves.sort_unstable_by(Valve::cmp);

    // We only care about non-zero valves with the exception of `AA`.
    let size = valves.iter().filter(|v| v.flow > 0).count() + 1;
    let mut distance = vec![u32::MAX; size * size];

    // Eliminate zero valves. Assumes that zero valves are "tunnels" with each linking 2 other
    // valves. For all non-zero valves follows the tunnels to find the distance to each
    // immediate neighbor.
    let indices: FastMap<_, _> = valves.iter().enumerate().map(|(i, v)| (v.name, i)).collect();

    for (from, valve) in valves.iter().enumerate().take(size) {
        // Distance to ourself is zero.
        distance[from * size + from] = 0;

        // Follow "tunnels" of zero valves to our non-zero neighbors.
        for edge in &valve.edges {
            let mut prev = valve.name;
            let mut cur = edge;
            let mut to = indices[cur];
            let mut total = 1;

            while to >= size {
                let next = valves[to].edges.iter().find(|&&e| e != prev).unwrap();
                prev = cur;
                cur = next;
                to = indices[cur];
                total += 1;
            }

            distance[from * size + to] = total;
        }
    }

    // Floyd-Warshall algorithm to find the pairwise distance between any two valves.
    for k in 0..size {
        for i in 0..size {
            for j in 0..size {
                let candidate = distance[i * size + k].saturating_add(distance[k * size + j]);
                if candidate < distance[i * size + j] {
                    distance[i * size + j] = candidate;
                }
            }
        }
    }

    // Add 1 minute to each distance to include the time needed to open a valve.
    distance.iter_mut().for_each(|d| *d += 1);
    // Index of AA is one less than size
    let aa = size - 1;
    // Binary mask of all initial unopened valves not including AA.
    let all_valves = (1 << aa) - 1;
    // Extract flow information.
    let flow: Vec<_> = valves.iter().take(size).map(|v| v.flow).collect();
    // Closest neighbor to each valve
    let closest: Vec<_> = distance
        .chunks_exact(size)
        .map(|chunk| *chunk.iter().filter(|&&d| d > 1).min().unwrap())
        .collect();

    // Compact representation of tunnels and valves.
    Input { size, aa, all_valves, flow, distance, closest }
}

/// Explore the tunnels, finding the highest possible score for a single entity.
pub fn part1(input: &Input) -> u32 {
    let mut score = 0;
    // Return the current high score for the heuristic.
    let mut high_score = |_, pressure: u32| {
        score = score.max(pressure);
        score
    };

    let start = State { todo: input.all_valves, from: input.aa, time: 30, pressure: 0 };
    explore(input, &start, &mut high_score);

    score
}

/// Return the maximum possible score from two entities exploring the tunnels simultaneously.
pub fn part2(input: &Input) -> u32 {
    // Step 1
    // Find both the highest possible score and the remaining unopened valves from you
    // exploring the tunnels.
    let mut you = 0;
    let mut remaining = 0;
    // Keep track of the unopened valves associated with the high score.
    let mut high_score = |todo: usize, pressure: u32| {
        if pressure > you {
            you = pressure;
            remaining = todo;
        }
        you
    };

    let first = State { todo: input.all_valves, from: input.aa, time: 26, pressure: 0 };
    explore(input, &first, &mut high_score);

    // Step 2
    // Find the highest possible score when only allowing the unopened valves from the
    // previous run. This will set a minimum baseline score for the heuristic.
    let mut elephant = 0;
    let mut high_score = |_, pressure: u32| {
        elephant = elephant.max(pressure);
        elephant
    };

    let second = State { todo: remaining, from: input.aa, time: 26, pressure: 0 };
    explore(input, &second, &mut high_score);

    // Step 3
    // Explore a third time allowing only scores that are higher than the previous minimum.
    // Instead of a single score, store the high score for each possible `2ⁱ` combinations
    // of valves. The index of the score is the bitmask of the *opened* valves.
    let mut score = vec![0; input.all_valves + 1];
    let mut high_score = |todo: usize, pressure: u32| {
        let done = input.all_valves ^ todo;
        score[done] = score[done].max(pressure);
        // Always return the elephant value from step 2 for the heuristic.
        elephant
    };

    let third = State { todo: input.all_valves, from: input.aa, time: 26, pressure: 0 };
    explore(input, &third, &mut high_score);

    // Combine the score using the disjoint sets approach. As no valve can be opened twice
    // only consider scores where there is no overlap by using a bitwise AND.
    let mut result = you + elephant;

    // Find valid non-zero results then sort in order to check combinations faster.
    let mut candidates: Vec<_> = score.into_iter().enumerate().filter(|(_, s)| *s > 0).collect();
    candidates.sort_unstable_by_key(|t| t.1);

    for i in (1..candidates.len()).rev() {
        let (mask1, you) = candidates[i];

        // Since results are sorted, all subsequent scores are lower than this one.
        // If the maximum possible sum from remaining scores is lower than the current result
        // then we're done.
        if you * 2 <= result {
            break;
        }

        for j in (0..i).rev() {
            let (mask2, elephant) = candidates[j];

            // Find the best result where the two sets of valves are disjoint.
            if mask1 & mask2 == 0 {
                result = result.max(you + elephant);
                break;
            }
        }
    }

    result
}

fn explore(input: &Input, state: &State, high_score: &mut impl FnMut(usize, u32) -> u32) {
    let State { todo, from, time, pressure } = *state;
    let score = high_score(todo, pressure);
    let mut valves = todo;

    while valves > 0 {
        // Stores the set of unopened valves in a single integer as a bit mask with a 1
        // for each unopened valve. This code iterates over each valve by finding the lowest
        // 1 bit then removing it from the set.
        let to = valves.trailing_zeros() as usize;
        let mask = 1 << to;
        valves ^= mask;

        // Check if there's enough time to reach the valve.
        let needed = input.distance[from * input.size + to];
        if needed >= time {
            continue;
        }

        // Calculate the total pressure released by a valve up front.
        let todo = todo ^ mask;
        let time = time - needed;
        let pressure = pressure + time * input.flow[to];

        // Pretend that we could visit each remaining unopened valve in descending order
        // of flow taking only the minimum possible time to reach each valve. As this is always
        // better than the actual graph if we can't beat the high score then we can prune
        // this branch right away.
        let heuristic = {
            let mut valves = todo;
            let mut time = time;
            let mut pressure = pressure;

            // Assume that all valves have a distance of 3 or more.
            while valves > 0 && time > 3 {
                let to = valves.trailing_zeros() as usize;
                valves ^= 1 << to;
                time -= input.closest[to];
                pressure += time * input.flow[to];
            }

            pressure
        };

        // Only explore further if it's possible to beat the high score.
        if heuristic > score {
            let next = State { todo, from: to, time, pressure };
            explore(input, &next, high_score);
        }
    }
}
