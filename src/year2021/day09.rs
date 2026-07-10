//! # Smoke Basin
//!
//! Part two is the classic [flood fill](https://en.wikipedia.org/wiki/Flood_fill) algorithm with a
//! twist to return the size of the filled area. This algorithm can be implemented either as a
//! [DFS](https://en.wikipedia.org/wiki/Depth-first_search) using recursion or as a
//! [BFS](https://en.wikipedia.org/wiki/Breadth-first_search) using an auxiliary data structure
//! such as a [`VecDeque`].
//!
//! This solution uses a DFS approach as it's faster and Rust's stack size limit seems enough
//! to accommodate the maximum basin size. While we could use the [`Grid`] and [`Point`] modules
//! to take in the original grid one line at a time with 2D coordinates, it turns out to be
//! somewhat faster to instead just operate on a 1D array with an explicit border, where newline
//! is treated the same as `'9'`, in order to eliminate bounds checking. We can also tweak the
//! flood fill to track the lowest value seen along the way, to share the work between part
//! one and part two.
//!
//! [`VecDeque`]: std::collections::VecDeque
//! [`Grid`]: crate::util::grid
//! [`Point`]: crate::util::point

pub struct Basin {
    lowest: u32, // Lowest integer seen within basin so far.
    size: u32,   // Number of cells in the basin.
}

pub fn parse(input: &str) -> Vec<Basin> {
    // Create a larger grid with all borders already filled with 9.
    let width = input.lines().next().unwrap().len() + 1;
    let mut grid = Vec::with_capacity(input.len() + 2 * width);
    grid.resize(width, b'9');
    grid.extend_from_slice(input.as_bytes());
    grid.resize(grid.len() + width, b'9');

    // Collect all basins in the grid. Masking with 15 turns '0' through '9' into their numeric
    // value, and '\n' into 10, so that we can use newline as a second barrier character.
    let mut basins = Vec::with_capacity(256);
    for idx in width..width + input.len() {
        if grid[idx] & 15 < 9 {
            basins.push(flood_fill(&mut grid, idx, width as isize));
        }
    }

    // Note that select_nth_unstable will partition the array faster than a full sort; with the
    // partition in place, the final three elements are the largest.
    let pivot = basins.len() - 3;
    basins.select_nth_unstable_by_key(pivot, |b| b.size);

    basins
}

pub fn part1(basins: &[Basin]) -> u32 {
    basins.iter().map(|b| b.lowest + 1).sum::<u32>()
}

pub fn part2(basins: &[Basin]) -> u32 {
    // The list of basins is not sorted overall, but does have the largest three at the end.
    basins[basins.len() - 3..].iter().map(|b| b.size).product()
}

fn flood_fill(grid: &mut [u8], idx: usize, width: isize) -> Basin {
    let mut lowest = (grid[idx] & 15) as u32;
    let mut size = 1;
    grid[idx] = b'9';

    for delta in [1, -1, width, -width] {
        let other = idx.wrapping_add(delta as usize);
        if grid[other] & 15 < 9 {
            let basin = flood_fill(grid, other, width);
            lowest = lowest.min(basin.lowest);
            size += basin.size;
        }
    }

    Basin { lowest, size }
}
