//! # Hill Climbing Algorithm
//!
//! Pretty much textbook implementation of a BFS (Breadth-first search). If you're not familiar with
//! BFS, [this blog post is a great introduction](https://www.redblobgames.com/pathfinding/a-star/introduction.html)
//! to the algorithm, plus some others that come in handy for Advent of Code.
//!
//! Implementation notes:
//! * A [`VecDeque`] of [`Point`] is used to store the frontier as it gives better performance than
//!   [`vec`] when used as a FIFO queue.
//! * [`Grid`] is used to store both the height information and seen nodes.
//!
//! For part two we could search for all `a` locations and repeatedly start a BFS search from there,
//! then find the lowest value. However, a much faster approach is to search *backwards* from the
//! end location. Due to the fact that BFS always explores closest nodes first this will find the
//! closest `a` location in a single search. In fact, we can just run one single search, finding
//! the part two answer first, then continuing on to the `S` location for part one.
//!
//! [`Grid`]: crate::util::grid
//! [`Point`]: crate::util::point
use std::collections::VecDeque;

use crate::util::grid::*;
use crate::util::point::*;

type Input = (u32, u32);

/// Uses the utility [`Grid`] module to parse a 2D array of ASCII characters.
///
/// [`Grid`]: crate::util::grid
pub fn parse(input: &str) -> Input {
    let mut grid = Grid::parse(input);

    // Run the BFS algorithm implementation with the reversed height transition rules baked in.
    // In fact, we don't need a separate seen grid; we can modify the original grid in place.
    let start = grid.find(b'E').unwrap();
    let mut todo = VecDeque::from([(start, b'z' - 1, 1)]);
    grid[start] = 0;
    let mut part_two = None;

    while let Some((point, height, cost)) = todo.pop_front() {
        for next in ORTHOGONAL.map(|d| d + point) {
            if !grid.contains(next) {
                continue;
            }
            if grid[next] == b'S' {
                return (cost, part_two.unwrap());
            }
            if grid[next] >= height {
                if grid[next] == b'a' {
                    part_two = part_two.or(Some(cost));
                }
                todo.push_back((next, grid[next] - 1, cost + 1));
                grid[next] = 0;
            }
        }
    }

    unreachable!()
}

/// Find the shortest path from `E` to `S`.
pub fn part1(input: &Input) -> u32 {
    input.0
}

/// Find the shortest path from `E` to closest `a`.
pub fn part2(input: &Input) -> u32 {
    input.1
}
