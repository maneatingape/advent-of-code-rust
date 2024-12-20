//! # Race Condition
use crate::util::grid::*;
use crate::util::point::*;
use crate::util::thread::*;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Input {
    grid: Grid<u8>,
    forward: Grid<i32>,
    reverse: Grid<i32>,
    full: i32,
}

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();

    let forward = bfs(&grid, start);
    let reverse = bfs(&grid, end);
    let full = forward[end];

    Input { grid, forward, reverse, full }
}

pub fn part1(input: &Input) -> u32 {
    let Input { grid, forward, reverse, full } = input;
    let mut total = 0;

    for y in 1..grid.height - 1 {
        for x in 1..grid.width - 1 {
            let first = Point::new(x, y);

            if grid[first] != b'#' {
                for second in ORTHOGONAL.map(|o| first + o * 2) {
                    if grid.contains(second) && grid[second] != b'#' {
                        let cost = forward[first] + reverse[second] + 2;

                        if *full - cost >= 100 {
                            total += 1;
                        }
                    }
                }
            }
        }
    }

    total
}

pub fn part2(input: &Input) -> u32 {
    let Input { grid, .. } = input;
    let mut items = Vec::with_capacity(10_000);

    for y in 1..grid.height - 1 {
        for x in 1..grid.width - 1 {
            let point = Point::new(x, y);
            if grid[point] != b'#' {
                items.push(point);
            }
        }
    }

    let total = AtomicU32::new(0);
    spawn_batches(items, |batch| worker(input, &total, batch));
    total.into_inner()
}

fn worker(input: &Input, total: &AtomicU32, batch: Vec<Point>) {
    let Input { grid, forward, reverse, full } = input;
    let mut cheats = 0;

    for first in batch {
        for y in -20..21_i32 {
            for x in (y.abs() - 20)..(21 - y.abs()) {
                let second = first + Point::new(x, y);

                if grid.contains(second) && grid[second] != b'#' {
                    let manhattan = x.abs() + y.abs();
                    let cost = forward[first] + reverse[second] + manhattan;

                    if *full - cost >= 100 {
                        cheats += 1;
                    }
                }
            }
        }
    }

    total.fetch_add(cheats, Ordering::Relaxed);
}

fn bfs(grid: &Grid<u8>, start: Point) -> Grid<i32> {
    let mut todo = VecDeque::new();
    let mut seen = grid.same_size_with(i32::MAX);

    todo.push_back((start, 0));
    seen[start] = 0;

    while let Some((position, cost)) = todo.pop_front() {
        let cost = cost + 1;

        for next in ORTHOGONAL.map(|o| position + o) {
            if grid[next] != b'#' && cost < seen[next] {
                todo.push_back((next, cost));
                seen[next] = cost;
            }
        }
    }

    seen
}
