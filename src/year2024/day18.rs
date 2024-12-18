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
//! start time. Squares are blocked only if the grid time is less than or equal to the start time.
//!
//! A [binary search](https://en.wikipedia.org/wiki/Binary_search) is much faster than a
//! linear search with complexity `O(log₂n)` vs `O(n)`. For example `log₂(3450) = 12`.
use crate::util::grid::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;
use std::collections::VecDeque;

pub fn parse(input: &str) -> Grid<u16> {
    let mut grid = Grid::new(71, 71, u16::MAX);

    for (i, [x, y]) in input.iter_signed::<i32>().chunk::<2>().enumerate() {
        grid[Point::new(x, y)] = i as u16;
    }

    grid
}

pub fn part1(grid: &Grid<u16>) -> u32 {
    bfs(grid, 1024).unwrap()
}

pub fn part2(grid: &Grid<u16>) -> String {
    let mut lower = 0;
    let mut upper = 5041;

    while lower < upper {
        let middle = (lower + upper) / 2;
        if bfs(grid, middle).is_some() {
            lower = middle + 1;
        } else {
            upper = middle;
        }
    }

    let index = grid.bytes.iter().position(|&time| time == lower).unwrap() as i32;
    format!("{},{}", index % grid.width, index / grid.width)
}

fn bfs(grid: &Grid<u16>, time: u16) -> Option<u32> {
    let mut todo = VecDeque::new();
    let mut seen = grid.clone();

    todo.push_back((ORIGIN, 0));
    seen[ORIGIN] = 0;

    while let Some((position, cost)) = todo.pop_front() {
        if position == Point::new(70, 70) {
            return Some(cost);
        }

        for next in ORTHOGONAL.map(|o| position + o) {
            if seen.contains(next) && time < seen[next] {
                todo.push_back((next, cost + 1));
                seen[next] = 0;
            }
        }
    }

    None
}
