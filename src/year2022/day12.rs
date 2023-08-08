//! # Hill Climbing Algorithm
//!
//! Pretty much textbook implementation of a BFS (Breadth First Search). If you're not familar with
//! BFS, [this blog post is a great introduction](https://www.redblobgames.com/pathfinding/a-star/introduction.html)
//! to the algorithm, plus some others that come in handy for Advent of Code.
//!
//! Implementation notes:
//! * A [`VecDeque`] of [`Point`] is used to store the frontier as it gives better performance
//!   than [`vec`] when used as a FIFO queue.
//! * [`Grid`] is used to store both the height information and visited nodes.
//!
//! For Part 2 we could search for all `a` locations and repeatedly start a BFS search from there,
//! then find the lowest value. However a much faster approach is to search *backwards* from the
//! end location. Due the the fact that BFS always explores closest nodes first this will find the
//! closest `a` location in a single search. For part 1 it will have the same result, so we
//! can re-use the same code.
//!
//! [`Grid`]: crate::util::grid
//! [`Point`]: crate::util::point
use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

type Input = (Grid<u8>, Point);

/// Uses the utility [`Grid`] class to parse a 2D array of ASCII characters.
///
/// [`Grid`]: crate::util::grid
pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find(b'E');
    (grid, start.unwrap())
}

/// Find the shortest path from `E` to `S`
pub fn part1(input: &Input) -> u32 {
    bfs(input, b'S')
}

/// Find the shortest path from `E` to closest `a`
pub fn part2(input: &Input) -> u32 {
    bfs(input, b'a')
}

/// BFS algorithm implementation with the reversed height transition rules baked in.
fn bfs(input: &Input, end: u8) -> u32 {
    let (grid, start) = input;
    let mut todo = VecDeque::from([(*start, 0)]);
    let mut visited = grid.default_copy::<bool>();

    while let Some((point, cost)) = todo.pop_front() {
        if grid[point] == end {
            return cost;
        }
        for next in ORTHOGONAL.iter().map(|&x| x + point) {
            if grid.contains(next)
                && !visited[next]
                && height(grid, point) - height(grid, next) <= 1
            {
                todo.push_back((next, cost + 1));
                visited[next] = true;
            }
        }
    }

    unreachable!()
}

/// Map `S` to `a` and `E` to `z`, otherwise use the value unchanged.
fn height(grid: &Grid<u8>, point: Point) -> i32 {
    match grid[point] {
        b'S' => 'a' as i32,
        b'E' => 'z' as i32,
        b => b as i32,
    }
}
