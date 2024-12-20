//! # Race Condition
use crate::util::grid::*;
use crate::util::point::*;
use crate::util::thread::*;
use std::sync::atomic::{AtomicU32, Ordering};

pub fn parse(input: &str) -> Grid<i32> {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();

    let mut time = grid.same_size_with(i32::MAX);
    let mut elapsed = 0;

    let mut position = start;
    let mut direction = ORTHOGONAL.into_iter().find(|&o| grid[position + o] != b'#').unwrap();

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
    let mut cheats = 0;

    for y in 1..time.height - 1 {
        for x in 1..time.width - 1 {
            let point = Point::new(x, y);

            if time[point] != i32::MAX {
                cheats += check(time, point, Point::new(2, 0));
                cheats += check(time, point, Point::new(0, 2));
            }
        }
    }

    cheats
}

pub fn part2(time: &Grid<i32>) -> u32 {
    let mut items = Vec::with_capacity(10_000);

    for y in 1..time.height - 1 {
        for x in 1..time.width - 1 {
            let point = Point::new(x, y);

            if time[point] != i32::MAX {
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

    // (p1, p2) is the reciprocal of (p2, p1) so we only need to check each pair once. Checking the
    // wonky diamond shape on the right ensures complete coverage without duplicating checks.
    //      #        .
    //     ###      ...
    //    ##### => ..###
    //     ###      ###
    //      #        #
    for point in batch {
        for x in 2..21 {
            cheats += check(time, point, Point::new(x, 0));
        }

        for y in 1..21 {
            for x in (y - 20)..(21 - y) {
                cheats += check(time, point, Point::new(x, y));
            }
        }
    }

    total.fetch_add(cheats, Ordering::Relaxed);
}

#[inline]
fn check(time: &Grid<i32>, first: Point, delta: Point) -> u32 {
    let second = first + delta;

    (time.contains(second)
        && time[second] != i32::MAX
        && (time[first] - time[second]).abs() - first.manhattan(second) >= 100) as u32
}
