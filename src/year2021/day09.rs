//! # Smoke Basin
//!
//! Part 2 is the classic [flood fill](https://en.wikipedia.org/wiki/Flood_fill) algorithm with a
//! twist to return the size of the filled area. This algorithm can be implemented either as a
//! [DFS](https://en.wikipedia.org/wiki/Depth-first_search) using recursion or as a
//! [BFS](https://en.wikipedia.org/wiki/Breadth-first_search) using an auxilary data structure
//! such as a [`VecDeque`].
//!
//! This solution uses a DFS approach as it's faster and Rust's stack size limit seems enough
//! to accommodate the maximum basin size. 2 dimensional grids are common in AoC problems so we
//! use our utility [`Grid`] and [`Point`] modules.
//!
//! [`VecDeque`]: std::collections::VecDeque
//! [`Grid`]: crate::util::grid
//! [`Point`]: crate::util::point
use crate::util::grid::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(grid: &Grid<u8>) -> u32 {
    let mut risk_levels = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            let point = Point { x, y };
            let cur = grid[point];
            let low_point = ORTHOGONAL
                .iter()
                .map(|&n| point + n)
                .filter(|&n| grid.contains(n))
                .all(|n| grid[n] > cur);

            if low_point {
                risk_levels += 1 + (cur - b'0') as u32;
            }
        }
    }

    risk_levels
}

pub fn part2(grid: &Grid<u8>) -> u32 {
    let mut visited = grid.default_copy::<bool>();
    let mut basins = Vec::new();

    for x in 0..grid.width {
        for y in 0..grid.height {
            let next = Point { x, y };
            if grid[next] < b'9' && !visited[next] {
                basins.push(flood_fill(grid, &mut visited, next));
            }
        }
    }

    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn flood_fill(grid: &Grid<u8>, visited: &mut Grid<bool>, point: Point) -> u32 {
    visited[point] = true;
    let mut size = 1;

    for next in ORTHOGONAL.iter().map(|&n| point + n) {
        if grid.contains(next) && grid[next] < b'9' && !visited[next] {
            size += flood_fill(grid, visited, next)
        }
    }

    size
}
