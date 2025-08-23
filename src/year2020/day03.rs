//! # Toboggan Trajectory
//!
//! Two dimensional grids of ASCII characters are a common Advent of Code theme,
//! so we use our utility [`Grid`] class to parse the data.
//!
//! [`Grid`]: crate::util::grid
use crate::util::grid::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(input: &Grid<u8>) -> usize {
    toboggan(input, 3, 1)
}

pub fn part2(input: &Grid<u8>) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(dx, dy)| toboggan(input, dx, dy))
        .product()
}

fn toboggan(grid: &Grid<u8>, dx: i32, dy: i32) -> usize {
    (0..grid.height / dy)
        .filter(|&i| {
            let point = Point::new((i * dx) % grid.width, i * dy);
            grid[point] == b'#'
        })
        .count()
}
