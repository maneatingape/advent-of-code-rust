//! # Guard Gallivant
//!
//! Part two is sped up by pre-computing the next obstacle in each direction from any point in
//! the grid. If there is nothing left in the way then coordinates outside the grid are used.
//! One dimensional example:
//!
//! ```none
//!     .#...
//!     Left: (-1, 2, 2, 2, 2)
//!     Right: (1, 1, 5, 5, 5)
//! ```
//!
//! This allows us to "shortcut" to each obstacle when looking for cycles. The remaining tricky
//! part is including the extra obstacle which is different for each point on the guard's path.
//!
//! The search can be parallelized across multiple threads as each position is independent.
use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;
use crate::util::thread::*;
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

/// Count distinct positions in the guard's path, which will eventually leave the grid.
pub fn part1(grid: &Grid<u8>) -> usize {
    let mut grid = grid.clone();
    let mut position = grid.find(b'^').unwrap();
    let mut direction = UP;
    let mut result = 1;

    while grid.contains(position + direction) {
        if grid[position + direction] == b'#' {
            direction = direction.clockwise();
            continue;
        }

        let next = position + direction;

        // Avoid double counting when the path crosses itself.
        if grid[next] == b'.' {
            result += 1;
            grid[next] = b'^';
        }

        position = next;
    }

    result
}

/// Follow the guard's path, checking every step for a potential cycle.
pub fn part2(grid: &Grid<u8>) -> usize {
    let mut grid = grid.clone();
    let mut position = grid.find(b'^').unwrap();
    let mut direction = UP;
    let mut path = Vec::with_capacity(5_000);

    while grid.contains(position + direction) {
        if grid[position + direction] == b'#' {
            direction = direction.clockwise();
        }

        let next = position + direction;

        // Avoid double counting when the path crosses itself.
        if grid[next] == b'.' {
            path.push((position, direction));
            grid[next] = b'^';
        }

        position = next;
    }

    // Use as many cores as possible to parallelize the remaining search.
    let shortcut = Shortcut::from(&grid);
    let total = AtomicUsize::new(0);

    spawn_parallel_iterator(&path, |iter| worker(&shortcut, &total, iter));
    total.into_inner()
}

fn worker(shortcut: &Shortcut, total: &AtomicUsize, iter: ParIter<'_, (Point, Point)>) {
    let mut seen = FastSet::new();
    let result = iter
        .filter(|(position, direction)| {
            seen.clear();
            is_cycle(shortcut, &mut seen, *position, *direction)
        })
        .count();

    total.fetch_add(result, Ordering::Relaxed);
}

fn is_cycle(
    shortcut: &Shortcut,
    seen: &mut FastSet<(Point, Point)>,
    mut position: Point,
    mut direction: Point,
) -> bool {
    let obstacle = position + direction;

    while shortcut.up.contains(position) {
        // Reaching the same position in the same direction is a cycle.
        if !seen.insert((position, direction)) {
            return true;
        }

        // The tricky part is checking for the new time travelling instigated obstacle.
        position = match direction {
            UP => {
                let next = shortcut.up[position];
                if position.x == obstacle.x && position.y > obstacle.y && obstacle.y >= next.y {
                    obstacle - UP
                } else {
                    next
                }
            }
            DOWN => {
                let next = shortcut.down[position];
                if position.x == obstacle.x && position.y < obstacle.y && obstacle.y <= next.y {
                    obstacle - DOWN
                } else {
                    next
                }
            }
            LEFT => {
                let next = shortcut.left[position];
                if position.y == obstacle.y && position.x > obstacle.x && obstacle.x >= next.x {
                    obstacle - LEFT
                } else {
                    next
                }
            }
            RIGHT => {
                let next = shortcut.right[position];
                if position.y == obstacle.y && position.x < obstacle.x && obstacle.x <= next.x {
                    obstacle - RIGHT
                } else {
                    next
                }
            }
            _ => unreachable!(),
        };

        direction = direction.clockwise();
    }

    false
}

struct Shortcut {
    up: Grid<Point>,
    down: Grid<Point>,
    left: Grid<Point>,
    right: Grid<Point>,
}

impl Shortcut {
    fn from(grid: &Grid<u8>) -> Self {
        let mut up = grid.same_size_with(ORIGIN);
        let mut down = grid.same_size_with(ORIGIN);
        let mut left = grid.same_size_with(ORIGIN);
        let mut right = grid.same_size_with(ORIGIN);

        for x in 0..grid.width {
            let mut last = Point::new(x, -1);

            for y in 0..grid.height {
                let point = Point::new(x, y);
                if grid[point] == b'#' {
                    last = Point::new(x, y + 1);
                }
                up[point] = last;
            }
        }

        for x in 0..grid.width {
            let mut last = Point::new(x, grid.height);

            for y in (0..grid.height).rev() {
                let point = Point::new(x, y);
                if grid[point] == b'#' {
                    last = Point::new(x, y - 1);
                }
                down[point] = last;
            }
        }

        for y in 0..grid.height {
            let mut last = Point::new(-1, y);

            for x in 0..grid.width {
                let point = Point::new(x, y);
                if grid[point] == b'#' {
                    last = Point::new(x + 1, y);
                }
                left[point] = last;
            }
        }

        for y in 0..grid.height {
            let mut last = Point::new(grid.width, y);

            for x in (0..grid.width).rev() {
                let point = Point::new(x, y);
                if grid[point] == b'#' {
                    last = Point::new(x - 1, y);
                }
                right[point] = last;
            }
        }

        Shortcut { up, down, left, right }
    }
}
