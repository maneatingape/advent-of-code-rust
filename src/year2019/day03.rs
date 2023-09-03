//! # Crossed Wires
//!
//! The input follow some implicit rules that can be used to simplify our approach:
//!
//! * Wires cross only at right angles to each other, so we only need to consider horizontal lines
//!   when moving vertically and vice-versa.
//! * There is only a single vertical line at a given x coordinates and vice-versa.
//!
//! This makes [`BTreeMap`] a great choice to store horizontal or vertical line segments as there
//! are no collisions. The [`range`] method can lookup all line segments contained between two
//! coordinates to check for intersections.
//!
//! First we build two maps, one vertical and one horizontal, of each line segment for the first
//! wire. Then we trace the steps of the second wire, looking for any intersections. We calculate
//! both part one and part two at the same time, by also including the distance so far
//! from the starting point of each lines.
//!
//! [`range`]: BTreeMap::range
use crate::util::parse::*;
use crate::util::point::*;
use std::collections::BTreeMap;

type Input = (i32, i32);

struct Line {
    start: Point,
    end: Point,
    distance: i32,
}

pub fn parse(input: &str) -> Input {
    // Map a line into an iterator of direction and distance pairs.
    let lines: Vec<_> = input.lines().collect();
    let steps = |i: usize| {
        let first = lines[i].bytes().filter(u8::is_ascii_alphabetic);
        let second = lines[i].iter_signed::<i32>();
        first.zip(second)
    };

    // Build two maps, one for vertical segments and one for horizontal.
    let mut start = ORIGIN;
    let mut distance = 0;
    let mut vertical = BTreeMap::new();
    let mut horizontal = BTreeMap::new();

    for (direction, amount) in steps(0) {
        let delta = Point::from(direction);
        let end = start + delta * amount;
        let line = Line { start, end, distance };

        if start.x == end.x {
            vertical.insert(start.x, line);
        } else {
            horizontal.insert(start.y, line);
        }

        start = end;
        distance += amount;
    }

    // Trace the steps of the second wire, checking for intersections.
    let mut start = ORIGIN;
    let mut distance = 0;
    let mut manhattan = i32::MAX;
    let mut delay = i32::MAX;

    for (direction, amount) in steps(1) {
        let delta = Point::from(direction);
        let end = start + delta * amount;

        // Use a block to scope the `update` lamdbas mutable borrow of `distance`.
        {
            // Checks for intersections, ignoring the initial intersection at the origin.
            let mut update = |line: &Line, candidate: Point| {
                if candidate.manhattan(line.start) < line.end.manhattan(line.start)
                    && candidate.signum(line.start) == line.end.signum(line.start)
                    && candidate.manhattan(ORIGIN) > 0
                {
                    manhattan = manhattan.min(candidate.manhattan(ORIGIN));
                    delay = delay.min(
                        distance
                            + candidate.manhattan(start)
                            + line.distance
                            + candidate.manhattan(line.start),
                    );
                }
            };

            // BTreeMaps are sorted and can return all key/value pairs in a range.
            match direction {
                b'U' => {
                    for (&y, line) in horizontal.range(end.y..=start.y) {
                        update(line, Point::new(start.x, y));
                    }
                }
                b'D' => {
                    for (&y, line) in horizontal.range(start.y..=end.y) {
                        update(line, Point::new(start.x, y));
                    }
                }
                b'L' => {
                    for (&x, line) in vertical.range(end.x..=start.x) {
                        update(line, Point::new(x, start.y));
                    }
                }
                b'R' => {
                    for (&x, line) in vertical.range(start.x..=end.x) {
                        update(line, Point::new(x, start.y));
                    }
                }
                _ => unreachable!(),
            }
        }

        start = end;
        distance += amount;
    }

    (manhattan, delay)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}
