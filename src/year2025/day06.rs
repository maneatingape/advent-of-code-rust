//! # Trash Compactor
use crate::util::grid::*;
use crate::util::point::*;

type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let bottom = grid.height - 1;
    let mut right = grid.width;

    let block = |(part_one, part_two), left| {
        let rows = (0..bottom).map(|y| (left..right).fold(0, |num, x| acc(&grid, num, x, y)));
        let cols = (left..right).map(|x| (0..bottom).fold(0, |num, y| acc(&grid, num, x, y)));

        let plus = grid[Point::new(left, bottom)] == b'+';
        let first: u64 = if plus { rows.sum() } else { rows.product() };
        let second: u64 = if plus { cols.sum() } else { cols.product() };

        right = left - 1;
        (part_one + first, part_two + second)
    };

    (0..grid.width).rev().filter(|&x| grid[Point::new(x, bottom)] != b' ').fold((0, 0), block)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}

fn acc(grid: &Grid<u8>, number: u64, x: i32, y: i32) -> u64 {
    let digit = grid[Point::new(x, y)];
    if digit == b' ' { number } else { 10 * number + u64::from(digit - b'0') }
}
