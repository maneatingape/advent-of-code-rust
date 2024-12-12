//! # Garden Groups
//!
//! Part one flood fills each region, adding 4 to the perimeter for each plot
//! then subtracting 2 for each neighbour that we've already added.
//!
//! Part two counts corners, as the number of corners equals the number of sides.
//! We remove a corner when another plot is adjacent either up, down, left or right:
//!
//! ```none
//!     .#.    ...
//!     .#.    ##.
//!     ...    ...
//! ```
//!
//! We add back a corner when it's concave, for example where a plot is above, right but
//! not above and to the right:
//!
//! ```none
//!     .#.
//!     .##
//!     ...
//! ```
//!
//! There are 8 neighbours to check, giving 2⁸ possibilities. These are precomputed and cached
//! in a lookup table.
use crate::util::grid::*;
use crate::util::point::*;
use std::array::from_fn;
use std::collections::VecDeque;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(grid: &Grid<u8>) -> i32 {
    let mut todo = VecDeque::new();
    let mut seen = grid.same_size_with(false);
    let mut result = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if seen[point] {
                continue;
            }

            // Flood fill each region.
            let kind = grid[point];
            let mut area = 0;
            let mut perimeter = 0;

            todo.push_back(point);
            seen[point] = true;

            while let Some(point) = todo.pop_front() {
                area += 1;

                for next in ORTHOGONAL.map(|o| point + o) {
                    if grid.contains(next) && grid[next] == kind {
                        if !seen[next] {
                            seen[next] = true;
                            todo.push_back(next);
                        }
                    } else {
                        perimeter += 1;
                    }
                }
            }

            result += area * perimeter;
        }
    }

    result
}

pub fn part2(grid: &Grid<u8>) -> usize {
    // Lookup table that returns number of corners for all combinations of neighbours.
    let lut = sides_lut();

    let mut result = 0;
    let mut todo = VecDeque::new();
    let mut seen = grid.same_size_with(-1);
    let mut region = Vec::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if seen[point] != -1 {
                continue;
            }

            let kind = grid[point];
            let id = y * grid.width + x;

            todo.push_back(point);
            seen[point] = id;

            while let Some(point) = todo.pop_front() {
                region.push(point);

                for next in ORTHOGONAL.map(|o| point + o) {
                    if grid.contains(next) && grid[next] == kind && seen[next] == -1 {
                        seen[next] = id;
                        todo.push_back(next);
                    }
                }
            }

            let size = region.len();

            for point in region.drain(..) {
                let index = DIAGONAL.iter().fold(0, |acc, &d| {
                    (acc << 1) | (seen.contains(point + d) && seen[point + d] == id) as usize
                });
                result += size * lut[index];
            }
        }
    }

    result
}

/// There are 8 neighbours to check, giving 2⁸ possibilities. Precompute the number of corners
/// once into a lookup table to speed things up.
fn sides_lut() -> [usize; 256] {
    from_fn(|neighbours| {
        let [up_left, up, up_right, left, right, down_left, down, down_right] =
            from_fn(|i| neighbours & (1 << i) != 0);
        let mut sides = 0;

        if !(up || left) || (up && left && !up_left) {
            sides += 1;
        }
        if !(up || right) || (up && right && !up_right) {
            sides += 1;
        }
        if !(down || left) || (down && left && !down_left) {
            sides += 1;
        }
        if !(down || right) || (down && right && !down_right) {
            sides += 1;
        }

        sides
    })
}
