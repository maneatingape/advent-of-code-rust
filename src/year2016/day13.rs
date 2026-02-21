//! # A Maze of Twisty Little Cubicles
//!
//! We first generate the maze dynamically then explore it using a
//! [BFS](https://en.wikipedia.org/wiki/Breadth-first_search) solving both part one and part
//! two simultaneously.
//!
//! As we start at (1, 1) and the most steps that we are interested in for part two is 50, while
//! part one requires more than 50 steps but should easily be reachable without exceeding bounds,
//! we can bound the maze to 2 + 50 = 52 in each dimension and use a fixed size array.  Rather
//! than filling the array up front, we can lazily populate it as the horizon expands.

use crate::util::parse::*;
use std::collections::VecDeque;

type Input = (u32, u32);

pub fn parse(input: &str) -> Input {
    let favorite: usize = input.unsigned();

    // Lazy evaluation: set maze[x][y] to true once a point is visited
    let mut maze = [[false; 52]; 52];
    maze[1][1] = true;
    let mut at = |x: usize, y: usize| -> bool {
        if maze[x][y] {
            return false;
        }
        maze[x][y] = true;
        let n = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + favorite;
        n.count_ones().is_multiple_of(2)
    };

    let mut part_two = 0;
    let mut todo = VecDeque::new();

    todo.push_back((1, 1, 0));

    while let Some((x, y, cost)) = todo.pop_front() {
        // Target is at least 68 moves from the start. Since we're doing a BFS we're guaranteed
        // to have checked all locations less than or equal to 50 before reaching the target.
        if x == 31 && y == 39 {
            return (cost, part_two);
        }
        if cost <= 50 {
            part_two += 1;
        }

        if x > 0 && at(x - 1, y) {
            todo.push_back((x - 1, y, cost + 1));
        }
        if y > 0 && at(x, y - 1) {
            todo.push_back((x, y - 1, cost + 1));
        }
        if at(x + 1, y) {
            todo.push_back((x + 1, y, cost + 1));
        }
        if at(x, y + 1) {
            todo.push_back((x, y + 1, cost + 1));
        }
    }

    unreachable!()
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
