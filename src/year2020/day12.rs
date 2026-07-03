//! # Rain Risk
//!
//! On this problem parsing takes almost all the time, so for maximum speed
//! a custom parser solves both parts during a single pass over the input bytes
use crate::util::parse::*;
use crate::util::point::*;

type Input = (i32, i32);

pub fn parse(input: &str) -> Input {
    let first = input.bytes().filter(u8::is_ascii_uppercase);
    let second = input.iter_signed();

    let mut part_one = ORIGIN;
    let mut part_two = ORIGIN;
    let mut direction = RIGHT;
    let mut waypoint = Point::new(10, -1);

    for (command, amount) in first.zip(second) {
        match command {
            b'N' => {
                part_one.y -= amount;
                waypoint.y -= amount;
            }
            b'S' => {
                part_one.y += amount;
                waypoint.y += amount;
            }
            b'E' => {
                part_one.x += amount;
                waypoint.x += amount;
            }
            b'W' => {
                part_one.x -= amount;
                waypoint.x -= amount;
            }
            b'L' => {
                direction = rotate(direction, 360 - amount);
                waypoint = rotate(waypoint, 360 - amount);
            }
            b'R' => {
                direction = rotate(direction, amount);
                waypoint = rotate(waypoint, amount);
            }
            b'F' => {
                part_one += direction * amount;
                part_two += waypoint * amount;
            }
            _ => unreachable!(),
        }
    }

    (part_one.manhattan(ORIGIN), part_two.manhattan(ORIGIN))
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}

fn rotate(point: Point, amount: i32) -> Point {
    match amount {
        90 => point.clockwise(),
        180 => point * -1,
        270 => point.counter_clockwise(),
        _ => unreachable!(),
    }
}
