//! # Many-Worlds Interpretation
//!
//! Our high-level approach is to simplify the problem into graph path finding. We only
//! ever need to move directly from key to key, so the maze becomes a graph where the nodes are
//! keys and the edge weight is the distance between keys. Doors modify which edges
//! are connected depending on the keys currently possessed.
//!
//! We first find the distance between every pair of keys then run
//! [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm) to find the
//! shortest path that visits every node in the graph.

//! The maze is also constructed in such a way to make our life easier:
//! * There is only ever one possible path to each key. We do not need to consider
//!   paths of different lengths that need different keys.
//! * As a corollary, if key `b` lies between `a` and `c` then `|ac| = |ab| + |bc|`. This
//!   enables a huge optimization that we only need to consider immediate neighbours.
//!   If we do not possess key `b` then it never makes sense to skip from `a` to `c` since `b` is
//!   along the way. We can model this by treating keys the same as doors. This optimization
//!   sped up my solution by a factor of 30.
//!
//! On top of this approach we apply some high-level tricks to go faster:
//! * We store previously seen pairs of `(location, keys collected)` to `total distance` in a map.
//!   If we are in the same location with the same keys but at a higher cost, then this situation
//!   can never be optimal so the solution can be discarded.
//! * When finding the distance between every pair of keys, it's faster to first only find the immediate
//!   neighbors of each key using a [Breadth first search](https://en.wikipedia.org/wiki/Breadth-first_search)
//!   then run the [Floyd-Warshall algorithm](https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm)
//!   to construct the rest of the graph. Even though the Floyd-Warshall asymptotic bound of `O(n³)`
//!   is higher than the asymptotic bounds of repeated BFS, this was twice as fast in practice
//!   for my input.
//!
//! We also apply some low-level tricks to go even faster:
//! * The set of remaining keys needed is stored as bits in an `u32`. We can have at most 26 keys
//!   so this will always fit. For example needing `a`, `b` and `e` is represented as `10011`.
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

/// `position` and `remaining` are both bitfields. For example a robot at key `d` that needs
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

/// `initial` is the complete set of keys that we need to collect. Will always be binary
/// `11111111111111111111111111` for the real input but fewer for sample data.
///
/// `maze` is the adjacency of distances and doors between each pair of keys and the robots
/// starting locations.
struct Maze {
    initial: State,
    maze: [[Door; 30]; 30],
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

    // Find the location of every key and robot in the maze.
    for (i, &b) in bytes.iter().enumerate() {
        if let Some(key) = is_key(b) {
            initial.remaining |= 1 << key;
            found.push((i, key));
        }
        if b == b'@' {
            initial.position |= 1 << robots;
            found.push((i, robots));
            robots += 1;
        }
    }

    // Start a BFS from each key and robot's location stopping at the nearest neighbor.
    // As a minor optimization we re-use the same `todo` and `seen` between each search.
    let default = Door { distance: u32::MAX, needed: 0 };

    let mut maze = [[default; 30]; 30];
    let mut seen = vec![usize::MAX; bytes.len()];
    let mut todo = VecDeque::new();

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
                maze[from][to] = Door { distance, needed };
                maze[to][from] = Door { distance, needed };
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
        maze[i][i].distance = 0;
    }

    for k in RANGE {
        for i in RANGE {
            for j in RANGE {
                let candidate = maze[i][k].distance.saturating_add(maze[k][j].distance);
                if maze[i][j].distance > candidate {
                    maze[i][j].distance = candidate;
                    // `(1 << k)` is a crucial optimization. By treating intermediate keys like
                    // doors we speed things up by a factor of 30.
                    maze[i][j].needed = maze[i][k].needed | (1 << k) | maze[k][j].needed;
                }
            }
        }
    }

    Maze { initial, maze }
}

fn explore(width: usize, bytes: &[u8]) -> u32 {
    let mut todo = MinHeap::with_capacity(5_000);
    let mut cache = FastMap::with_capacity(5_000);

    let Maze { initial, maze } = parse_maze(width, bytes);
    todo.push(0, initial);

    while let Some((total, State { position, remaining })) = todo.pop() {
        // Finish immediately if no keys left.
        // Since we're using Dijkstra this will always be the optimal solution.
        if remaining == 0 {
            return total;
        }

        // The set of robots is stored as bits in a `u32` shifted by the index of the location.
        for from in position.biterator() {
            // The set of keys still needed is also stored as bits in a `u32` similar as robots.
            for to in remaining.biterator() {
                let Door { distance, needed } = maze[from][to];

                // u32::MAX indicates that two nodes are not connected. Only possible in part two.
                if distance != u32::MAX && remaining & needed == 0 {
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
                        todo.push(next_total, next_state);
                    }
                }
            }
        }
    }

    unreachable!()
}

// Convenience functions to find keys and robots
fn is_key(b: u8) -> Option<usize> {
    b.is_ascii_lowercase().then(|| (b - b'a') as usize)
}

fn is_door(b: u8) -> Option<usize> {
    b.is_ascii_uppercase().then(|| (b - b'A') as usize)
}
