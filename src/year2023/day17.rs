//! # Clumsy Crucible
//!
//! Our high level approach is an [A*](https://en.wikipedia.org/wiki/A*_search_algorithm) search.
//! This [fantastic blog](https://www.redblobgames.com/pathfinding/a-star/introduction.html)
//! is a great introduction to this algorithm.
//!
//! The heuristic is the [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry)
//! to the bottom right corner. This will never overestimate the actual distance which is an
//! essential characteristic in the heuristic.
//!
//! A crucial insight speeds things up. We only needs to store `(position, direction)` pairs in
//! the map of previously seen costs and do not also need to store the number of steps.
//! The reason is that each time we generate new states from the current state we loop over all
//! possible forward states. This implicitly means that every new state will always make a left or
//! right turn, alternating between horizontal and vertical movements.
//!
//! It's a little more subtle but we also don't need to store 4 directions but only 2, horizontal
//! and vertical. The reason is similar to not encoding the number of steps. As we are always
//! implictly going to make a left or right turn immediately, entering a square from the opposite
//! direction is equivalent. This reduces the storage space and time by half.
//!
//! To speed things up even further we use a trick. Classic A* uses a generic priority queue that
//! can be implemented in Rust using a [`BinaryHeap`]. However the total cost follows a strictly
//! increasing order in a constrained range of values, so we can use a much faster
//! [bucket queue](https://en.wikipedia.org/wiki/Bucket_queue). The maximum possible increase in
//! heuristic is 10 * 9 from heat plus 10 for the distance change for a total of 100 buckets.
//!
//! [`BinaryHeap`]: std::collections::BinaryHeap
use crate::util::grid::*;
use crate::util::parse::*;

/// Parse the input into a 2D grid of `u8` then convert to `u32` for convenience.
pub fn parse(input: &str) -> Grid<u32> {
    let Grid { width, height, bytes } = Grid::parse(input);
    let bytes = bytes.iter().map(|b| b.to_decimal() as u32).collect();
    Grid { width, height, bytes }
}

/// Search with a maximum of 3 steps in any direction.
pub fn part1(grid: &Grid<u32>) -> u32 {
    astar::<1, 3>(grid)
}

/// Search with a minimum of 4 and maximum of 10 steps in any direction. Using const generics
/// to specify the limits allows the compiler to optimize and unroll loops, speeding things
/// up by about 25%, versus specifying the loop limits as regular parameters.
pub fn part2(grid: &Grid<u32>) -> u32 {
    astar::<4, 10>(grid)
}

/// Optimized A* search.
fn astar<const L: usize, const U: usize>(grid: &Grid<u32>) -> u32 {
    let width = grid.width as usize;
    let height = grid.height as usize;
    let heat = &grid.bytes;

    let mut index = 0;
    let mut todo = vec![Vec::with_capacity(100); 100];
    let mut cost = vec![[0_u32; 2]; heat.len()];

    // Start from the top left corner checking both vertical and horizontal directions.
    todo[0].push((0, 0, 0));
    todo[0].push((0, 0, 1));

    loop {
        // All items in the same bucket have the same priority.
        while let Some((x, y, direction)) = todo[index % 100].pop() {
            // Retrieve cost for our current location and direction.
            let index = width * y + x;
            let steps = cost[index][direction];

            // The heuristic is used as an index into the bucket priority queue.
            let heuristic =
                |x: usize, y: usize, cost: u32| (cost as usize + width - x + height - y) % 100;

            // Check if we've reached the end.
            if x == width - 1 && y == height - 1 {
                return steps;
            }

            // Alternate directions each turn. We arbitrarily pick `0` to mean vertical and `1` to
            // mean horizontal. These constants are used as offsets into the `cost` array.
            if direction == 0 {
                // We just moved vertically so now check both left and right directions.

                // Left
                {
                    let mut index = index;
                    let mut steps = steps;

                    // Each direction loop is the same:
                    // * Check to see if we gone out of bounds
                    // * Increase the cost by the "heat" of the square we've just moved into.
                    // * Check if we've already been to this location with a lower cost.
                    // * Add new state to priority queue.
                    for i in 1..=U {
                        if i > x {
                            break;
                        }

                        index -= 1;
                        steps += heat[index];

                        if i >= L && (cost[index][1] == 0 || steps < cost[index][1]) {
                            todo[heuristic(x - i, y, steps)].push((x - i, y, 1));
                            cost[index][1] = steps;
                        }
                    }
                }

                // Right
                {
                    let mut index = index;
                    let mut steps = steps;

                    for i in 1..=U {
                        if x + i >= width {
                            break;
                        }

                        index += 1;
                        steps += heat[index];

                        if i >= L && (cost[index][1] == 0 || steps < cost[index][1]) {
                            todo[heuristic(x + i, y, steps)].push((x + i, y, 1));
                            cost[index][1] = steps;
                        }
                    }
                }
            } else {
                // We just moved horizontally so now check both up and down directions.

                // Up
                {
                    let mut index = index;
                    let mut steps = steps;

                    for i in 1..=U {
                        if i > y {
                            break;
                        }

                        index -= width;
                        steps += heat[index];

                        if i >= L && (cost[index][0] == 0 || steps < cost[index][0]) {
                            todo[heuristic(x, y - i, steps)].push((x, y - i, 0));
                            cost[index][0] = steps;
                        }
                    }
                }

                // Down
                {
                    let mut index = index;
                    let mut steps = steps;

                    for i in 1..=U {
                        if y + i >= height {
                            break;
                        }

                        index += width;
                        steps += heat[index];

                        if i >= L && (cost[index][0] == 0 || steps < cost[index][0]) {
                            todo[heuristic(x, y + i, steps)].push((x, y + i, 0));
                            cost[index][0] = steps;
                        }
                    }
                }
            }
        }

        // Bump priority by one to check the next bucket.
        index += 1;
    }
}
