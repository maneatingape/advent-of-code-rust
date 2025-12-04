//! # Printing Department
use crate::util::grid::*;
use crate::util::point::*;

type Input = (usize, usize);

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

    let part_one = todo.len();
    let mut part_two = 0;

    while let Some(point) = todo.pop() {
        part_two += 1;

        for next in DIAGONAL.map(|d| point + d) {
            if padded[next] == 4 {
                todo.push(next);
            }
            padded[next] -= 1;
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
