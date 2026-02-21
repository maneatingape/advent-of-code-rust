//! # No Time for a Taxicab
//!
//! The solution is short as it leverages two utility modules,
//! [`parse`] for extracting integers from surrounding text and [`point`] for two dimensional
//! rotations and translations.
//!
//! For part two, it is faster to remember line segments and check for intersections than
//! to store information on every intermediate point visited.
//!
//! [`parse`]: crate::util::parse
//! [`point`]: crate::util::point
use crate::util::integer::*;
use crate::util::parse::*;
use crate::util::point::*;

type Pair = (u8, i32);

// Represent the line segment between two points.
struct Segment {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Segment {
    fn new(first: Point, second: Point) -> Self {
        let (x1, x2) = first.x.minmax(second.x);
        let (y1, y2) = first.y.minmax(second.y);
        Segment { x1, x2, y1, y2 }
    }

    // Return the point of intersection between two orthogonal segments, if there is one.
    fn intersects(&self, other: &Segment) -> Option<Point> {
        let overlap =
            !(other.x2 < self.x1 || other.x1 > self.x2 || other.y2 < self.y1 || other.y1 > self.y2);
        overlap.then_some(Point::new(self.x1.max(other.x1), self.y1.max(other.y1)))
    }
}

pub fn parse(input: &str) -> Vec<Pair> {
    let first = input.bytes().filter(u8::is_ascii_uppercase);
    let second = input.iter_signed();
    first.zip(second).collect()
}

pub fn part1(input: &[Pair]) -> i32 {
    let mut position = ORIGIN;
    let mut direction = UP;

    for &(turn, amount) in input {
        direction =
            if turn == b'L' { direction.counter_clockwise() } else { direction.clockwise() };
        position += direction * amount;
    }

    position.manhattan(ORIGIN)
}

pub fn part2(input: &[Pair]) -> i32 {
    let mut position = ORIGIN;
    let mut direction = UP;
    // Store two lists of segments, one for each orthogonal direction.
    let mut this_axis = Vec::with_capacity(input.len() / 2);
    let mut other_axis = Vec::with_capacity(input.len() / 2);

    for &(turn, amount) in input {
        direction =
            if turn == b'L' { direction.counter_clockwise() } else { direction.clockwise() };
        let target = position + direction * amount;
        // Exclude the current location from the next segment to track.
        let segment = Segment::new(position + direction, target);

        for other in &other_axis {
            if let Some(point) = segment.intersects(other) {
                return point.manhattan(ORIGIN);
            }
        }
        this_axis.push(segment);
        (this_axis, other_axis, position) = (other_axis, this_axis, target);
    }

    unreachable!()
}
