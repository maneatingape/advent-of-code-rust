//! # Smoke Basin
//!
//! Part two is the classic [flood fill](https://en.wikipedia.org/wiki/Flood_fill) algorithm with a
//! twist to return the size of the filled area. This algorithm can be implemented either as a
//! [DFS](https://en.wikipedia.org/wiki/Depth-first_search) using recursion or as a
//! [BFS](https://en.wikipedia.org/wiki/Breadth-first_search) using an auxiliary data structure
//! such as a [`VecDeque`].
//!
//! This solution uses a DFS approach as it's faster and Rust's stack size limit seems enough
//! to accommodate the maximum basin size. Note that when masked, newline can be treated
//! the same as `'9'` for a natural barrier that eliminates bounds checking. The [`Grid`] and
//! [`Point`] modules make it easy to perform a flood fill that tracks the lowest value seen along
//! the way, to share the work between part one and part two.
//!
//! [`VecDeque`]: std::collections::VecDeque
//! [`Grid`]: crate::util::grid
//! [`Point`]: crate::util::point
use crate::util::grid::*;
use crate::util::point::*;

pub struct Basin {
    lowest: u32, // Lowest integer seen within basin so far.
    size: u32,   // Number of cells in the basin.
}

pub fn parse(input: &str) -> Vec<Basin> {
    // A newline border allows us to avoid boundary checks.
    let mut grid = Grid::parse_with_border(input);

    // Collect all basins in the grid. Masking with 15 turns '0' through '9' into their numeric
    // value, and '\n' into 10, so that we can use newline as a second barrier character.
    let mut basins = Vec::with_capacity(256);
    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if grid[point] & 0xf < 9 {
                basins.push(flood_fill(&mut grid, point));
            }
        }
    }

    // Note that select_nth_unstable will partition the array faster than a full sort. With the
    // partition in place, the final three elements are the largest.
    let pivot = basins.len() - 3;
    basins.select_nth_unstable_by_key(pivot, |b| b.size);

    basins
}

pub fn part1(basins: &[Basin]) -> u32 {
    basins.iter().map(|b| b.lowest + 1).sum()
}

pub fn part2(basins: &[Basin]) -> u32 {
    // The list of basins is not sorted overall, but does have the largest three at the end.
    basins[basins.len() - 3..].iter().map(|b| b.size).product()
}

fn flood_fill(grid: &mut Grid<u8>, point: Point) -> Basin {
    let mut lowest = (grid[point] & 0xf) as u32;
    let mut size = 1;
    grid[point] = b'9';

    for next in ORTHOGONAL.map(|d| point + d) {
        if grid[next] & 0xf < 9 {
            let basin = flood_fill(grid, next);
            lowest = lowest.min(basin.lowest);
            size += basin.size;
        }
    }

    Basin { lowest, size }
}
