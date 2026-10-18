//! # Race Condition
//!
//! Examining the input shows that there is only a single path from start to finish with
//! no branches. This simplifies checking for shortcuts as any empty space will be on the shortest
//! path from start to end. The cheating rules allow us to "warp" up to `n` squares away to any
//! empty space as measured by [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry).
//!
//! For part one this is a distance of 2. When checking surrounding squares we make 3 optimizations:
//!
//! * Don't check any squares only 1 away as we can always just move to these normally.
//! * Checking from point `p => q` is always the negative of `q => p`, e.g. if `p = 30, q = 50` then
//!   `p => q = 20` and `q => p = -20`. This means we only ever need to check any pair once.
//! * Additionally, we only need to check down and right. Previous rows and columns will already
//!   have checked points above and to the left when we reach an empty square.
//!
//! ```none
//!   #         .
//!  ###       ...
//! ##P## =>  ....#
//!  ###       ...
//!   #         #
//! ```
//!
//! For part two the distance is increased to 20 and the shape now resembles a wonky diamond.
//! This shape ensures complete coverage without duplicating checks.
//!
//! ```none
//!   #        .
//!  ###      ...
//! ##P## => ..P##
//!  ###      ###
//!   #        #
//! ```
use crate::util::grid::*;
use crate::util::point::*;
use crate::util::thread::*;

/// Create a grid the same size as input with the time taken from start to any location.
pub fn parse(input: &str) -> Grid<i32> {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();

    // Add padding to avoid boundary checks.
    let mut time = Grid::new(grid.width + 20, grid.height + 20, i32::MAX);
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
    [Point::new(2, 0), Point::new(0, 2)].iter().map(|&delta| cheats(time, delta)).sum()
}

/// Searches for all cheats up to distance 20, parallelizing the work over multiple threads.
pub fn part2(time: &Grid<i32>) -> u32 {
    let mut deltas = Vec::with_capacity(419);

    for x in 2..21 {
        deltas.push(Point::new(x, 0));
    }

    for y in 1..21 {
        for x in (y - 20)..(21 - y) {
            deltas.push(Point::new(x, y));
        }
    }

    // // Use as many cores as possible to parallelize the remaining search.
    spawn_parallel_iterator(&deltas, |iter| iter.map(|&delta| cheats(time, delta)).sum::<u32>())
        .into_iter()
        .sum()
}

/// Compare two slices against each other so that the compiler can auto-vectorize the loops.
/// Boundary checks are not needed due to the padding on the right and bottom of the grid.
fn cheats(time: &Grid<i32>, delta: Point) -> u32 {
    let start = Point::new(1, 1);
    let maximum = time.width * time.height;
    let threshold = 100 + delta.manhattan(ORIGIN);
    let slice_from = |point: Point| &time.bytes[(time.width * point.y + point.x) as usize..];

    slice_from(start)
        .iter()
        .zip(slice_from(start + delta))
        .map(|(&from, &to)| {
            let saved = (from - to).abs();
            u32::from(threshold <= saved && saved <= maximum)
        })
        .sum()
}
