//! # Printing Department
//!
//! To speed things up, we build a queue of rolls to be removed. For part one, the length of the
//! initial list is the answer. For part two, as we remove rolls from the list one at a time, we
//! update neighboring rolls. If any neighbor drops below the threshold, then we add it to the
//! list. This approach avoids a time-consuming scan of the entire grid to find new rolls to remove.
//!
//! Either [breadth-first search](https://en.wikipedia.org/wiki/Breadth-first_search) using a
//! `VecDeque` or [depth-first search](https://en.wikipedia.org/wiki/Depth-first_search) using
//! a `Vec` will work. The depth-first search is faster, so we choose that.
use crate::util::grid::*;
use crate::util::point::*;

type Input = (Vec<Point>, Grid<u8>);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let offset = Point::new(1, 1);
    let mut todo = Vec::new();
    // Build a grid with an empty edge to avoid boundary checks.
    let mut padded = Grid::new(grid.width + 2, grid.height + 2, u8::MAX);

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);

            if grid[point] == b'@' {
                let count = DIAGONAL
                    .iter()
                    .map(|&d| point + d)
                    .filter(|&next| grid.contains(next) && grid[next] == b'@')
                    .count();

                // Add rolls that can be removed to the initial list.
                if count < 4 {
                    todo.push(point + offset);
                }
                padded[point + offset] = count as u8;
            }
        }
    }

    (todo, padded)
}

pub fn part1(input: &Input) -> usize {
    let (todo, _) = input;
    todo.len()
}

pub fn part2(input: &Input) -> usize {
    let (mut todo, mut padded) = input.clone();
    let mut removed = 0;

    // Update neighbors as rolls are removed. If they drop below the threshold, then add to the list.
    while let Some(point) = todo.pop() {
        removed += 1;

        for next in DIAGONAL.map(|d| point + d) {
            if padded[next] == 4 {
                todo.push(next);
            }
            padded[next] -= 1;
        }
    }

    removed
}
