//! # Ceres Search
//!
//! For part one we search each vertical, horizontal and diagonal line individually.
//! By using a `u32` bitmask of ASCII values we can check in both directions efficiently at the
//! same time.
//!
//! For part two the difference of the ASCII values of "M" and "S" is 6. No other combination of
//! letters has this value, so if both diagonals are a 6 then we have a match.
use crate::util::grid::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(grid: &Grid<u8>) -> u32 {
    let size = grid.width;
    let mut result = 0;

    // Horizontal and vertical
    for i in 0..size {
        result += scan_line(grid, Point::new(i, 0), DOWN, size);
        result += scan_line(grid, Point::new(0, i), RIGHT, size);
    }

    // Diagonals
    for i in 0..size - 3 {
        result += scan_line(grid, Point::new(i, 0), DOWN + RIGHT, size - i);
        result += scan_line(grid, Point::new(0, i + 1), DOWN + RIGHT, size - 1 - i);
        result += scan_line(grid, Point::new(size - 1 - i, 0), DOWN + LEFT, size - i);
        result += scan_line(grid, Point::new(size - 1, i + 1), DOWN + LEFT, size - 1 - i);
    }

    result
}

pub fn part2(grid: &Grid<u8>) -> u32 {
    let mut result = 0;

    for x in 1..grid.width - 1 {
        for y in 1..grid.height - 1 {
            let point = Point::new(x, y);

            if grid[point] == b'A' {
                let ul = grid[Point::new(x - 1, y - 1)];
                let ur = grid[Point::new(x + 1, y - 1)];
                let dl = grid[Point::new(x - 1, y + 1)];
                let dr = grid[Point::new(x + 1, y + 1)];
                // ASCII "M" is 77 and "S" is 53 so the absolute difference is 6.
                // No other combination of letters causes this difference.
                // "MS" on both diagonals is a match.
                result += (ul.abs_diff(dr) == 6 && ur.abs_diff(dl) == 6) as u32;
            }
        }
    }

    result
}

/// Searches a horizontal, vertical or diagonal line in both directions at once.
fn scan_line(grid: &Grid<u8>, mut point: Point, direction: Point, size: i32) -> u32 {
    let mut bytes = 0;
    let mut result = 0;

    for _ in 0..size {
        bytes = (bytes << 8) | (grid[point] as u32);
        point += direction;
        // "XMAS" and "SAMX" in hex.
        result += (bytes == 0x584d4153 || bytes == 0x53414d58) as u32;
    }

    result
}
