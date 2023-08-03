//! # Rope Bridge
//!
//! This solution relies on two of our utility classes. Two dimensional problems are common in AoC,
//! so having a decent [`Point`] (or `Coord` or `Pos`) class in your back pocket is handy.
//!
//! The second utility class is [`FastSet`]. By default, Rust's [`HashMap`] and [`HashSet`] use
//! a [DDoS](https://en.wikipedia.org/wiki/Denial-of-service_attack) resistant but slower hashing
//! algorithm. The [`FastSet`] class replaces this with a much faster (between 2x to 5x from my testing)
//! algorithm, used by both the [RustC compiler](https://github.com/rust-lang/rustc-hash) and also
//! used by [Firefox](https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html).
//!
//! [`HashSet`]: std::collections::HashSet
//! [`HashMap`]: std::collections::HashMap
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;

type Input = (Point, u32);

/// Converts input lines into a pair of [`Point`] and integer amount, to indicate direction and
/// magnitude respectively.
pub fn parse(input: &str) -> Vec<Input> {
    input
        .split_ascii_whitespace()
        .chunk::<2>()
        .map(|[d, n]| {
            let point = Point::from_string(d);
            let amount = n.unsigned();
            (point, amount)
        })
        .collect()
}

/// Simulate a rope length of 2
pub fn part1(input: &[Input]) -> usize {
    simulate::<2>(input)
}

/// Simulate a rope length of 10
pub fn part2(input: &[Input]) -> usize {
    simulate::<10>(input)
}

/// Simulates a rope of arbitrary length.
///
/// The head knot always moves according the instructions from the problem input. Remaining knots
/// move according to their delta from the head (2nd knot) or the previous knot
/// (3rd and subsequent knots). A `FastSet` stores unique points visited by the tail.
///
/// Using const generics for the rope length allows the compiler to optimize the loop and speeds
/// things up by about 40%.
fn simulate<const N: usize>(input: &[Input]) -> usize {
    let mut rope: Vec<Point> = vec![ORIGIN; N];
    let mut tail: FastSet<Point> = FastSetBuilder::with_capacity(5_000);

    for (step, amount) in input {
        for _ in 0..*amount {
            rope[0] += *step;
            for i in 1..N {
                if apart(rope[i - 1], rope[i]) {
                    let next = delta(rope[i - 1], rope[i]);
                    rope[i] += next;
                }
            }
            tail.insert(rope[N - 1]);
        }
    }

    tail.len()
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
