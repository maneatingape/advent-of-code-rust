//! # Mode Maze
//!
//! Our high-level approach is an [A*](https://en.wikipedia.org/wiki/A*_search_algorithm) search.
//! This [fantastic blog](https://www.redblobgames.com/pathfinding/a-star/introduction.html)
//! is a great introduction to this algorithm. The heuristic is the
//! [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) to the target. This will
//! never overestimate the actual distance, which is an essential characteristic of the heuristic.
//! Interestingly, benchmarking showed that adding the time to switch tools if we don't have the
//! torch to the heuristic slowed things down.
//!
//! The various input files all have a target with a first coordinate less than 20, and a
//! second coordinate between 700 and 800. It is slightly faster to swap the coordinate system
//! to treat the first coordinate as the number of rows (height), and the second as the number of
//! columns (width), since memory is more efficient with row-major iteration and cross-row motion
//! when rows are short (the rest of this file generally avoids the terms `x` and `y` to minimize
//! confusion in relation to the puzzle statement).
//!
//! Part two needs a larger grid than part one, but pre-populating the larger grid during the
//! parse avoids repeated work for the portion of the grid used by both parts.
//!
//! Using A* instead of [Dijkstra](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm) results
//! in a 6x speedup on an unconstrained grid. This is because Dijkstra explores the grid evenly
//! in both axes, so if the target is 700 deep, then we will explore an area roughly 700 x 700
//! in size. In contrast, A* prefers reducing the distance to the target, exploring a narrower
//! area approximately 130 x 700 in size. On the other hand, use of an unconstrained grid does
//! unnecessary work, since all known input files can be solved with a grid of width 80. The smaller
//! grid benefits Dijkstra more than A*, although A* remains faster. The state is a tuple of
//! `(location, tool)` in order to track the time per tool separately.
//!
//! To speed things up even further we use a trick. Classic A* uses a generic priority queue that
//! can be implemented in Rust using a [`BinaryHeap`]. However, the total cost follows a strictly
//! increasing order in a constrained range of values, so we can use a much faster
//! [bucket queue](https://en.wikipedia.org/wiki/Bucket_queue). The range of the increase is from
//! 0 (moving toward the target and not changing tools) to 7 (staying put and changing tools),
//! requiring 8 buckets total.
//!
//! [`BinaryHeap`]: std::collections::BinaryHeap
use crate::util::grid::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;
use std::array::from_fn;

/// The index of each tool is the tool that *cannot* be used in that region, for example
/// Rocky => 0 => Neither, Wet => 1 => Torch, and Narrow => 2 => Climbing Gear.
const TORCH: usize = 1;
const BUCKETS: usize = 8;

/// Amount of slop beyond the target to include in the grid. Too little, and this will miss
/// paths that can take a detour to avoid a tool swap. Too large, and this will waste time
/// exploring additional points in the frontier that end up not affecting the shortest path. The
/// values picked here match empirical testing against multiple known input files, although
/// it is conceivable that an alternative cave depth may need a larger height.
const SLOP_WIDTH: i32 = 3;
const SLOP_HEIGHT: i32 = 65;

pub struct Input {
    cave: Grid<u8>, // region types for grid, (x + SLOP_HEIGHT) by (y + SLOP_WIDTH)
    height: i32,    // x coordinate of the target, < 20
    width: i32,     // y coordinate of the target, > 700
}

pub fn parse(input: &str) -> Input {
    // The puzzle describes the input as X,Y, but it is more efficient to use the numbers as
    // row,column, rearranged to have row-major iteration.
    let [depth, target_row, target_col] = input.iter_signed::<i32>().chunk::<3>().next().unwrap();

    let target = Point::new(target_col, target_row);

    let mut row = vec![0; (target_col + SLOP_WIDTH) as usize];
    let mut grid = Grid::new(target_col + SLOP_WIDTH, target_row + SLOP_HEIGHT, 0_u8);

    // Erosion levels in the first row (when puzzle X is zero) are set to a scaled geologic index.
    for c in 0..row.len() {
        row[c] = (48271 * c as i32 + depth) % 20183;
        grid[Point::new(c as i32, 0)] = (row[c] % 3) as u8;
    }

    // Remaining rows have the first column (when puzzle Y is zero) set to a scaled geologic
    // index, and other columns set by the product of two neighboring erosion levels, except
    // for the target point having a hard-coded geologic index of 0.
    for r in 1..target_row + SLOP_HEIGHT {
        let mut prev = (16807 * r + depth) % 20183;
        row[0] = prev;
        grid[Point::new(0, r)] = (row[0] % 3) as u8;

        for c in 1..target_col + SLOP_WIDTH {
            let point = Point::new(c, r);
            let c = c as usize;
            let geologic = if point == target { 0 } else { prev * row[c] };
            row[c] = (geologic + depth) % 20183;
            prev = row[c];
            grid[point] = (row[c] % 3) as u8;
        }
    }

    Input { cave: grid, height: target_row, width: target_col }
}

/// Calculate the risk level of the relevant subset of the overall cave grid.
pub fn part1(input: &Input) -> i32 {
    let Input { cave, height, width } = input;
    cave.bytes
        .chunks(cave.width as usize)
        .take(*height as usize + 1)
        .map(|row| row.iter().take(*width as usize + 1).map(|point| *point as i32).sum::<i32>())
        .sum()
}

/// A* search for the shortest path to the target.
pub fn part2(input: &Input) -> i32 {
    let &Input { height, width, cave: ref erosion } = input;
    let target = Point::new(width, height);

    // Initialize bucket queue with pre-allocated capacity to reduce reallocations needed.
    let mut base = 0;
    let mut todo: [_; BUCKETS] = from_fn(|_| Vec::with_capacity(1_000));

    // Populate times for the cave, which already has extra width and height so the search does
    // not exceed the bounds of the grid.
    // Subtle trick here. By setting the time to zero for the tool that cannot be used,
    // we implicitly disallow it during the A* search as the time to reach the square will
    // always be greater than zero.
    let mut cave = Grid::new(erosion.width, erosion.height, [i32::MAX; 3]);
    for (i, level) in erosion.bytes.iter().enumerate() {
        cave.bytes[i][*level as usize] = 0;
    }

    // Start at origin with the torch equipped.
    todo[0].push((ORIGIN, TORCH));
    cave[ORIGIN][TORCH] = 0;

    loop {
        // All items in the same bucket have the same priority.
        while let Some((point, tool)) = todo[base % BUCKETS].pop() {
            let time = cave[point][tool];

            // Check for completion.
            if point == target && tool == TORCH {
                return time;
            }

            // Move to adjacent region with the same tool.
            for next in ORTHOGONAL.map(|o| point + o) {
                // We don't need an additional check that the tool cannot be used in the
                // destination region, as the time check will take care of that.
                if cave.contains(next) && time + 1 < cave[next][tool] {
                    let heuristic = next.manhattan(target);
                    let index = (time + 1 + heuristic) as usize;

                    cave[next][tool] = time + 1;
                    todo[index % BUCKETS].push((next, tool));
                }
            }

            // Stay put and change to the other possible tool.
            for other in 0..3 {
                if time + 7 < cave[point][other] {
                    let heuristic = point.manhattan(target);
                    let index = (time + 7 + heuristic) as usize;

                    cave[point][other] = time + 7;
                    todo[index % BUCKETS].push((point, other));
                }
            }
        }

        base += 1;
    }
}
