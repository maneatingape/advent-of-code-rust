//! # Cosmic Expansion
//!
//! Parses each galaxy into a two dimensional [`Point`].
//! Then calculates a [prefix sum](https://en.wikipedia.org/wiki/Prefix_sum) of the number of
//! horizontal and vertical gaps to left or above each coordinate. Using the sample data:
//!
//! ```none
//!     Horizontal: [0, 0, 0, 1, 1, 1, 1, 2, 2, 2]
//!     Vertical:   [0, 0, 1, 1, 1, 2, 2, 2, 3, 3]
//! ```
//!
//! Then to stretch each each point we add the number of gaps. For example:
//!
//! * Original galaxy `(6, 4)`
//! * `vertical[6] = 2`
//! * `horizontal[4] = 1`
//! * Stretched `(8, 5)`
//!
//! For part two we scale the gaps by a factor.
use crate::util::grid::*;
use crate::util::point::*;

pub struct Input {
    points: Vec<Point>,
    horizontal: Vec<i32>,
    vertical: Vec<i32>,
}

pub fn parse(input: &str) -> Input {
    let grid: Grid<u8> = Grid::parse(input);
    let size = grid.width as usize;

    // Build points keeping track of empty rows and columns.
    let mut points = Vec::new();
    let mut rows = vec![true; size];
    let mut columns = vec![true; size];

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if grid[point] == b'#' {
                points.push(point);
                rows[y as usize] = false;
                columns[x as usize] = false;
            }
        }
    }

    // Calculate prefix sum of horizontal and vertical gaps.
    let mut h = 0;
    let mut v = 0;
    let mut horizontal = vec![0; size];
    let mut vertical = vec![0; size];

    for i in 0..size {
        h += rows[i] as i32;
        v += columns[i] as i32;
        horizontal[i] = h;
        vertical[i] = v;
    }

    Input { points, horizontal, vertical }
}

pub fn part1(input: &Input) -> u64 {
    stretch(input, 1)
}

pub fn part2(input: &Input) -> u64 {
    stretch(input, 999999)
}

fn stretch(input: &Input, factor: i32) -> u64 {
    let mut result = 0;
    let points: Vec<_> = input
        .points
        .iter()
        .map(|p| {
            // Stretch the distance between points based on how many
            // empty row or columns are to the left or above.
            let x = p.x + factor * input.vertical[p.x as usize];
            let y = p.y + factor * input.horizontal[p.y as usize];
            Point::new(x, y)
        })
        .collect();

    // Sum the Manhattan distance between all pairs of points.
    for (i, p1) in points.iter().enumerate().skip(1) {
        result += points.iter().take(i).map(|&p2| p1.manhattan(p2) as u64).sum::<u64>();
    }

    result
}
