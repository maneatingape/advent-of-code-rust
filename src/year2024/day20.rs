//! # Race Condition
use crate::util::grid::*;
use crate::util::point::*;
use crate::util::thread::*;
use std::sync::atomic::{AtomicU32, Ordering};

pub fn parse(input: &str) -> Grid<i32> {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();

    let mut position = start;
    let mut direction = ORTHOGONAL.into_iter().find(|&o| grid[position + o] != b'#').unwrap();

    let mut time = Grid::new(grid.width + 19, grid.height + 19, i32::MAX);
    let mut elapsed = 0;

    while position != end {
        time[position] = elapsed;
        elapsed += 1;

        direction = [direction, direction.clockwise(), direction.counter_clockwise()]
            .into_iter()
            .find(|&d| grid[position + d] != b'#')
            .unwrap();
        position += direction;
    }

    time[end] = elapsed;
    time
}

pub fn part1(time: &Grid<i32>) -> u32 {
    let mut total = 0;

    for y in 1..time.height - 20 {
        for x in 1..time.width - 20 {
            let first = Point::new(x, y);

            if time[first] < i32::MAX {
                for second in [Point::new(2, 0), Point::new(0, 2)].map(|o| first + o) {
                    if time[second] < i32::MAX {
                        let saved = time[first].abs_diff(time[second]) - 2;

                        if saved >= 100 {
                            total += 1;
                        }
                    }
                }
            }
        }
    }

    total
}

pub fn part2(time: &Grid<i32>) -> u32 {
    let mut items = Vec::with_capacity(10_000);

    for y in 1..time.height - 20 {
        for x in 1..time.width - 20 {
            let point = Point::new(x, y);
            if time[point] < i32::MAX {
                items.push(point);
            }
        }
    }

    let total = AtomicU32::new(0);
    spawn_batches(items, |batch| worker(time, &total, batch));
    total.into_inner()
}

fn worker(time: &Grid<i32>, total: &AtomicU32, batch: Vec<Point>) {
    let mut cheats = 0;

    for first in batch {
        for y in -20..21_i32 {
            for x in (y.abs() - 20)..(21 - y.abs()) {
                let second = first + Point::new(x, y);

                if time.contains(second) && time[second] < i32::MAX {
                    let manhattan = x.abs() + y.abs();
                    let saved = time[second] - time[first] - manhattan;

                    if saved >= 100 {
                        cheats += 1;
                    }
                }
            }
        }
    }

    total.fetch_add(cheats, Ordering::Relaxed);
}
