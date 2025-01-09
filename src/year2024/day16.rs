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

    // Forwards Dijkstra. Since turns are so much more expensive than moving forward, we can
    // treat this as a glorified BFS using two priority queues. This is much faster than using
    // an actual min heap.
    let mut todo_first = VecDeque::new();
    let mut todo_second = VecDeque::new();
    // State is `(position, direction)`.
    let mut seen = grid.same_size_with([u32::MAX; 4]);
    let mut lowest = u32::MAX;

    todo_first.push_back((start, 0, 0));
    seen[start][0] = 0;

    while !todo_first.is_empty() {
        while let Some((position, direction, cost)) = todo_first.pop_front() {
            if cost >= lowest {
                continue;
            }
            if position == end {
                lowest = cost;
                continue;
            }

            // -1.rem_euclid(4) = 3
            let left = (direction + 3) % 4;
            let right = (direction + 1) % 4;
            let next = [
                (position + DIRECTIONS[direction], direction, cost + 1),
                (position, left, cost + 1000),
                (position, right, cost + 1000),
            ];

            for tuple @ (next_position, next_direction, next_cost) in next {
                if grid[next_position] != b'#' && next_cost < seen[next_position][next_direction] {
                    // Find the next bucket.
                    if next_direction == direction {
                        todo_first.push_back(tuple);
                    } else {
                        todo_second.push_back(tuple);
                    }
                    seen[next_position][next_direction] = next_cost;
                }
            }
        }

        (todo_first, todo_second) = (todo_second, todo_first);
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

        // println!("cost = {cost}, lowest = {lowest}, path.bytes...count() = {}", path.bytes.iter().filter(|&&b| b).count());
        let next = [
            (position - DIRECTIONS[direction], direction, cost - 1),
            //
            // This isn't a clever or performant fix, but prevents the: "attempt to subtract with overflow" error for my input.
            // 
            // I think the correct fix is to capture the length of the initial "same direction" moves from the start
            // and add that length to the final result without processing that part of the path here??
            // 
            // Hopefully this little bit of debug output before the panic will suffice.
            // The correct answer for my input is 467      
            //
            // cost = 1005, lowest = 83432, path.bytes...count() = 462
            // cost = 1004, lowest = 83432, path.bytes...count() = 463
            // cost = 4, lowest = 83432, path.bytes...count() = 463
            // cost = 3, lowest = 83432, path.bytes...count() = 464
            // cost = 2, lowest = 83432, path.bytes...count() = 465
            // cost = 1, lowest = 83432, path.bytes...count() = 466
            // 2024 Day 16
            // Part 1: 83432
            // Part 2: 467
            (position, left, match cost {
                0..1000 => 0,
                _ => cost - 1000
            }),
            (position, right, match cost {
                0..1000 => 0,
                _ => cost - 1000
            }),
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
