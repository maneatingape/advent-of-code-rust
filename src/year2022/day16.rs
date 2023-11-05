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
//! taking only 3 minutes to travel and open each valve (pretending that the valves are separated
//! by only a single intermediate corridor). As this will always be better than the actual
//! maximum possible we can immediately prune any branch that would still be less than the
//! current high score.
//!
//! ## Part Two
//!
//! We take a different approach, treating both "you" and the elephant independently.
//!
//! First we calculate the maximum value for any possible combination of valves reachable in
//! 26 minutes by a single entity. Of the total possible 32768 valve combinations in my input
//! around 3000 or 10% were reachable in the time available.
//!
//! Then we check every possible pair formed by those values, considering only the pairs
//! where the sets of valves are [disjoint](https://en.wikipedia.org/wiki/Disjoint_sets),
//! which is when you and the elephant have visited different sets of valves.
//!
//! The maximum value is our answer. Brute force would take `O(n²)` or ~9 x 10⁶ comparisons but
//! we can use dynamic programming to check in only `2ⁱ` where `i` is the number of non-zero
//! valves.
use crate::util::hash::*;
use crate::util::parse::*;
use std::cmp::Ordering;
use std::collections::VecDeque;

/// Simplified graph of valves. Valves are stored in descending order of flow so the valve at
/// index 0 has the highest flow, valve at index 1 the second highest and so on.
///
/// * `size` Number of non-zero valves plus 1 for `AA`
/// * `todo` Bitmask with a `1` for each initial unopened non-zero valve. For example if there
///   are 5 valves this would be binary `11111`.
/// * `flow` Stores the flow for each valve
/// * `distance` Adjacency matrix of distances between each pair of valves.
pub struct Input {
    size: usize,
    todo: usize,
    flow: Vec<u32>,
    distance: Vec<u32>,
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
    // Binary mask of all initial unopened valves.
    let todo = (1 << (size - 1)) - 1;
    // Extract flow information.
    let flow: Vec<_> = valves.iter().take(size).map(|v| v.flow).collect();
    // Compact representation of tunnels and valves.
    Input { size, todo, flow, distance }
}

pub fn part1(input: &Input) -> u32 {
    let mut score = 0;

    // Interestingly a regular vec is faster than both a traditional BFS using a VecDeque or a
    // recursive DFS.
    let mut queue = Vec::new();
    // Out starting location `AA` has the highest index.
    queue.push(State { todo: input.todo, from: input.size - 1, time: 30, pressure: 0 });

    while let Some(State { todo, from, time, pressure }) = queue.pop() {
        // Try to increase the high score.
        score = score.max(pressure);
        let mut valves = todo;

        while valves > 0 {
            // Stores the set of unopened valves in a single integer as a bit mask with a 1
            // for each unopened valve. This code iterates over each valve by finding the lowest
            // 1 bit then removing it from the set.
            let to = valves.trailing_zeros() as usize;
            let mask = 1 << to;
            valves ^= mask;

            let needed = input.distance[from * input.size + to];
            if needed < time {
                let remaining = time - needed;
                let flow = input.flow[to];

                // Pretend that we could visit each remaining unopened valve in descending order
                // of flow with only a tunnel of 1 length between them. As this is always better
                // than the actual graph if we can't beat the high score then we can prune
                // this branch right away.
                let heuristic = {
                    let mut pressure = pressure + remaining * flow;
                    let mut valves = todo ^ mask;
                    let mut remaining = remaining;

                    while valves > 0 && remaining > 3 {
                        remaining -= 3;
                        let to = valves.trailing_zeros() as usize;
                        let flow = input.flow[to];
                        pressure += remaining * flow;
                        valves ^= 1 << to;
                    }

                    pressure
                };

                // Push the next state. We calculate the total pressure released by a
                // valve up front.
                if heuristic > score {
                    let next = State {
                        todo: todo ^ mask,
                        from: to,
                        time: remaining,
                        pressure: pressure + remaining * flow,
                    };
                    queue.push(next);
                }
            }
        }
    }

    score
}

pub fn part2(input: &Input) -> u32 {
    // Instead of a single score, store the high score for each possible `2ⁱ` combinations of valves.
    let mut score = vec![0; input.todo + 1];

    // Tradtional BFS is fastest.
    let mut queue = VecDeque::with_capacity(input.todo);
    queue.push_back(State { todo: input.todo, from: input.size - 1, time: 26, pressure: 0 });

    // Cache seen states before. If we're in the same location with the same valves opened
    // but a lower score then we can prune the state.
    let stride = score.len();
    let mut cache = vec![0; stride * (input.size - 1)];

    while let Some(State { todo, from, time, pressure }) = queue.pop_front() {
        let done = todo ^ input.todo;
        score[done] = score[done].max(pressure);

        let mut valves = todo;
        while valves > 0 {
            // Same bitwise set approach as part one.
            let to = valves.trailing_zeros() as usize;
            let mask = 1 << to;
            valves ^= mask;

            let needed = input.distance[from * input.size + to];
            if needed < time {
                let next = todo ^ mask;
                let remaining = time - needed;
                let flow = input.flow[to];
                let pressure = pressure + remaining * flow;

                // No heuristic as in part one as we need to check every possible path.
                // We can eliminate worse duplicate states though.
                let index = stride * to + next;
                if cache[index] == 0 || cache[index] < pressure {
                    cache[index] = pressure;
                    queue.push_back(State { todo: next, from: to, time: remaining, pressure });
                }
            }
        }
    }

    // Only around 10% of the possible 32768 combinations are valid. Dynamically fill in the
    // maximum possible values for the remaining combination so that we can compare disjoint sets.
    let mut result = 0;
    let mut visited: Vec<_> = score.iter().map(|&s| s > 0).collect();
    subsets(input.todo, &mut score, &mut visited);

    // Treat the bitwise representation of the index as the valves visited by "you".
    for (i, you) in score.iter().enumerate() {
        let elephant = score[input.todo ^ i];
        result = result.max(you + elephant);
    }

    result
}

/// Dynamically finds the maximum score for any combination of valves.
///
/// For example, say there are 4 valves `abcd`, but it's only possible to visit 3 in the time
/// available. This calculates the value of `abcd` as the maximum of `bcd`, `acd`, `abd` and `abc`.
fn subsets(todo: usize, score: &mut [u32], visited: &mut [bool]) -> u32 {
    let mut valves = todo;
    let mut max = 0;

    while valves != 0 {
        // Visit subsets by removing one bit at a time.
        let mask = 1 << valves.trailing_zeros();
        let next = todo ^ mask;
        let result = if visited[next] { score[next] } else { subsets(next, score, visited) };
        valves ^= mask;
        max = max.max(result);
    }

    // Memoize computed values.
    score[todo] = max;
    visited[todo] = true;
    max
}
