//! # Garden Groups
//!
//! Solves both parts simultaneously by flood filling each region.
//!
//! For part one we increment the perimeter for each neighboring plot belonging to a different
//! region or out of bounds.
//!
//! For part two we count the end of each straight side in the clockwise direction.
//! A side ends when the next plot along it is outside the region (an outer corner), or
//! the diagonal plot across the edge is inside the region (an inner corner).
use crate::util::grid::*;
use crate::util::point::*;

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    // A newline border allows us to avoid boundary checks.
    let grid = Grid::parse_with_border(input);

    let mut todo = Vec::new();
    let mut seen = grid.same_size_with(false);

    let mut part_one = 0;
    let mut part_two = 0;

    // Iterate over every point, skipping the border.
    for y in 1..grid.height - 1 {
        for x in 1..grid.width {
            let start = Point::new(x, y);
            let same = |point| grid[point] == grid[start];

            // Skip already filled points.
            if seen[start] {
                continue;
            }

            // Flood fill.
            let mut area = 0;
            let mut perimeter = 0;
            let mut sides = 0;

            todo.push(start);
            seen[start] = true;

            while let Some(point) = todo.pop() {
                area += 1;

                for direction in ORTHOGONAL {
                    let next = point + direction;

                    if same(next) {
                        if !seen[next] {
                            todo.push(next);
                            seen[next] = true;
                        }
                    } else {
                        let clockwise = direction.clockwise();
                        perimeter += 1;
                        sides += usize::from(!same(point + clockwise) || same(next + clockwise));
                    }
                }
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
