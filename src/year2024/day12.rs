//! # Garden Groups
//!
//! Solves both parts simultaneously by flood filling each region.
//!
//! For part one we increment the perimeter for each neighboring plot belonging to a different
//! region or out of bounds.
//!
//! For part two we count each plot on the edge as either 0, 1 or 2 sides then divide by 2.
//! An edge plot contributes nothing if it has 2 edge neighbors facing the same way,
//! one if it has a single neighbor and two if it has no neighbors.
//!
//! For example, considering the right edge:
//!
//! ```none
//!     ...        ...        .#. > 1
//!     .#. > 2    .#. > 1    .#. > 0
//!     ...        .#. > 1    .#. > 1
//! ```
use crate::util::grid::*;
use crate::util::point::*;

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);

    let mut todo = Vec::new();
    let mut edge = Vec::new();
    let mut seen = grid.same_size_with(false);

    let mut part_one = 0;
    let mut part_two = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            // Skip already filled points.
            let point = Point::new(x, y);
            if seen[point] {
                continue;
            }

            // Flood fill, using area as an index.
            let kind = grid[point];
            let check = |point| grid.contains(point) && grid[point] == kind;

            let mut area = 0;
            let mut perimeter = 0;
            let mut sides = 0;

            todo.push(point);
            seen[point] = true;

            while area < todo.len() {
                let point = todo[area];
                area += 1;

                for direction in ORTHOGONAL {
                    let next = point + direction;

                    if check(next) {
                        if !seen[next] {
                            todo.push(next);
                            seen[next] = true;
                        }
                    } else {
                        edge.push((point, direction));
                        perimeter += 1;
                    }
                }
            }

            // Sum sides for all plots in the region.
            for &(p, d) in &edge {
                let r = d.clockwise();
                let l = d.counter_clockwise();

                sides += (!check(p + l) || check(p + l + d)) as usize;
                sides += (!check(p + r) || check(p + r + d)) as usize;
            }

            todo.clear();
            edge.clear();

            part_one += area * perimeter;
            part_two += area * (sides / 2);
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
