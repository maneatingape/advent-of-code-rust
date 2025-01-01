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
//!
//! For part two each path is independent so we can use multiple threads in parallel to speed
//! up the search.
use crate::util::grid::*;
use crate::util::point::*;
use crate::util::thread::*;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

type Pair = (Point, u32);

const UP: u32 = 0;
const DOWN: u32 = 1;
const LEFT: u32 = 2;
const RIGHT: u32 = 3;

pub struct Input {
    grid: Grid<u8>,
    up: Grid<i32>,
    down: Grid<i32>,
    left: Grid<i32>,
    right: Grid<i32>,
}

/// Atomics can be safely shared between threads.
struct Shared<'a> {
    input: &'a Input,
    tiles: AtomicUsize,
    mutex: Mutex<Vec<Pair>>,
}

/// Build four precomputed grids of the next coordinate in each direction for every point.
pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);

    let mut up: Grid<i32> = grid.same_size_with(0);
    let mut down: Grid<i32> = grid.same_size_with(0);
    let mut left: Grid<i32> = grid.same_size_with(0);
    let mut right: Grid<i32> = grid.same_size_with(0);

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
    // Build list of edge tiles and directions to process.
    let Input { grid, .. } = input;
    let mut todo = Vec::new();

    for x in 0..grid.width {
        todo.push((Point::new(x, 0), DOWN));
        todo.push((Point::new(x, grid.height - 1), UP));
    }

    for y in 0..grid.height {
        todo.push((Point::new(0, y), RIGHT));
        todo.push((Point::new(grid.width - 1, y), LEFT));
    }

    // Setup thread safe wrappers
    let shared = Shared { input, tiles: AtomicUsize::new(0), mutex: Mutex::new(todo) };

    // Use as many cores as possible to parallelize the search.
    spawn(|| worker(&shared));

    shared.tiles.load(Ordering::Relaxed)
}

/// Process starting locations from a shared queue.
fn worker(shared: &Shared<'_>) {
    loop {
        let start = {
            let mut exclusive = shared.mutex.lock().unwrap();
            let Some(start) = exclusive.pop() else {
                break;
            };
            start
        };
        shared.tiles.fetch_max(count(shared.input, start), Ordering::Relaxed);
    }
}

/// Count the number of energized tiles from a single starting location.
fn count(input: &Input, start: Pair) -> usize {
    let Input { grid, up, down, left, right } = input;

    let mut todo = VecDeque::with_capacity(1_000);
    let mut seen: Grid<u8> = grid.same_size_with(0);
    let mut energized: Grid<bool> = grid.same_size_with(false);

    todo.push_back(start);

    while let Some((position, direction)) = todo.pop_front() {
        let mut next = |direction: u32| {
            // Beams can loop, so check if we've already been here.
            let mask = 1 << direction;
            if seen[position] & mask != 0 {
                return;
            }
            seen[position] |= mask;

            match direction {
                UP => {
                    let x = position.x;
                    let last = up[position];

                    for y in last..position.y {
                        energized[Point::new(x, y + 1)] = true;
                    }
                    if last >= 0 {
                        todo.push_back((Point::new(x, last), UP));
                    }
                }
                DOWN => {
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
                    let y = position.y;
                    let last = left[position];

                    for x in last..position.x {
                        energized[Point::new(x + 1, y)] = true;
                    }
                    if last >= 0 {
                        todo.push_back((Point::new(last, y), LEFT));
                    }
                }
                RIGHT => {
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
            }
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
