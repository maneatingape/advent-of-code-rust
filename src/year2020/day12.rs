//! # Rain Risk
//!
//! Our [`Point`] utility class comes in handy for this problem.
//!
//! [`Point`]: crate::util::point
use crate::util::parse::*;
use crate::util::point::*;

type Command = (u8, i32);

pub fn parse(input: &str) -> Vec<Command> {
    input.lines().map(|line| (line.as_bytes()[0], (&line[1..]).signed())).collect()
}

pub fn part1(input: &[Command]) -> i32 {
    let mut position = ORIGIN;
    let mut direction = Point { x: 1, y: 0 };

    for &(command, amount) in input {
        match command {
            b'N' => position.y -= amount,
            b'S' => position.y += amount,
            b'E' => position.x += amount,
            b'W' => position.x -= amount,
            b'L' => direction = rotate(direction, -amount),
            b'R' => direction = rotate(direction, amount),
            b'F' => position += direction * amount,
            _ => unreachable!(),
        }
    }

    position.manhattan(ORIGIN)
}

pub fn part2(input: &[Command]) -> i32 {
    let mut position = ORIGIN;
    let mut waypoint = Point { x: 10, y: -1 };

    for &(command, amount) in input {
        match command {
            b'N' => waypoint.y -= amount,
            b'S' => waypoint.y += amount,
            b'E' => waypoint.x += amount,
            b'W' => waypoint.x -= amount,
            b'L' => waypoint = rotate(waypoint, -amount),
            b'R' => waypoint = rotate(waypoint, amount),
            b'F' => position += waypoint * amount,
            _ => unreachable!(),
        }
    }

    position.manhattan(ORIGIN)
}

fn rotate(point: Point, amount: i32) -> Point {
    match amount {
        90 | -270 => point.clockwise(),
        180 | -180 => point * -1,
        270 | -90 => point.counter_clockwise(),
        _ => unreachable!(),
    }
}
