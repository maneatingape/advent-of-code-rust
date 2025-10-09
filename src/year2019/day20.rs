//! # Donut Maze
//!
//! The approach to this solution is very similar to [`Day 18`] however parsing the maze
//! cleanly is quite tricky.
//!
//! We first simplify the problem by running a [breadth first search] from each portal
//! creating a list of distances between each pair of portals.
//!
//! Then a second BFS over this list efficiently solves both parts. For part two we use a cache to
//! memoize previously seen values. We optimize part two further by not recursing deeper than the
//! number of portals as this would mean a redundant trip to an already visited portal.
//!
//! [`Day 18`]: crate::year2019::day18
//! [breadth first search]: https://en.wikipedia.org/wiki/Breadth-first_search
use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;
use std::collections::VecDeque;

type Key = ((u8, u8), Kind);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Inner,
    Outer,
    Start,
    End,
}

enum Tile {
    Wall,
    Open,
    Portal(Key, Kind),
}

struct Edge {
    to: usize,
    kind: Kind,
    distance: u32,
}

pub struct Maze {
    start: usize,
    portals: Vec<Vec<Edge>>,
}

/// Parsing takes two passes. First we find the location of each portal. Then we BFS from each
/// portal to build a list of distance pairs.
pub fn parse(input: &str) -> Maze {
    let grid = Grid::parse(input);
    let width = grid.width as usize;

    let mut tiles: Vec<_> =
        grid.bytes.iter().map(|&b| if b == b'.' { Tile::Open } else { Tile::Wall }).collect();
    let mut map = FastMap::new();
    let mut found = Vec::new();
    let mut start = usize::MAX;

    // Find all labels
    for y in (1..grid.height - 1).step_by(2) {
        for x in (1..grid.width - 1).step_by(2) {
            let point = Point::new(x, y);
            if !grid[point].is_ascii_uppercase() {
                continue;
            }

            // Decode the relative orientation of the label and the portal
            let (first, second, third) = if grid[point + UP] == b'.' {
                (point, point + DOWN, point + UP)
            } else if grid[point + DOWN] == b'.' {
                (point + UP, point, point + DOWN)
            } else if grid[point + LEFT] == b'.' {
                (point, point + RIGHT, point + LEFT)
            } else if grid[point + RIGHT] == b'.' {
                (point + LEFT, point, point + RIGHT)
            } else {
                continue;
            };

            let pair = (grid[first], grid[second]);
            let index = (grid.width * third.y + third.x) as usize;
            let inner = 2 < x && x < grid.width - 3 && 2 < y && y < grid.height - 3;

            let (kind, opposite) = if inner {
                (Kind::Inner, Kind::Outer)
            } else {
                match pair {
                    (b'A', b'A') => {
                        start = found.len();
                        (Kind::Start, Kind::Start)
                    }
                    (b'Z', b'Z') => (Kind::End, Kind::End),
                    _ => (Kind::Outer, Kind::Inner),
                }
            };

            // `(pair, opposite)` is the key to the linked portal. Start and End map to themselves.
            tiles[index] = Tile::Portal((pair, opposite), kind);
            map.insert((pair, kind), found.len());
            found.push(index);
        }
    }

    // BFS from each portal. As a minor optimization we reuse `todo` and `visited`.
    let mut portals = Vec::new();
    let mut todo = VecDeque::new();
    let mut visited = vec![0; tiles.len()];

    for start in found {
        let mut edges = Vec::new();
        todo.push_back((start, 0));

        while let Some((index, steps)) = todo.pop_front() {
            visited[index] = start;

            for next_index in [index + 1, index - 1, index + width, index - width] {
                let next_steps = steps + 1;

                if visited[next_index] != start {
                    match tiles[next_index] {
                        Tile::Wall => (),
                        Tile::Open => {
                            todo.push_back((next_index, next_steps));
                        }
                        Tile::Portal(key, kind) => {
                            let to = map[&key];
                            edges.push(Edge { to, kind, distance: next_steps });
                        }
                    }
                }
            }
        }

        portals.push(edges);
    }

    Maze { start, portals }
}

/// Straight BFS with no caching or any optimization tricks.
pub fn part1(input: &Maze) -> u32 {
    let mut todo = VecDeque::new();
    todo.push_back((0, input.start));

    while let Some((steps, index)) = todo.pop_front() {
        for &Edge { to, kind, distance } in &input.portals[index] {
            let next_steps = steps + distance + 1;

            match kind {
                Kind::Inner | Kind::Outer => todo.push_back((next_steps, to)),
                Kind::End => return next_steps - 1,
                Kind::Start => (),
            }
        }
    }

    unreachable!()
}

/// BFS with memoization of previously seen states.
pub fn part2(input: &Maze) -> u32 {
    let mut cache = FastMap::with_capacity(2_000);
    let mut todo = VecDeque::new();
    todo.push_back((0, input.start, 0));

    while let Some((steps, index, level)) = todo.pop_front() {
        let key = (index, level);
        if let Some(min) = cache.get(&key)
            && *min <= steps
        {
            continue;
        }
        cache.insert(key, steps);

        for &Edge { to, kind, distance } in &input.portals[index] {
            let next_steps = steps + distance + 1;

            match kind {
                // No need to recurse further than the number of portals
                Kind::Inner if level < input.portals.len() => {
                    todo.push_back((next_steps, to, level + 1));
                }
                Kind::Outer if level > 0 => {
                    todo.push_back((next_steps, to, level - 1));
                }
                Kind::End if level == 0 => {
                    return next_steps - 1;
                }
                _ => (),
            }
        }
    }

    unreachable!()
}
