//! # Garden Groups
//!
//! Solves both parts simultaneously by flood filling each region.
//!
//! For part one we increment the perimeter for each neighbouring plot out of bounds
//! or a different kind of plant.
//!
//! For part two we count corners, as the number of corners equals the number of sides.
//! We remove a corner when another plot is adjacent either up, down, left or right.
//! For example, considering the top left corner of the plot:
//!
//! ```none
//!     ..      .#      ..      ##
//!     .# ✓    .# ✗    ## ✗    ## ✗
//! ```
//!
//! However we add back a corner when it's concave, for example where a plot is above, left but
//! not above and to the left:
//!
//! ```none
//!     .#
//!     ## ✓
//! ```
//!
//! There are 8 neighbours to check, giving 2⁸ possibilities. To speed things up these are
//! precomputed and cached in a lookup table.
use crate::util::grid::*;
use crate::util::point::*;
use std::array::from_fn;

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let lut = sides_lut();

    let mut region = Vec::new();
    let mut seen = grid.same_size_with(0);

    let mut part_one = 0;
    let mut part_two = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            // Skip already filled points.
            let point = Point::new(x, y);
            if seen[point] > 0 {
                continue;
            }

            // Assign a unique id to each region based on the first point that we encounter.
            let kind = grid[point];
            let id = y * grid.width + x + 1;

            // Flood fill, using area as an index.
            let mut area = 0;
            let mut perimeter = 0;
            let mut sides = 0;

            region.push(point);
            seen[point] = id;

            while area < region.len() {
                let point = region[area];
                area += 1;

                for next in ORTHOGONAL.map(|o| point + o) {
                    if grid.contains(next) && grid[next] == kind {
                        if seen[next] == 0 {
                            region.push(next);
                            seen[next] = id;
                        }
                    } else {
                        perimeter += 1;
                    }
                }
            }

            // Sum sides for all plots in the region.
            for point in region.drain(..) {
                let index = DIAGONAL.iter().fold(0, |acc, &d| {
                    (acc << 1) | (seen.contains(point + d) && seen[point + d] == id) as usize
                });
                sides += lut[index];
            }

            part_one += area * perimeter;
            part_two += area * sides;
        }
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

/// There are 8 neighbours to check, giving 2⁸ possibilities.
/// Precompute the number of corners once into a lookup table to speed things up.
fn sides_lut() -> [usize; 256] {
    from_fn(|neighbours| {
        let [up_left, up, up_right, left, right, down_left, down, down_right] =
            from_fn(|i| neighbours & (1 << i) > 0);

        let ul = !(up || left) || (up && left && !up_left);
        let ur = !(up || right) || (up && right && !up_right);
        let dl = !(down || left) || (down && left && !down_left);
        let dr = !(down || right) || (down && right && !down_right);

        (ul as usize) + (ur as usize) + (dl as usize) + (dr as usize)
    })
}
