//! # Many-Worlds Interpretation
//!
//! Our high-level approach is to simplify the problem into graph pathfinding. We only
//! ever need to move directly from key to key, so the maze becomes a graph where the nodes are
//! keys and the edge weight is the distance between keys. Doors modify which edges
//! are connected depending on the keys currently possessed.
//!
//! We first find the distance between every pair of keys then run the
//! [A* algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm) to find the
//! shortest path that visits every node in the graph. The heuristic is the sum of the minimum
//! possible distances to reach every remaining key, which helps the search focus on the
//! overall minimum rather than the nearest key, for fewer nodes explored. Since the heuristic
//! is consistent, no node will ever ever have its score reduced after the first visit.
//!
//! The maze is also constructed in such a way to make our life easier:
//! * There is only ever one possible path to each key. We do not need to consider
//!   paths of different lengths that need different keys.
//! * As a corollary, if key `b` lies between `a` and `c` then `|ac| = |ab| + |bc|`. This
//!   enables a huge optimization that we only need to consider immediate neighbors.
//!   If we do not possess key `b` then it never makes sense to skip from `a` to `c` since `b` is
//!   along the way. We can model this by treating keys the same as doors. This optimization
//!   sped up my solution by a factor of 30.
//!
//! On top of this approach we apply some high-level tricks to go faster:
//! * We store previously seen pairs of `(location, keys collected)` to `total distance` in a map.
//!   If we are in the same location with the same keys but at a higher cost, then this situation
//!   can never be optimal so the solution can be discarded.
//! * When finding the distance between every pair of keys, it's faster to first only find the immediate
//!   neighbors of each key using a [Breadth-first search](https://en.wikipedia.org/wiki/Breadth-first_search)
//!   then run the [Floyd-Warshall algorithm](https://en.wikipedia.org/wiki/Floyd-Warshall_algorithm)
//!   to construct the rest of the graph. Even though the Floyd-Warshall asymptotic bound of `O(n³)`
//!   is higher than the asymptotic bounds of repeated BFS, this was twice as fast in practice
//!   for my input.
//!
//! We also apply some low-level tricks to go even faster:
//! * The set of remaining keys needed is stored as bits in a `u32`. We can have at most 26 keys
//!   so this will always fit. For example, needing `a`, `b` and `e` is represented as `10011`.
//! * Robot location is also stored the same way. Robots can only ever be in their initial location
//!   or at a key, so this gives a max of 26 + 4 = 30 locations. As a nice bonus this allows
//!   part one and part two to share the same code.
//! * For fast lookup of distance between keys, the maze is stored as
//!   [adjacency matrix](https://en.wikipedia.org/wiki/Adjacency_matrix). `a` is index 0, `b` is
//!   index 1 and robots' initial positions are from 26 to 29 inclusive.
//!   For example (simplifying by moving robot from index 26 to 2):
//!
//!   ```none
//!       #########    [0 6 2]
//!       #b.A.@.a# => [6 0 4]
//!       #########    [2 4 0]
//!   ```
use crate::util::bitset::*;
use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::heap::*;
use std::collections::VecDeque;
use std::ops::Range;

const RANGE: Range<usize> = 0..30;

/// `position` and `remaining` are both bitfields. For example, a robot at key `d` that needs
/// `b` and `c` would be stored as `position = 1000` and `remaining = 110`.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct State {
    position: u32,
    remaining: u32,
}

/// `distance` in the edge weight between nodes. `needed` stores any doors in between as a bitfield.
#[derive(Clone, Copy)]
struct Door {
    distance: u32,
    needed: u32,
}

type Matrix = [[Door; 30]; 30];

/// `initial` is the complete set of keys that we need to collect. Will always be binary
/// `11111111111111111111111111` for the real input but fewer for sample data.
///
/// `masks` maps the set of keys in the same quadrant, for prefiltering in part 2.
/// `minimum` is the smallest distance from a key to any of its neighbors, for the A* heuristic.
/// `matrix` is the adjacency of distances and doors between each pair of keys and the robots'
/// starting locations.
struct Maze {
    initial: State,
    masks: [u32; 30],
    minimum: [u32; 26],
    matrix: Matrix,
}

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(input: &Grid<u8>) -> u32 {
    explore(input.width as usize, &input.bytes)
}

pub fn part2(input: &Grid<u8>) -> u32 {
    let mut modified = input.bytes.clone();
    let mut patch = |s: &str, offset: i32| {
        let middle = (input.width * input.height) / 2;
        let index = (middle + offset * input.width - 1) as usize;
        modified[index..index + 3].copy_from_slice(s.as_bytes());
    };

    patch("@#@", -1);
    patch("###", 0);
    patch("@#@", 1);

    explore(input.width as usize, &modified)
}

fn parse_maze(width: usize, bytes: &[u8]) -> Maze {
    let mut initial = State::default();
    let mut found = Vec::new();
    let mut robots = 26;
    let mut quadrants = [0_u32; 4];

    // Find the location of every key and robot in the maze.
    // Sort keys into quadrants based on location compared to the midpoint.
    assert_eq!(width % 2, 1);
    assert_eq!((bytes.len() / width) % 2, 1);
    for (i, &b) in bytes.iter().enumerate() {
        let quad =
            2 * (i / width < bytes.len() / width / 2) as usize + (i % width < width / 2) as usize;
        if let Some(key) = is_key(b) {
            initial.remaining |= 1 << key;
            quadrants[quad] |= 1 << key;
            found.push((i, key));
        }
        if b == b'@' {
            initial.position |= 1 << robots;
            quadrants[quad] |= 1 << robots;
            found.push((i, robots));
            robots += 1;
        }
    }
    if robots == 27 {
        quadrants[0] |= quadrants[1] | quadrants[2] | quadrants[3];
        quadrants[1] = 0;
        quadrants[2] = 0;
        quadrants[3] = 0;
    }

    // Start a BFS from each key and robot's location stopping at the nearest neighbor.
    // As a minor optimization we reuse the same `todo` and `seen` between each search.
    let default = Door { distance: u32::MAX, needed: 0 };

    let mut matrix = [[default; 30]; 30];
    let mut seen = vec![usize::MAX; bytes.len()];
    let mut todo = VecDeque::new();
    let mut masks = [0; 30];

    for (start, from) in found {
        todo.push_front((start, 0, 0));
        seen[start] = from;

        while let Some((index, distance, mut needed)) = todo.pop_front() {
            if let Some(door) = is_door(bytes[index]) {
                needed |= 1 << door;
            }

            if let Some(to) = is_key(bytes[index])
                && distance > 0
            {
                // Store the reciprocal edge weight and doors in the adjacency matrix.
                matrix[from][to] = Door { distance, needed };
                matrix[to][from] = Door { distance, needed };
                // Faster to stop here and use Floyd-Warshall later.
                continue;
            }

            for next in [index + 1, index - 1, index + width, index - width] {
                if bytes[next] != b'#' && seen[next] != from {
                    todo.push_back((next, distance + 1, needed));
                    seen[next] = from;
                }
            }
        }
    }

    // Fill in the rest of the graph using the Floyd-Warshall algorithm.
    // As a slight twist we also build the list of intervening doors at the same time.
    for i in RANGE {
        matrix[i][i].distance = 0;
        for mask in quadrants {
            if mask & (1 << i) != 0 {
                masks[i] = mask;
            }
        }
    }

    for k in RANGE {
        for i in RANGE {
            for j in RANGE {
                let candidate = matrix[i][k].distance.saturating_add(matrix[k][j].distance);
                if matrix[i][j].distance > candidate {
                    matrix[i][j].distance = candidate;
                    // `(1 << k)` is a crucial optimization. By treating intermediate keys like
                    // doors we speed things up by a factor of 30.
                    matrix[i][j].needed = matrix[i][k].needed | (1 << k) | matrix[k][j].needed;
                }
            }
        }
    }

    let mut minimum = [0; 26];
    for i in initial.remaining.biterator() {
        minimum[i] = matrix[i]
            .iter()
            .take(initial.remaining.count_ones() as usize)
            .map(|d| d.distance)
            .filter(|&dist| dist > 0)
            .min()
            .unwrap_or(0);
    }

    Maze { initial, masks, minimum, matrix }
}

fn explore(width: usize, bytes: &[u8]) -> u32 {
    let mut todo = MinHeap::with_capacity(5_000);
    let mut cache = FastMap::with_capacity(5_000);

    let Maze { initial, masks, minimum, matrix } = parse_maze(width, bytes);
    let heuristic: u32 = minimum.iter().sum();
    todo.push(heuristic, (initial, heuristic));

    while let Some((guess, (State { position, remaining }, heuristic))) = todo.pop() {
        let total = guess - heuristic;
        // Finish immediately if no keys left.
        // Since we're using A* with a consistent heuristic this will always be the optimal solution.
        if remaining == 0 {
            return total;
        }

        // Avoid next-neighbor checks if this state was visited in the meantime by a better path.
        if let Some(&best) = cache.get(&State { position, remaining })
            && total > best
        {
            continue;
        }

        // The set of robots is stored as bits in a `u32` shifted by the index of the location.
        for from in position.biterator() {
            // The set of keys still needed is also stored as bits in a `u32` similarly to robots.
            // Filter the list of destinations to keys in the same quadrant.
            for to in (remaining & masks[from]).biterator() {
                let Door { distance, needed } = matrix[from][to];

                // Don't move to a key that still has unmet dependencies.
                if remaining & needed == 0 {
                    let next_total = total + distance;
                    let from_mask = 1 << from;
                    let to_mask = 1 << to;
                    let next_state = State {
                        position: position ^ from_mask ^ to_mask,
                        remaining: remaining ^ to_mask,
                    };

                    // Memoize previously seen states to eliminate suboptimal states right away.
                    let best = cache.entry(next_state).or_insert(u32::MAX);
                    if next_total < *best {
                        *best = next_total;
                        let next_heuristic = heuristic - minimum[to];
                        let next_guess = next_total + next_heuristic;
                        todo.push(next_guess, (next_state, next_heuristic));
                    }
                }
            }
        }
    }

    unreachable!()
}

// Convenience functions to find keys and robots.
fn is_key(b: u8) -> Option<usize> {
    b.is_ascii_lowercase().then(|| (b - b'a') as usize)
}

fn is_door(b: u8) -> Option<usize> {
    b.is_ascii_uppercase().then(|| (b - b'A') as usize)
}
