//! # The Floor Will Be Lava
//!
//! Brute force solution tracing the path of each beam, changing direction or splitting
//! according to the rules of each tile.
//!
//! Some beams can enter a closed loop so we keep track of previously seen `(position, direction)`
//! pairs and stop if we've seen a pair before.
use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;
use std::collections::VecDeque;

type Pair = (Point, Point);

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(grid: &Grid<u8>) -> usize {
    let todo = &mut VecDeque::with_capacity(10_000);
    let seen = &mut FastSet::with_capacity(20_000);
    count(grid, todo, seen, (ORIGIN, RIGHT))
}

pub fn part2(grid: &Grid<u8>) -> usize {
    let mut result = 0;
    let todo = &mut VecDeque::with_capacity(10_000);
    let seen = &mut FastSet::with_capacity(20_000);

    let mut check = |start: Pair| {
        result = result.max(count(grid, todo, seen, start));
        todo.clear();
        seen.clear();
    };

    for x in 0..grid.width {
        check((Point::new(x, 0), DOWN));
        check((Point::new(x, grid.height - 1), UP));
    }

    for y in 0..grid.height {
        check((Point::new(0, y), RIGHT));
        check((Point::new(grid.width - 1, y), LEFT));
    }

    result
}

fn count(
    grid: &Grid<u8>,
    todo: &mut VecDeque<Pair>,
    seen: &mut FastSet<Pair>,
    start: Pair,
) -> usize {
    let mut energized: Grid<bool> = grid.default_copy();

    todo.push_back(start);

    while let Some((position, direction)) = todo.pop_front() {
        if !grid.contains(position) || !seen.insert((position, direction)) {
            continue;
        }

        energized[position] = true;

        match grid[position] {
            b'.' => todo.push_back((position + direction, direction)),
            b'/' => match direction {
                UP => todo.push_back((position + RIGHT, RIGHT)),
                DOWN => todo.push_back((position + LEFT, LEFT)),
                LEFT => todo.push_back((position + DOWN, DOWN)),
                RIGHT => todo.push_back((position + UP, UP)),
                _ => unreachable!(),
            },
            b'\\' => match direction {
                UP => todo.push_back((position + LEFT, LEFT)),
                DOWN => todo.push_back((position + RIGHT, RIGHT)),
                LEFT => todo.push_back((position + UP, UP)),
                RIGHT => todo.push_back((position + DOWN, DOWN)),
                _ => unreachable!(),
            },
            b'|' => match direction {
                UP | DOWN => todo.push_back((position + direction, direction)),
                LEFT | RIGHT => {
                    todo.push_back((position + UP, UP));
                    todo.push_back((position + DOWN, DOWN));
                }
                _ => unreachable!(),
            },
            b'-' => match direction {
                LEFT | RIGHT => todo.push_back((position + direction, direction)),
                UP | DOWN => {
                    todo.push_back((position + LEFT, LEFT));
                    todo.push_back((position + RIGHT, RIGHT));
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    energized.bytes.iter().filter(|&&b| b).count()
}
