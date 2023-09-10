//! # Oxygen System
//!
//! [Breadth first search](https://en.wikipedia.org/wiki/Breadth-first_search) is the simplest
//! path finding algorithm and is suitable when the cost of moving between locations is identical.
//! [This excellent blog](https://www.redblobgames.com/pathfinding/a-star/introduction.html)
//! has more detail on the various path finding algorithms that come in handy during Advent of Code.
//!
//! The tricky part is determining the shape of the maze. If we assume the maze consists only of
//! corridors of width one and has no loops or rooms, then we can use the simple
//! [wall follower](https://en.wikipedia.org/wiki/Maze-solving_algorithm#Wall_follower)
//! algorithm to eventually trace our way through the entire maze back to the starting point.
use super::intcode::*;
use crate::util::hash::*;
use crate::util::parse::*;
use crate::util::point::*;
use std::collections::VecDeque;

type Input = (FastSet<Point>, Point);

/// Build the shape of the maze using the right-hand version of the wall following algorithm.
pub fn parse(input: &str) -> Input {
    let code: Vec<_> = input.iter_signed().collect();
    let mut computer = Computer::new(&code);
    let mut first = true;
    let mut direction = UP;
    let mut position = ORIGIN;
    let mut oxygen_system = ORIGIN;
    let mut visited = FastSet::new();

    loop {
        direction = if first { direction.clockwise() } else { direction.counter_clockwise() };

        match direction {
            UP => computer.input(&[1]),
            DOWN => computer.input(&[2]),
            LEFT => computer.input(&[3]),
            RIGHT => computer.input(&[4]),
            _ => unreachable!(),
        }

        match computer.run() {
            State::Output(0) => first = false,
            State::Output(result) => {
                first = true;
                position += direction;
                visited.insert(position);

                if result == 2 {
                    oxygen_system = position;
                }
                if position == ORIGIN {
                    break;
                }
            }
            _ => unreachable!(),
        }
    }

    (visited, oxygen_system)
}

/// BFS from the starting point until we find the oxygen system.
pub fn part1(input: &Input) -> i32 {
    let (mut maze, oxygen_system) = input.clone();
    let mut todo = VecDeque::from([(ORIGIN, 0)]);

    while let Some((point, cost)) = todo.pop_front() {
        maze.remove(&point);
        if point == oxygen_system {
            return cost;
        }

        for movement in ORTHOGONAL {
            let next_point = point + movement;
            if maze.contains(&next_point) {
                todo.push_back((next_point, cost + 1));
            }
        }
    }

    unreachable!()
}

/// BFS from the oxygen system to all points in the maze.
pub fn part2(input: &Input) -> i32 {
    let (mut maze, oxygen_system) = input.clone();
    let mut todo = VecDeque::from([(oxygen_system, 0)]);
    let mut minutes = 0;

    while let Some((point, cost)) = todo.pop_front() {
        maze.remove(&point);
        minutes = minutes.max(cost);

        for movement in ORTHOGONAL {
            let next_point = point + movement;
            if maze.contains(&next_point) {
                todo.push_back((next_point, cost + 1));
            }
        }
    }

    minutes
}
