//! # Oxygen System
//!
//! [Breadth first search](https://en.wikipedia.org/wiki/Breadth-first_search) is the simplest
//! path finding algorithm and is suitable when the cost of moving between locations is identical.
//! [This excellent blog](https://www.redblobgames.com/pathfinding/a-star/introduction.html)
//! has more detail on the various path finding algorithms that come in handy during Advent of Code.
//!
//! The tricky part is that our droid is stateful. If we move to a new location without fully
//! exploring the previous location, then we would somehow have to retrace our steps.
//!
//! This solution side-steps that issue by using the [`clone`] method to take a snapshot
//! of our droid at every location. We can then restart from any location using that snapshot.
//! Cloning is a relatively slow operation, due to the large `vec` of intcode instructions, so we
//! optimize by cloning as little as possible, using 3 tricks:
//!
//! * If the result of movement is `0` then the droid is still in the same location and can be
//!   re-used without cloning. We store a clone of the droid in an option using the [`take`] and
//!   [`replace`] methods to avoid the compiler complaining.
//! * We maximize the chances of getting a wall collision first by keeping track of the
//!   direction that the droid came from. The maze consists mostly of long narrow corridors, so
//!   for example if we are heading South, then trying East and West will most likely be walls.
//!   We try South last and can skip North entirely as we came from that direction.
//! * Finally we don't need to any more clones when trying the last direction from a location
//!   as there are no more possibilites to try. Otherwise we make a clone then restore the droid
//!   to the previous location using a reverse direction command.
//!
//! For part two, we perform a *second* BFS, starting from the location of the oxygen system. This
//! uses the data gathered from part one, so is much faster as it doesn't need to run the
//! intcode program.
//!
//! [`clone`]: std::clone::Clone
//! [`take`]: Option::take
//! [`replace`]: Option::replace
use super::day09::intcode::*;
use crate::util::hash::*;
use crate::util::parse::*;
use crate::util::point::*;
use std::collections::VecDeque;

/// Maximize chances of colliding with a wall when searching directions. The maze consists
/// of long narrow corridors, so checking perpendicular to our direction will most likely
/// hit a wall.
const DIRECTIONS: [&[(i64, Point)]; 5] = [
    &[(1, UP), (2, DOWN), (3, LEFT), (4, RIGHT)],
    &[(3, LEFT), (4, RIGHT), (1, UP)],
    &[(3, LEFT), (4, RIGHT), (2, DOWN)],
    &[(1, UP), (2, DOWN), (3, LEFT)],
    &[(1, UP), (2, DOWN), (4, RIGHT)],
];

/// Reverse direction lookup used to restore droid to previous location.
const REVERSE: [i64; 5] = [0, 2, 1, 4, 3];

type Input = (i32, i32);

pub fn parse(input: &str) -> Input {
    let code: Vec<_> = input.iter_signed().collect();
    let computer = Computer::new(&code);

    // Breadth first search over all possible points in the maze. As we need the complete maze for
    // part two we intentionally don't exit early when the oxygen system is found.
    let mut todo = VecDeque::from([(0, 0, ORIGIN, computer)]);
    let mut visited = FastSet::build([ORIGIN]);
    let mut oxygen_system = (0, ORIGIN);

    while let Some((cost, from, point, computer)) = todo.pop_front() {
        let limit = DIRECTIONS[from as usize].len() - 1;
        let iter = DIRECTIONS[from as usize].iter().enumerate();

        let mut storage = Some(computer);

        for (index, &(command, movement)) in iter {
            let next_cost = cost + 1;
            let next_point = point + movement;

            if visited.contains(&next_point) {
                continue;
            }

            let mut next_computer = storage.take().unwrap();
            next_computer.input(&[command]);
            let State::Output(result) = next_computer.run() else {
                unreachable!();
            };

            if result == 0 {
                // The droid is still at the same location, so put it back into the option.
                storage.replace(next_computer);
            } else {
                if result == 2 {
                    oxygen_system = (next_cost, next_point);
                }

                // We moved so restore the snapshot of previous location as long as there
                // are more directions to try.
                if index < limit {
                    let mut restore = next_computer.clone();
                    restore.input(&[REVERSE[command as usize]]);
                    restore.run();
                    storage.replace(restore);
                }

                todo.push_back((next_cost, command, next_point, next_computer));
                visited.insert(next_point);
            }
        }
    }

    // Start a second search from the location of the oxygen system. To speed things up we re-use
    // the points from the first search to avoid needing to run the intcode program.
    let mut todo = VecDeque::from([(0, oxygen_system.1)]);
    let mut minutes = 0;

    while let Some((cost, point)) = todo.pop_front() {
        visited.remove(&point);
        minutes = minutes.max(cost);

        for movement in ORTHOGONAL {
            let next_cost = cost + 1;
            let next_point = point + movement;

            if visited.contains(&next_point) {
                todo.push_back((next_cost, next_point));
            }
        }
    }

    (oxygen_system.0, minutes)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}
