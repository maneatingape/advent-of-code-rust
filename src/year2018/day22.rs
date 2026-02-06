//! # Mode Maze
//!
//! Our high-level approach is an [A*](https://en.wikipedia.org/wiki/A*_search_algorithm) search.
//! This [fantastic blog](https://www.redblobgames.com/pathfinding/a-star/introduction.html)
//! is a great introduction to this algorithm. The heuristic is the
//! [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) to the target. This will
//! never overestimate the actual distance which is an essential characteristic in the heuristic.
//! Interestingly benchmarking showed that adding the time to switch tools if we don't have the
//! torch to the heuristic slowed things down.
//!
//! Using A* instead of [Dijkstra](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm) results
//! in a 6x speedup. This is because Dijkstra explores the grid evenly in both axes, so if the
//! target is 700 deep, then we will explore an area roughly 700 x 700 in size. In contrast A*
//! prefers reducing the distance to the target, exploring a more narrow area
//! approximately 130 x 700 in size. The state is a tuple of `(location, tool)` in order to track
//! the time per tool separately.
//!
//! To speed things up even further we use a trick. Classic A* uses a generic priority queue that
//! can be implemented in Rust using a [`BinaryHeap`]. However the total cost follows a strictly
//! increasing order in a constrained range of values, so we can use a much faster
//! [bucket queue](https://en.wikipedia.org/wiki/Bucket_queue). The range of the increase is from
//! 0 (moving toward the target and not changing tools) to 7 (staying put and changing tools)
//! requiring 8 buckets total.
//!
//! [`BinaryHeap`]: std::collections::BinaryHeap
use crate::util::grid::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;
use std::array::from_fn;

/// The index of each tool is that tool that *cannot* be used in that region, for example
/// Rocky => 0 => Neither, Wet => 1 => Torch and Narrow => 2 => Climbing Gear.
const TORCH: usize = 1;
const BUCKETS: usize = 8;

type Input = [i32; 3];

#[derive(Clone, Copy)]
struct Region {
    erosion: i32,
    minutes: [i32; 3],
}

impl Region {
    fn update(&mut self, geologic: i32) -> i32 {
        let erosion = geologic % 20183;
        self.erosion = erosion;
        // Subtle trick here. By setting the time to zero for the tool that cannot be used,
        // we implicitly disallow it during the A* search as the time to reach the square will
        // always be greater than zero.
        self.minutes[(erosion % 3) as usize] = 0;
        erosion
    }
}

pub fn parse(input: &str) -> Input {
    input.iter_signed::<i32>().chunk::<3>().next().unwrap()
}

/// Build the minimum grid to the target then calculate the risk level.
pub fn part1(input: &Input) -> i32 {
    let [_, height, width] = *input;
    let cave = scan_cave(input, width + 1, height + 1);
    cave.bytes.iter().map(|r| r.erosion % 3).sum()
}

/// A* search for the shortest path to the target.
pub fn part2(input: &Input) -> i32 {
    // Swap width and height.
    let [_, height, width] = *input;
    let target = Point::new(width, height);

    // Initialise bucket queue with pre-allocated capacity to reduce reallocations needed.
    let mut base = 0;
    let mut todo: [_; BUCKETS] = from_fn(|_| Vec::with_capacity(1000));

    // Add extra width and height so the search does not exceed the bounds of the grid.
    let mut cave = scan_cave(input, width + 10, height + 140);

    // Start at origin with the torch equipped.
    todo[0].push((ORIGIN, TORCH));
    cave[ORIGIN].minutes[TORCH] = 0;

    loop {
        // All items in the same bucket have the same priority.
        while let Some((point, tool)) = todo[base % BUCKETS].pop() {
            let time = cave[point].minutes[tool];

            // Check for completion.
            if point == target && tool == TORCH {
                return time;
            }

            // Move to adjacent region with the same tool.
            for next in ORTHOGONAL.map(|o| point + o) {
                // We don't need an additional check that the tool cannot be used in the
                // destination region, as the time check will take care of that.
                if next.x >= 0 && next.y >= 0 && time + 1 < cave[next].minutes[tool] {
                    let heuristic = next.manhattan(target);
                    let index = (time + 1 + heuristic) as usize;

                    cave[next].minutes[tool] = time + 1;
                    todo[index % BUCKETS].push((next, tool));
                }
            }

            // Stay put and change to the other possible tool.
            for other in 0..3 {
                if time + 7 < cave[point].minutes[other] {
                    let heuristic = point.manhattan(target);
                    let index = (time + 7 + heuristic) as usize;

                    cave[point].minutes[other] = time + 7;
                    todo[index % BUCKETS].push((point, other));
                }
            }
        }

        base += 1;
    }
}

/// Calculate the erosion level for each region. We swap width and height for a small speed boost
/// without affecting the outcome of the shortest path.
fn scan_cave(input: &Input, width: i32, height: i32) -> Grid<Region> {
    let [depth, target_y, target_x] = *input;
    let target = Point::new(target_x, target_y);

    let region = Region { erosion: 0, minutes: [i32::MAX; 3] };
    let mut grid = Grid::new(width, height, region);

    grid[Point::new(0, 0)].update(depth);

    for x in 1..width {
        grid[Point::new(x, 0)].update(48271 * x + depth);
    }

    for y in 1..height {
        let mut prev = grid[Point::new(0, y)].update(16807 * y + depth);

        for x in 1..width {
            let point = Point::new(x, y);
            if point == target {
                grid[point].update(depth);
            } else {
                let up = grid[point + UP].erosion;
                prev = grid[point].update(prev * up + depth);
            }
        }
    }

    grid
}
