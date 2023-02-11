use crate::util::point::*;
use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<Point> {
    input
        .trim()
        .as_bytes()
        .iter()
        .map(Point::from_byte)
        .collect()
}

pub fn part1(input: &[Point]) -> usize {
    deliver(input, |_| true)
}

pub fn part2(input: &[Point]) -> usize {
    deliver(input, |i| i % 2 == 0)
}

fn deliver(input: &[Point], predicate: fn(usize) -> bool) -> usize {
    let mut santa = ORIGIN;
    let mut robot = ORIGIN;
    let mut set = HashSet::from([ORIGIN]);

    for (index, point) in input.iter().enumerate() {
        if predicate(index) {
            santa += *point;
            set.insert(santa);
        } else {
            robot += *point;
            set.insert(robot);
        }
    }

    set.len()
}
