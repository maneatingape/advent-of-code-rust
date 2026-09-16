//! # Clumsy Crucible
//!
//! Our high-level approach is an [A*](https://en.wikipedia.org/wiki/A*_search_algorithm) search.
//! This [fantastic blog](https://www.redblobgames.com/pathfinding/a-star/introduction.html)
//! is a great introduction to this algorithm.
//!
//! A crucial insight speeds things up. We only need to store `(position, direction)` pairs in
//! the map of previously seen costs and do not also need to store the number of steps.
//! The reason is that each time we generate new states from the current state we loop over all
//! possible forward states. This implicitly means that every new state will always make a left or
//! right turn, alternating between horizontal and vertical movements.
//!
//! It's a little more subtle but we also don't need to store 4 directions but only 2, horizontal
//! and vertical. The reason is similar to not encoding the number of steps. As we are always
//! implicitly going to make a left or right turn immediately, entering a square from the opposite
//! direction is equivalent. This reduces the storage space and time by half.
//!
//! ## Heuristic
//!
//! The obvious heuristic is the [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry)
//! to the bottom right corner. This never overestimates the actual cost, however it is so weak that
//! the search ends up visiting almost every state in the grid.
//!
//! Instead we spend a little time up front computing a much sharper bound. Relaxing the puzzle by
//! dropping the straight line rule entirely leaves a plain grid shortest path problem. Any real
//! crucible route is also a valid route in the relaxed problem, so the relaxed distance from each
//! square to the bottom right corner can never exceed the true remaining cost. The relaxed
//! distances are computed once during parsing with a backwards [Dijkstra](https://en.wikipedia.org/wiki/Dijkstra's_algorithm)
//! from the bottom right corner then shared with both parts.
//!
//! ## Implementation
//!
//! Classic A* uses a generic priority queue that can be implemented in Rust using a [`BinaryHeap`].
//! However the total cost follows a strictly increasing order in a constrained range of values, so
//! we can use a much faster [bucket queue](https://en.wikipedia.org/wiki/Bucket_queue).
//!
//! As the buckets are drained in increasing cost order, an entry is stale if its cost no longer
//! agrees with the bucket it was found in. Checking this skips roughly half the states in part two.
//!
//! Finally the grid is surrounded by a border of zero cost squares. A square is only worth visiting
//! if it improves on the previous best cost, and nothing improves on zero, so the search can move
//! blindly in a straight line without a single bounds check.
//!
//! [`BinaryHeap`]: std::collections::BinaryHeap
use std::array::from_fn;

use crate::util::grid::*;
use crate::util::parse::*;

/// Border is the size of the longest possible straight line.
const BORDER: usize = 10;

pub struct Input {
    size: usize,
    stride: usize,
    start: usize,
    end: usize,
    heat: Vec<u8>,
    heuristic: Vec<u16>,
}

/// Parse the input into a bordered grid then precompute the heuristic shared by both parts.
pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let size = grid.width as usize;
    let stride = size + 2 * BORDER;
    let start = stride * BORDER + BORDER;
    let end = stride * stride - start - 1;

    let mut heat = vec![0; stride * stride];
    let mut heuristic = vec![0; stride * stride];

    for y in 0..size {
        for x in 0..size {
            heat[start + y * stride + x] = grid.bytes[y * size + x].to_decimal();
            heuristic[start + y * stride + x] = u16::MAX;
        }
    }

    let mut input = Input { size, stride, start, end, heat, heuristic };
    dijkstra(&mut input);
    input
}

/// Search with a maximum of 3 steps in any direction.
pub fn part1(input: &Input) -> u16 {
    astar::<1, 3>(input)
}

/// Search with a minimum of 4 and maximum of 10 steps in any direction. Using const generics
/// to specify the limits allows the compiler to optimize and unroll loops, speeding things
/// up by about 5%, versus specifying the loop limits as regular parameters.
pub fn part2(input: &Input) -> u16 {
    astar::<4, 10>(input)
}

/// Cost to each square from the bottom right corner if the crucible could turn freely.
///
/// Entering a square always costs that square's heat, so the reverse edge from a square to each
/// of its neighbours has the same weight in every direction.
fn dijkstra(input: &mut Input) {
    let Input { size, stride, end, .. } = *input;
    let Input { heat, heuristic, .. } = input;

    let mut loss = 0;
    let mut remaining = size * size;
    let mut todo: [_; 10] = from_fn(|_| Vec::with_capacity(100));

    heuristic[end] = 0;
    todo[0].push(end);

    while remaining > 0 {
        while let Some(position) = todo[loss % 10].pop() {
            // Skip stale entries.
            if heuristic[position] as usize == loss {
                remaining -= 1;

                let cost = loss as u16 + heat[position] as u16;
                let bucket = cost as usize % 10;

                for next in [position - 1, position + 1, position - stride, position + stride] {
                    if cost < heuristic[next] {
                        heuristic[next] = cost;
                        todo[bucket].push(next);
                    }
                }
            }
        }

        loss += 1;
    }
}

/// Optimized A* search.
fn astar<const L: usize, const U: usize>(input: &Input) -> u16 {
    let Input { size, stride, start, end, .. } = *input;
    let Input { heat, heuristic, .. } = input;

    // The border remains zero so that squares outside the grid are never visited.
    let mut cost = vec![[0; 2]; heat.len()];

    for y in 0..size {
        let from = start + y * stride;
        cost[from..from + size].fill([u16::MAX; 2]);
    }

    // Total cost of both starting states is the heuristic alone.
    let mut loss = heuristic[start] as usize;
    let mut todo: [_; 100] = from_fn(|_| Vec::with_capacity(1_000));

    // We arbitrarily pick `0` to mean vertical and `1` to mean horizontal, stored in the lowest
    // bit of the state alongside the position.
    todo[loss % 100].push(start << 1);
    todo[loss % 100].push((start << 1) | 1);
    cost[start] = [0; 2];

    loop {
        // All items in the same bucket have the same priority.
        while let Some(state) = todo[loss % 100].pop() {
            let position = state >> 1;
            let direction = state & 1;
            let steps = cost[position][direction];

            // A cheaper route to this state was found after it was added to the queue.
            if steps as usize + heuristic[position] as usize != loss {
                continue;
            }

            // Check if we've reached the end.
            if position == end {
                return steps;
            }

            // Alternate directions each turn, so a vertical arrival leaves horizontally
            // and vice-versa.
            let turn = 1 - direction;
            let delta = if direction == 0 { 1 } else { stride };

            // Both directions along the new axis are the same:
            // * Increase the cost by the "heat" of the square we've just moved into.
            // * Check if we've already been to this square with a lower cost.
            // * Add new state to priority queue.
            for delta in [delta, delta.wrapping_neg()] {
                let mut next = position;
                let mut extra = steps;

                for i in 1..U + 1 {
                    next = next.wrapping_add(delta);
                    extra += heat[next] as u16;

                    if i >= L && extra < cost[next][turn] {
                        cost[next][turn] = extra;
                        let bucket = (extra as usize + heuristic[next] as usize) % 100;
                        todo[bucket].push((next << 1) | turn);
                    }
                }
            }
        }

        // Bump priority by one to check the next bucket.
        loss += 1;
    }
}
