//! # Monitoring Station
//!
//! Integer only solution, avoiding floating point or trignometric lookups such as [`atan2`].
//!
//! ## Part One
//!
//! We compare each pair of points, first subtracting the current asteroid to get a relative vector.
//! Since all coordinates are integers we can check for multiple points on the same line by
//! reducing the vector by its [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor).
//! For example, looking from the origin `(0, 0)`, the points `(3, 5)`, `(6, 10)` and `(21, 35)`
//! are all on the same line, with gcds of 1, 2 and 7 respectively.
//!
//! For each point we build a set of previously seen values. Since we can see at most one asteroid
//! in a given direction, if a vector is already in the set then we ignore it. The final size of
//! the set is the number of visible asteroids.
//!
//! To speeds things up a little, we rely on the fact that if asteroid `a` can see `b`, then `b`
//! can see `a`. The solution is still `O(n²)` complexity but we reduce the work by half. This
//! works by implicitly processing asteroids in the same order as the input, from left to right and
//! top to bottom, so that nearest asteroids are always encountered first.
//!
//! ## Part Two
//!
//! The key insight is that we only need the *relative* angle between two vectors to sort them
//! in clockwise order. The [vector cross product](https://en.wikipedia.org/wiki/Cross_product)
//! of `a` and `b` can be defined as `|a||b|sinθ` where `θ` is the angle between the vectors.
//! `θ` will be negative if anti-clockwise, zero if on the same line or positive if clockwise.
//!
//! This works for angles up to 90°. To handle the complete 360° rotation, we first order points
//! by [quadrant](https://en.wikipedia.org/wiki/Quadrant_(plane_geometry)) then by relative angle.
//!
//! Finally we also order points from nearest to furthest, so that the total ordering is:
//! 1. Quadrant
//! 2. Clockwise angle
//! 3. Distance
//!
//! This results in a something like (where letters indicate angle and numbers indicate distance):
//!
//! `a1 a2 a3 b1 c1 c2 c3 c4 d1 d2`
//!
//! We want to process asteroids in the order:
//!
//! `a1 b1 c1 d1 a2 c2 d2 a3 c3 c4`
//!
//! We do this by first numbering the position within each group, then numbering the group and
//! sorting a second time in this order.
//!
//! [`atan2`]: f64::atan2
use crate::util::hash::*;
use crate::util::math::*;
use crate::util::point::*;
use std::cmp::Ordering;

type Input = (i32, i32);

pub fn parse(input: &str) -> Input {
    // Convert asteroids to `Point` objects.
    let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
    let mut points = Vec::new();

    for (y, row) in raw.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            if col == b'#' {
                points.push(Point::new(x as i32, y as i32));
            }
        }
    }

    // Find asteroid with the highest visibility.
    let mut visible = vec![0; points.len()];
    let mut seen = FastSet::new();
    let mut max_visible = 0;
    let mut max_index = 0;

    for i in 0..(points.len() - 1) {
        for j in (i + 1)..points.len() {
            let mut delta = points[j] - points[i];

            // Key insight is that points on the same line are integer multiples of each other.
            let factor = delta.x.gcd(delta.y).abs();
            delta.x /= factor;
            delta.y /= factor;

            // This works as the points are in order from left to right and top to bottom,
            // so we process points from nearest to furthest.
            if seen.insert(delta) {
                visible[i] += 1;
                visible[j] += 1;
            }
        }

        if visible[i] > max_visible {
            max_visible = visible[i];
            max_index = i;
        }

        seen.clear();
    }

    // Remove our new base of operations, then sort remaining asteroids in clockwise order to
    // group by angle.
    let station = points.swap_remove(max_index);
    points.iter_mut().for_each(|p| *p -= station);
    points.sort_unstable_by(|&a, &b| clockwise(a, b));

    // Sort asteroids a second time, first by order within the group, then the group's order.
    let mut groups = Vec::with_capacity(points.len());
    let mut first = 0;
    let mut second = 0;

    groups.push((first, second, 0));

    for i in 1..points.len() {
        if angle(points[i], points[i - 1]) == Ordering::Greater {
            first = 0;
            second += 1;
        } else {
            first += 1;
        }
        groups.push((first, second, i));
    }

    groups.sort_unstable();

    // The 200th asteroid is at index 199.
    let target = station + points[groups[199].2];
    let result = 100 * target.x + target.y;

    (max_visible, result)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}

/// The [`then`] method chains [`Ordering`] results.
///
/// [`then`]: Ordering::then
fn clockwise(point: Point, other: Point) -> Ordering {
    quadrant(point)
        .cmp(&quadrant(other))
        .then(angle(point, other))
        .then(distance(point).cmp(&distance(other)))
}

/// Divide points into one of four quadrants. For points exactly on an axis, for example (1, 0)
/// or (-5, 0) we can choose either adjacent quadrant as long as we're consistent.
fn quadrant(point: Point) -> i32 {
    match (point.x >= 0, point.y >= 0) {
        (true, false) => 0,
        (true, true) => 1,
        (false, true) => 2,
        (false, false) => 3,
    }
}

/// Positive if clockwise, zero if on the same line, negative if anti-clockwise.
fn angle(point: Point, other: Point) -> Ordering {
    (other.x * point.y).cmp(&(other.y * point.x))
}

/// Euclidean distance squared. No need to take square root since we're only interested
/// in the *relative* distance.
fn distance(point: Point) -> i32 {
    point.x * point.x + point.y * point.y
}
