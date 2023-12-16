//! # The Floor Will Be Lava
//!
//! Brute force solution tracing the path of each beam, changing direction or splitting
//! according to the rules of each tile.
//!
//! To speed things up the next coordinate in each direction is precomputed for every point
//! so that the empty spaces between mirrros and splitters are filled efficiently.
//!
//! Some beams can enter a closed loop so we keep track of previously seen `(position, direction)`
//! pairs and stop if we've seen a pair before.
use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

type Pair = (Point, Point);

pub struct Input {
    grid: Grid<u8>,
    up: Grid<i32>,
    down: Grid<i32>,
    left: Grid<i32>,
    right: Grid<i32>,
}

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);

    let mut up: Grid<i32> = grid.default_copy();
    let mut down: Grid<i32> = grid.default_copy();
    let mut left: Grid<i32> = grid.default_copy();
    let mut right: Grid<i32> = grid.default_copy();

    for x in 0..grid.width {
        let mut last = -1;

        for y in 0..grid.height {
            let point = Point::new(x, y);
            up[point] = last;

            if matches!(grid[point], b'/' | b'\\' | b'-') {
                last = y;
            }
        }
    }

    for x in 0..grid.width {
        let mut last = grid.height;

        for y in (0..grid.height).rev() {
            let point = Point::new(x, y);
            down[point] = last;

            if matches!(grid[point], b'/' | b'\\' | b'-') {
                last = y;
            }
        }
    }

    for y in 0..grid.height {
        let mut last = -1;

        for x in 0..grid.width {
            let point = Point::new(x, y);
            left[point] = last;

            if matches!(grid[point], b'/' | b'\\' | b'|') {
                last = x;
            }
        }
    }

    for y in 0..grid.height {
        let mut last = grid.width;

        for x in (0..grid.width).rev() {
            let point = Point::new(x, y);
            right[point] = last;

            if matches!(grid[point], b'/' | b'\\' | b'|') {
                last = x;
            }
        }
    }

    Input { grid, up, down, left, right }
}

pub fn part1(input: &Input) -> usize {
    count(input, (ORIGIN, RIGHT))
}

pub fn part2(input: &Input) -> usize {
    let Input { grid, .. } = input;
    let mut result = 0;

    for x in 0..grid.width {
        result = result.max(count(input, (Point::new(x, 0), DOWN)));
        result = result.max(count(input, (Point::new(x, grid.height - 1), UP)));
    }

    for y in 0..grid.height {
        result = result.max(count(input, (Point::new(0, y), RIGHT)));
        result = result.max(count(input, (Point::new(grid.width - 1, y), LEFT)));
    }

    result
}

#[allow(clippy::too_many_lines)]
fn count(input: &Input, start: Pair) -> usize {
    let Input { grid, up, down, left, right } = input;

    let mut todo = VecDeque::with_capacity(10_000);
    let mut energized: Grid<bool> = grid.default_copy();
    let mut seen_up: Grid<bool> = grid.default_copy();
    let mut seen_down: Grid<bool> = grid.default_copy();
    let mut seen_left: Grid<bool> = grid.default_copy();
    let mut seen_right: Grid<bool> = grid.default_copy();

    todo.push_back(start);

    while let Some((position, direction)) = todo.pop_front() {
        let mut next = |direction: Point| match direction {
            UP => {
                if seen_up[position] {
                    return;
                }
                seen_up[position] = true;

                let x = position.x;
                let last = up[position];

                for y in last + 1..position.y + 1 {
                    energized[Point::new(x, y)] = true;
                }

                if last >= 0 {
                    todo.push_back((Point::new(x, last), UP));
                }
            }
            DOWN => {
                if seen_down[position] {
                    return;
                }
                seen_down[position] = true;

                let x = position.x;
                let last = down[position];

                for y in position.y..last {
                    energized[Point::new(x, y)] = true;
                }

                if last < grid.height {
                    todo.push_back((Point::new(x, last), DOWN));
                }
            }
            LEFT => {
                if seen_left[position] {
                    return;
                }
                seen_left[position] = true;

                let y = position.y;
                let last = left[position];

                for x in last + 1..position.x + 1 {
                    energized[Point::new(x, y)] = true;
                }

                if last >= 0 {
                    todo.push_back((Point::new(last, y), LEFT));
                }
            }
            RIGHT => {
                if seen_right[position] {
                    return;
                }
                seen_right[position] = true;

                let y = position.y;
                let last = right[position];

                for x in position.x..last {
                    energized[Point::new(x, y)] = true;
                }

                if last < grid.width {
                    todo.push_back((Point::new(last, y), RIGHT));
                }
            }
            _ => unreachable!(),
        };

        match grid[position] {
            b'.' => next(direction),
            b'/' => match direction {
                UP => next(RIGHT),
                DOWN => next(LEFT),
                LEFT => next(DOWN),
                RIGHT => next(UP),
                _ => unreachable!(),
            },
            b'\\' => match direction {
                UP => next(LEFT),
                DOWN => next(RIGHT),
                LEFT => next(UP),
                RIGHT => next(DOWN),
                _ => unreachable!(),
            },
            b'|' => match direction {
                UP | DOWN => next(direction),
                LEFT | RIGHT => {
                    next(UP);
                    next(DOWN);
                }
                _ => unreachable!(),
            },
            b'-' => match direction {
                LEFT | RIGHT => next(direction),
                UP | DOWN => {
                    next(LEFT);
                    next(RIGHT);
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    energized.bytes.iter().filter(|&&b| b).count()
}
