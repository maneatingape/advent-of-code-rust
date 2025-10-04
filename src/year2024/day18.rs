//! # RAM Run
//!
//! We use a trick to speed things up. Instead of storing `#` and `.` in the grid, we store
//! the time when a block arrives. For example:
//!
//! ```none
//!            ...#...    ∞ ∞ ∞ 3 ∞ ∞ ∞
//!     5,4    ..#....    ∞ ∞ 4 ∞ ∞ ∞ ∞
//!     4,2    ....#..    ∞ ∞ ∞ ∞ 1 ∞ ∞
//!     4,5 => ....... => ∞ ∞ ∞ ∞ ∞ ∞ ∞
//!     3,0    .....#.    ∞ ∞ ∞ ∞ ∞ 0 ∞
//!     2,1    ....#..    ∞ ∞ ∞ ∞ 2 ∞ ∞
//!            .......    ∞ ∞ ∞ ∞ ∞ ∞ ∞
//! ```
//!
//! Now we can [BFS](https://en.wikipedia.org/wiki/Breadth-first_search) from any arbitrary
//! start time. Squares are safe if the grid time is greater than the start time.
//!
//! Part two uses an incremental flood fill, getting a little further each time and removing
//! blocking bytes in descending order of time until we reach the exit.
//!
//! * Start with `t = i32::MAX`
//! * Start flood fill from top-left origin.
//! * If we encounter a blocking byte with a time less than `t`
//!   then push `(time, position)` onto a max heap keyed by time.
//! * If we exhaust the flood fill `VecDeque` then pop the heap's top item.
//!   This is the oldest byte that we encountered blocking the way.
//!   Set `t` to the byte's time and push position to the dequeue.
//! * Restart flood fill from new position until we reach the exit.
use crate::util::grid::*;
use crate::util::heap::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;
use std::collections::VecDeque;

pub fn parse(input: &str) -> Grid<i32> {
    let mut grid = Grid::new(71, 71, i32::MAX);

    for (i, [x, y]) in input.iter_signed::<i32>().chunk::<2>().enumerate() {
        grid[Point::new(x, y)] = i as i32;
    }

    grid
}

/// BFS from start to exit using a fixed time of 1024.
pub fn part1(grid: &Grid<i32>) -> u32 {
    let mut grid = grid.clone();
    let mut todo = VecDeque::new();

    grid[ORIGIN] = 0;
    todo.push_back((ORIGIN, 0));

    while let Some((position, cost)) = todo.pop_front() {
        if position == Point::new(70, 70) {
            return cost;
        }

        for next in ORTHOGONAL.map(|o| position + o) {
            if grid.contains(next) && grid[next] > 1024 {
                grid[next] = 0;
                todo.push_back((next, cost + 1));
            }
        }
    }

    unreachable!()
}

/// Incremental flood fill that removes one blocking byte at a time in descending order.
pub fn part2(grid: &Grid<i32>) -> String {
    let exit = Point::new(70, 70);

    let mut time = i32::MAX;
    let mut grid = grid.clone();
    let mut todo = VecDeque::new();
    let mut heap = MinHeap::new();

    grid[ORIGIN] = 0;
    todo.push_back(ORIGIN);

    loop {
        // Incremental flood fill that makes as much progress as possible.
        while let Some(position) = todo.pop_front() {
            if position == exit {
                let index = grid.bytes.iter().position(|&b| b == time).unwrap() as i32;
                return format!("{},{}", index % grid.width, index / grid.width);
            }

            for next in ORTHOGONAL.map(|o| position + o) {
                if grid.contains(next) {
                    if time < grid[next] {
                        grid[next] = 0;
                        todo.push_back(next);
                    } else {
                        // Use negative value to convert min-heap to max-heap.
                        heap.push(-grid[next], next);
                    }
                }
            }
        }

        // Remove the latest blocking byte then try to make a little more progress in flood fill.
        let (first, saved) = heap.pop().unwrap();
        time = -first;
        todo.push_back(saved);
    }
}
