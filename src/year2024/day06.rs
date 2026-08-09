//! # Guard Gallivant
//!
//! Part two is sped up by pre-computing the next obstacle in each direction from any point in
//! the grid. If there is nothing left in the way then coordinates outside the grid are used.
//! One dimensional example:
//!
//! ```none
//! .#...
//! Left: (-1, 2, 2, 2, 2)
//! Right: (1, 1, 5, 5, 5)
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

        // Scan each row or column *against* the direction of travel, remembering the square just
        // before the most recent obstacle. Starting one square off the grid means that
        // coordinates outside the grid are used when nothing is in the way.
        let scan = |dst: &mut Grid<Point>, start: Point, step: Point, count: i32| {
            let mut last = start - step;
            let mut point = start;

            for _ in 0..count {
                if grid[point] == b'#' {
                    last = point + step;
                }
                dst[point] = last;
                point += step;
            }
        };

        // Process columns for up/down.
        for x in 0..grid.width {
            scan(&mut up, Point::new(x, 0), DOWN, grid.height);
            scan(&mut down, Point::new(x, grid.height - 1), UP, grid.height);
        }

        // Process rows for left/right.
        for y in 0..grid.height {
            scan(&mut left, Point::new(0, y), RIGHT, grid.width);
            scan(&mut right, Point::new(grid.width - 1, y), LEFT, grid.width);
        }

        Self { up, down, left, right }
    }
}

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
    let result = spawn_parallel_iterator(&path, |iter| worker(&shortcut, iter));
    result.into_iter().sum()
}

fn worker(shortcut: &Shortcut, iter: ParIter<'_, (Point, Point)>) -> usize {
    let mut seen = FastSet::new();
    iter.filter(|&&(position, direction)| {
        seen.clear();
        is_cycle(shortcut, &mut seen, position, direction)
    })
    .count()
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

        // The tricky part is checking for the newly introduced time-traveling obstacle.
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
