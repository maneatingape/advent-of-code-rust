//! # Perfectly Spherical Houses in a Vacuum
//!
//! We store Santa's path in a [`FastSet`] of [`Point`] objects that deduplicates visited points.
//! For part two we alternate between Santa and the robot, tracking two points simultaneously and
//! reusing the same deduplicating logic as part one.
//!
//! [`FastSet`]: crate::util::hash
//! [`Point`]: crate::util::point
use crate::util::hash::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Vec<Point> {
    input.trim().bytes().map(Point::from).collect()
}

pub fn part1(input: &[Point]) -> usize {
    deliver(input, |_| true)
}

pub fn part2(input: &[Point]) -> usize {
    deliver(input, |i| i.is_multiple_of(2))
}

fn deliver(input: &[Point], predicate: fn(usize) -> bool) -> usize {
    let mut santa = ORIGIN;
    let mut robot = ORIGIN;

    let mut set = FastSet::with_capacity(input.len());
    set.insert(ORIGIN);

    for (index, &point) in input.iter().enumerate() {
        if predicate(index) {
            santa += point;
            set.insert(santa);
        } else {
            robot += point;
            set.insert(robot);
        }
    }

    set.len()
}
