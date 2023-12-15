//! # Parabolic Reflector Dish
//!
//! To solve part two we look for a cycle where the dish returns to a previously seen state.
//! By storing each dish and a index in a `HashMap` we can calculate the offset and length of the
//! cycle then use that to find to state at the billionth step.
use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(input: &Grid<u8>) -> i32 {
    let grid = &mut input.clone();
    north(grid);
    load(grid)
}

pub fn part2(input: &Grid<u8>) -> i32 {
    let grid = &mut input.clone();

    let mut seen = FastMap::with_capacity(100);
    seen.insert(grid.clone(), 0);

    // Find cycle
    let (start, end) = loop {
        north(grid);
        west(grid);
        south(grid);
        east(grid);

        if let Some(previous) = seen.insert(grid.clone(), seen.len()) {
            break (previous, seen.len());
        }
    };

    let offset = 1_000_000_000 - start;
    let cycle_width = end - start;
    let remainder = offset % cycle_width;
    let target = start + remainder;

    let (grid, _) = seen.iter().find(|(_, &i)| i == target).unwrap();
    load(grid)
}

fn north(grid: &mut Grid<u8>) {
    for x in 0..grid.width {
        let mut fixed = 0;

        for y in 0..grid.height {
            let point = Point::new(x, y);
            match grid[point] {
                b'O' => {
                    if y > fixed {
                        grid[point] = b'.';
                        grid[Point::new(x, fixed)] = b'O';
                    }
                    fixed += 1;
                }
                b'#' => fixed = y + 1,
                _ => (),
            }
        }
    }
}

fn west(grid: &mut Grid<u8>) {
    for y in 0..grid.height {
        let mut fixed = 0;

        for x in 0..grid.width {
            let point = Point::new(x, y);
            match grid[point] {
                b'O' => {
                    if x > fixed {
                        grid[point] = b'.';
                        grid[Point::new(fixed, y)] = b'O';
                    }
                    fixed += 1;
                }
                b'#' => fixed = x + 1,
                _ => (),
            }
        }
    }
}

fn south(grid: &mut Grid<u8>) {
    for x in 0..grid.width {
        let mut fixed = grid.height - 1;

        for y in (0..grid.height).rev() {
            let point = Point::new(x, y);
            match grid[point] {
                b'O' => {
                    if y < fixed {
                        grid[point] = b'.';
                        grid[Point::new(x, fixed)] = b'O';
                    }
                    fixed -= 1;
                }
                b'#' => fixed = y - 1,
                _ => (),
            }
        }
    }
}

fn east(grid: &mut Grid<u8>) {
    for y in 0..grid.height {
        let mut fixed = grid.width - 1;

        for x in (0..grid.width).rev() {
            let point = Point::new(x, y);
            match grid[point] {
                b'O' => {
                    if x < fixed {
                        grid[point] = b'.';
                        grid[Point::new(fixed, y)] = b'O';
                    }
                    fixed -= 1;
                }
                b'#' => fixed = x - 1,
                _ => (),
            }
        }
    }
}

fn load(grid: &Grid<u8>) -> i32 {
    let mut load = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid[Point::new(x, y)] == b'O' {
                load += grid.height - y;
            }
        }
    }

    load
}
