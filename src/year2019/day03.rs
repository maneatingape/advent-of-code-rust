//! # Crossed Wires
//!
//! The input follows some implicit rules that can be used to simplify our approach:
//!
//! * Wires cross only at right angles to each other, so we only need to consider horizontal lines
//!   when moving vertically and vice-versa.
//! * There is only a single vertical line at a given x coordinate and vice-versa.
//!
//! This makes [`BTreeMap`] a great choice to store horizontal or vertical line segments as there
//! are no collisions. The [`range`] method can lookup all line segments contained between two
//! coordinates to check for intersections.
//!
//! First we build two maps, one vertical and one horizontal, of each line segment for the first
//! wire. Then we trace the steps of the second wire, looking for any intersections. We calculate
//! both part one and part two at the same time, by also including the distance so far
//! from the starting point of each line.
//!
//! [`range`]: BTreeMap::range
use std::collections::BTreeMap;

use crate::util::integer::*;
use crate::util::parse::*;
use crate::util::point::*;

type Input = (i32, i32);

struct Line {
    start: Point,
    end: Point,
    distance: i32,
}

pub fn parse(input: &str) -> Input {
    let (first, second) = input.split_once('\n').unwrap();

    // Build two maps, one for vertical segments and one for horizontal.
    let mut vertical = BTreeMap::new();
    let mut horizontal = BTreeMap::new();

    for line in lines(first) {
        if line.start.x == line.end.x {
            vertical.insert(line.start.x, line);
        } else {
            horizontal.insert(line.start.y, line);
        }
    }

    // Trace the steps of the second wire, checking for intersections.
    let mut manhattan = i32::MAX;
    let mut delay = i32::MAX;

    for Line { start, end, distance } in lines(second) {
        // Checks for intersections, ignoring the initial intersection at the origin.
        let mut update = |line: &Line, candidate: Point| {
            if candidate.manhattan(line.start) < line.end.manhattan(line.start)
                && signum(candidate, line.start) == signum(line.end, line.start)
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
        if start.x == end.x {
            let (lo, hi) = start.y.minmax(end.y);
            for (&y, line) in horizontal.range(lo..=hi) {
                update(line, Point::new(start.x, y));
            }
        } else {
            let (lo, hi) = start.x.minmax(end.x);
            for (&x, line) in vertical.range(lo..=hi) {
                update(line, Point::new(x, start.y));
            }
        }
    }

    (manhattan, delay)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}

/// Map a wire into an iterator of line segments, each with the distance so far to its start.
fn lines(wire: &str) -> impl Iterator<Item = Line> {
    let directions = wire.bytes().filter(u8::is_ascii_alphabetic);
    let amounts = wire.iter_signed();
    let mut start = ORIGIN;
    let mut distance = 0;

    directions.zip(amounts).map(move |(direction, amount)| {
        let line = Line { start, end: start + Point::from(direction) * amount, distance };
        start = line.end;
        distance += amount;
        line
    })
}

fn signum(a: Point, b: Point) -> Point {
    Point::new((a.x - b.x).signum(), (a.y - b.y).signum())
}
