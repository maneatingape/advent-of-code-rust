//! # Race Condition
//!
//! Examining the input shows that there is only a single path from start to finish with
//! no branches. This simplifies checking for shortcuts as any empty space will be on the shortest
//! path from start to end. The cheating rules allow us to "warp" up to `n` squares away to any
//! empty space as measured by [manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry).
//!
//! For part one this is a distance of 2. When checking surrounding squares we make 2 optimizations:
//!
//! * Don't check any squares only 1 away as we can always just move to these normally.
//! * Checking from point `p => q` is always the negative of `q => p`, e.g if `p = 30, q = 50` then
//!   `p => q = 20` and `q => p = -20`. This means we only ever need to check any pair once.
//! * Additionally we only need to check down and right. Previous rows and columns will already
//!   have checked points above and to the left when we reach an empty square.
//!
//! ```none
//!       #         .
//!      ###       ...
//!     ##P## =>  ....#
//!      ###       ...
//!       #         #
//! ```
//!
//! For part two the distance is increased to 20 and the shape now resembles a wonky diamond.
//! This shape ensures complete coverage without duplicating checks.
//!
//! ```none
//!      #        .
//!     ###      ...
//!    ##P## => ..P##
//!     ###      ###
//!      #        #
//! ```
use crate::util::grid::*;
use crate::util::point::*;
use crate::util::thread::*;
use std::sync::atomic::{AtomicU32, Ordering};

/// Create a grid the same size as input with the time taken from start to any location.
pub fn parse(input: &str) -> Grid<i32> {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();

    let mut time = grid.same_size_with(i32::MAX);
    let mut elapsed = 0;

    // Find starting direction, assuming start position is surrounded by 3 walls.
    let mut position = start;
    let mut direction = ORTHOGONAL.into_iter().find(|&o| grid[position + o] != b'#').unwrap();

    while position != end {
        time[position] = elapsed;
        elapsed += 1;

        // There are no branches so we only ever need to go straight ahead or turn left or right.
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

            // We only need to check 2 points to the right and down as previous empty squares
            // have already checked up and to the left.
            if time[point] != i32::MAX {
                cheats += check(time, point, Point::new(2, 0));
                cheats += check(time, point, Point::new(0, 2));
            }
        }
    }

    cheats
}

/// Searches for all cheats up to distance 20, parallelizing the work over multiple threads.
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

    // Use as many cores as possible to parallelize the remaining search.
    let total = AtomicU32::new(0);
    spawn_parallel_iterator(&items, |iter| worker(time, &total, iter));
    total.into_inner()
}

fn worker(time: &Grid<i32>, total: &AtomicU32, iter: ParIter<'_, Point>) {
    let mut cheats = 0;

    // (p1, p2) is the reciprocal of (p2, p1) so we only need to check each pair once.
    for &point in iter {
        for x in 2..21 {
            cheats += check(time, point, Point::new(x, 0));
        }

        for y in 1..21 {
            for x in (y - 20)..(21 - y) {
                cheats += check(time, point, Point::new(x, y));
            }
        }
    }

    // Update global total.
    total.fetch_add(cheats, Ordering::Relaxed);
}

// Check if we save enough time warping to another square.
#[inline]
fn check(time: &Grid<i32>, first: Point, delta: Point) -> u32 {
    let second = first + delta;

    (time.contains(second)
        && time[second] != i32::MAX
        && (time[first] - time[second]).abs() - first.manhattan(second) >= 100) as u32
}
