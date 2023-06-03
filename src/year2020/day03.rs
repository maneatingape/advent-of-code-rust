//! # Toboggan Trajectory
//!
//! Two dimensional grids of ASCII characters are a common AoC theme, so we use our utility
//! [`Grid`] class to parse the data.
//!
//! [`Grid`]: crate::util::grid
use crate::util::grid::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(input: &Grid<u8>) -> u64 {
    toboggan(input, 3, 1)
}

pub fn part2(input: &Grid<u8>) -> u64 {
    toboggan(input, 1, 1)
        * toboggan(input, 3, 1)
        * toboggan(input, 5, 1)
        * toboggan(input, 7, 1)
        * toboggan(input, 1, 2)
}

fn toboggan(grid: &Grid<u8>, dx: i32, dy: i32) -> u64 {
    let mut point = ORIGIN;
    let mut trees = 0;

    while point.y < grid.height {
        if grid[point] == b'#' {
            trees += 1
        }
        point.x = (point.x + dx) % grid.width;
        point.y += dy;
    }

    trees
}
