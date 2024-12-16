//! # Reindeer Maze
//!
//! Solves part one and part two simultaneously.
//!
//! Part one is a normal [Dijkstra](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
//! search from start to end.
//!
//! Part two is a a BFS *backwards* from the end to the finish, tracing the cost exactly
//! to find all possible paths. This reuses the cost information from the Dijkstra without
//! requiring any extra state keeping for the paths.
//!
//! To speed things up even further we use a trick. Classic Dijkstra uses a generic priority queue
//! that can be implemented in Rust using a [`BinaryHeap`]. However the total cost follows a
//! strictly increasing order in a constrained range of values, so we can use a much faster
//! [bucket queue](https://en.wikipedia.org/wiki/Bucket_queue). The maximum possible increase is
//! 1000 so we need 1001 buckets.
//!
//! [`BinaryHeap`]: std::collections::BinaryHeap
use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

type Input = (u32, usize);

/// Clockwise order starting with facing right.
const DIRECTIONS: [Point; 4] = [RIGHT, DOWN, LEFT, UP];

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();

    // Forwards Dijkstra
    let mut buckets = vec![Vec::new(); 1001];
    // State is `(position, direction)`.
    let mut seen = grid.same_size_with([u32::MAX; 4]);
    let mut cost = 0;
    let mut lowest = u32::MAX;

    buckets[0].push((start, 0));
    seen[start][0] = 0;

    while lowest == u32::MAX {
        let index = (cost % 1001) as usize;

        while let Some((position, direction)) = buckets[index].pop() {
            // Once we find the end node then stop. All paths of the same cost must be in
            // this bucket, so have already been accounted for.
            if position == end {
                lowest = cost;
                break;
            }

            // -1.rem_euclid(4) = 3
            let left = (direction + 3) % 4;
            let right = (direction + 1) % 4;
            let next = [
                (position + DIRECTIONS[direction], direction, cost + 1),
                (position, left, cost + 1000),
                (position, right, cost + 1000),
            ];

            for (next_position, next_direction, next_cost) in next {
                if grid[next_position] != b'#' && next_cost < seen[next_position][next_direction] {
                    // Find the next bucket.
                    let index = (next_cost % 1001) as usize;
                    buckets[index].push((next_position, next_direction));
                    seen[next_position][next_direction] = next_cost;
                }
            }
        }

        cost += 1;
    }

    // Backwards BFS
    let mut todo = VecDeque::new();
    let mut path = grid.same_size_with(false);

    // Lowest paths can arrive at end node in multiple directions.
    for direction in 0..4 {
        if seen[end][direction] == lowest {
            todo.push_back((end, direction, lowest));
        }
    }

    while let Some((position, direction, cost)) = todo.pop_front() {
        path[position] = true;
        if position == start {
            continue;
        }

        // Reverse direction and subtract cost.
        let left = (direction + 3) % 4;
        let right = (direction + 1) % 4;
        let next = [
            (position - DIRECTIONS[direction], direction, cost - 1),
            (position, left, cost - 1000),
            (position, right, cost - 1000),
        ];

        for (next_position, next_direction, next_cost) in next {
            // Trace our cost step by step so it will exactly match possible paths.
            if next_cost == seen[next_position][next_direction] {
                todo.push_back((next_position, next_direction, next_cost));
                // Set cost back to `u32::MAX` to prevent redundant path explorations.
                seen[next_position][next_direction] = u32::MAX;
            }
        }
    }

    (lowest, path.bytes.iter().filter(|&&b| b).count())
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}
