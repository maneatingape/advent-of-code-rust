//! # Printing Department
use crate::util::grid::*;
use crate::util::point::*;

type Input = (Vec<Point>, Grid<u8>);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let offset = Point::new(1, 1);
    let mut todo = Vec::new();
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
