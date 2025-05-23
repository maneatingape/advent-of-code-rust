//! # A Maze of Twisty Little Cubicles
//!
//! We first generate the maze dynamically then explore it using a
//! [BFS](https://en.wikipedia.org/wiki/Breadth-first_search) solving both part one and part
//! two simultaneously.
//!
//! As we start at (1, 1) and the most steps that we are interested in is 50, we can bound
//! the maze to 2 + 50 = 52 in each dimension and used a fixed size array.

// Explicit syntax is cleaner for this case.
#![allow(clippy::needless_range_loop)]

use crate::util::parse::*;
use std::collections::VecDeque;

type Input = (u32, u32);

pub fn parse(input: &str) -> Input {
    let favorite: usize = input.unsigned();
    let mut maze = [[false; 52]; 52];

    for x in 0..52 {
        for y in 0..52 {
            let n = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + favorite;
            let ones = n.count_ones();
            maze[x][y] = ones.is_multiple_of(2);
        }
    }

    let mut part_one = 0;
    let mut part_two = 0;
    let mut todo = VecDeque::new();

    todo.push_back((1, 1, 0));
    maze[1][1] = false;

    while let Some((x, y, cost)) = todo.pop_front() {
        if x == 31 && y == 39 {
            part_one = cost;
        }
        if cost <= 50 {
            part_two += 1;
        }

        if x > 0 && maze[x - 1][y] {
            todo.push_back((x - 1, y, cost + 1));
            maze[x - 1][y] = false;
        }
        if y > 0 && maze[x][y - 1] {
            todo.push_back((x, y - 1, cost + 1));
            maze[x][y - 1] = false;
        }
        if x < 51 && maze[x + 1][y] {
            todo.push_back((x + 1, y, cost + 1));
            maze[x + 1][y] = false;
        }
        if y < 51 && maze[x][y + 1] {
            todo.push_back((x, y + 1, cost + 1));
            maze[x][y + 1] = false;
        }
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
