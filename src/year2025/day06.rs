//! # Trash Compactor
//!
//! Processing the input from right to left means that we can use the operator on the bottom row
//! to split blocks of numbers and don't need special-case handling for the end of the input.
//! Blocks are the same height but can be different widths.
//!
//! Both parts are computed together. Each block is converted into a set of numbers twice,
//! in rows from top to bottom and columns from left to right.
//! Leading and trailing spaces are ignored.
//!
//! For performance, we avoid storing the numbers in an intermediate `vec` and just use the
//! iterators directly.
use crate::util::grid::*;
use crate::util::point::*;

type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let bottom = grid.height - 1;
    let mut right = grid.width;
    let mut part_one = 0;
    let mut part_two = 0;

    // Use operator on bottom row to delimit block boundaries.
    for left in (0..grid.width).rev().filter(|&x| grid[Point::new(x, bottom)] != b' ') {
        let rows = (0..bottom).map(|y| (left..right).fold(0, |num, x| acc(&grid, num, x, y)));
        let cols = (left..right).map(|x| (0..bottom).fold(0, |num, y| acc(&grid, num, x, y)));

        // Use iterators directly.
        let plus = grid[Point::new(left, bottom)] == b'+';
        let first: u64 = if plus { rows.sum() } else { rows.product() };
        let second: u64 = if plus { cols.sum() } else { cols.product() };

        right = left - 1;
        part_one += first;
        part_two += second;
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}

/// Ignore spaces when parsing a number.
fn acc(grid: &Grid<u8>, number: u64, x: i32, y: i32) -> u64 {
    let digit = grid[Point::new(x, y)];
    if digit == b' ' { number } else { 10 * number + u64::from(digit - b'0') }
}
