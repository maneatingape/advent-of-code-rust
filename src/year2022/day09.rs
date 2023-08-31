//! # Rope Bridge
//!
//! This solution relies on the [`Point`] utility class. Two dimensional problems are common in
//! Advent of Code, so having a decent `Point` (or `Coord` or `Pos`) class in your back pocket
//! is handy.
use crate::util::parse::*;
use crate::util::point::*;

type Pair = (Point, i32);
type Input = ([i32; 4], Vec<Pair>);

/// Converts input lines into a pair of [`Point`] and integer amount, to indicate direction and
/// magnitude respectively. Then determines the maximum extent of the head so that we can allocate
/// a two dimensional grid.
pub fn parse(input: &str) -> Input {
    let first = input.bytes().filter(u8::is_ascii_alphabetic).map(Point::from);
    let second = input.iter_signed::<i32>();
    let pairs = first.zip(second).collect();

    // Determine maximum extents
    let mut x1 = i32::MAX;
    let mut y1 = i32::MAX;
    let mut x2 = i32::MIN;
    let mut y2 = i32::MIN;
    let mut point = ORIGIN;

    for &(step, amount) in &pairs {
        point += step * amount;
        x1 = x1.min(point.x);
        y1 = y1.min(point.y);
        x2 = x2.max(point.x);
        y2 = y2.max(point.y);
    }

    ([x1, y1, x2, y2], pairs)
}

/// Simulate a rope length of 2
pub fn part1(input: &Input) -> u32 {
    simulate::<2>(input)
}

/// Simulate a rope length of 10
pub fn part2(input: &Input) -> u32 {
    simulate::<10>(input)
}

/// Simulates a rope of arbitrary length.
///
/// The head knot always moves according the instructions from the problem input. Remaining knots
/// move according to their delta from the head (2nd knot) or the previous knot
/// (3rd and subsequent knots).
///
/// Using const generics for the rope length allows the compiler to optimize the loop and speeds
/// things up by about 40%.
fn simulate<const N: usize>(input: &Input) -> u32 {
    let ([x1, y1, x2, y2], pairs) = input;
    let width = x2 - x1 + 1;
    let height = y2 - y1 + 1;
    let start = Point { x: -x1, y: -y1 };

    let mut distinct = 0;
    let mut rope = [start; N];
    let mut grid = vec![false; (width * height) as usize];

    for &(step, amount) in pairs {
        for _ in 0..amount {
            rope[0] += step;
            for i in 1..N {
                if !apart(rope[i - 1], rope[i]) {
                    break;
                }
                let next = delta(rope[i - 1], rope[i]);
                rope[i] += next;
            }

            let tail = rope[N - 1];
            let index = (width * tail.y + tail.x) as usize;

            if !grid[index] {
                grid[index] = true;
                distinct += 1;
            }
        }
    }

    distinct
}

/// Two knots are considered "apart" if the they are not diagonally adjacent, that is the absolute
/// distance in either x or y axes is greater than 1.
#[inline]
fn apart(a: Point, b: Point) -> bool {
    (a.x - b.x).abs() > 1 || (a.y - b.y).abs() > 1
}

/// The [`signum`] function comes in handy to figure out the direction that knots should move.
///
/// [`signum`]: i32::signum
#[inline]
fn delta(a: Point, b: Point) -> Point {
    Point { x: (a.x - b.x).signum(), y: (a.y - b.y).signum() }
}
