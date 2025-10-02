//! # Beacon Scanner
//!
//! A brute force approach is:
//! * Choose an arbitrary starting scanner, then add its beacons to a "known" set.
//! * For each remaining scanner, then for each of its possible 24 rotations, check its beacons
//!   by translating against every other beacon in the known set.
//! * If we find a match of 12 or more overlapping beacons, then merge the beacons into the known
//!   set.
//!
//! This approach will work but is a little slow as the number of potential comparisons is quite
//! high. We can speed things up by first creating a "signature" for each beacon similar to how
//! a hash is computed for an item in a hash map. Ideally this signature should be the same no
//! matter what the rotation of the beacons, as this will reduce the number of comparisons by a
//! factor of 24.
//!
//! The set of Euclidean distance squared between all beacons is a good choice, as it's invariant
//! under rotation and translation, quick to calculate and a good discriminant. To check for an
//! overlap of 12 beacons, we look for an overlap of a least 12 * 11 / 2 = 66 distances.
//! (12 beacons gives 12 * 11 = 132 pairs of distances but divided by 2 since the distance from
//! a -> b is the same as b -> a).
//!
//! An overlap indicates a potential match, but we need to confirm by checking the beacons against
//! each other in two steps. First confirming orientation by matching the deltas between
//! points, then by translating the beacons until 12 overlap.
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;
use std::ops::{Add, Sub};

/// Stores coordinates in x, y, z order.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Point3D(i32, i32, i32);

impl Point3D {
    fn parse([x, y, z]: [i32; 3]) -> Point3D {
        Point3D(x, y, z)
    }

    /// There are 24 possible 3D rotations of each point in increments of 90 degrees.
    fn transform(&self, index: usize) -> Point3D {
        let Point3D(x, y, z) = *self;
        match index {
            0 => Point3D(x, y, z),
            1 => Point3D(x, z, -y),
            2 => Point3D(x, -z, y),
            3 => Point3D(x, -y, -z),
            4 => Point3D(-x, -z, -y),
            5 => Point3D(-x, y, -z),
            6 => Point3D(-x, -y, z),
            7 => Point3D(-x, z, y),
            8 => Point3D(y, z, x),
            9 => Point3D(y, -x, z),
            10 => Point3D(y, x, -z),
            11 => Point3D(y, -z, -x),
            12 => Point3D(-y, x, z),
            13 => Point3D(-y, z, -x),
            14 => Point3D(-y, -z, x),
            15 => Point3D(-y, -x, -z),
            16 => Point3D(z, x, y),
            17 => Point3D(z, y, -x),
            18 => Point3D(z, -y, x),
            19 => Point3D(z, -x, -y),
            20 => Point3D(-z, y, x),
            21 => Point3D(-z, -x, y),
            22 => Point3D(-z, x, -y),
            23 => Point3D(-z, -y, -x),
            _ => unreachable!(),
        }
    }

    /// No need to take the square root as it's faster and easier to just use the integer
    /// value of the distance squared directly.
    fn euclidean(&self, other: &Point3D) -> i32 {
        let Point3D(dx, dy, dz) = *self - *other;
        dx * dx + dy * dy + dz * dz
    }

    fn manhattan(&self, other: &Point3D) -> i32 {
        let Point3D(dx, dy, dz) = *self - *other;
        dx.abs() + dy.abs() + dz.abs()
    }
}

/// Implement operators for points so that we can write `a + b` or `a - b`.
impl Add for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Point3D {
        let Point3D(x1, y1, z1) = self;
        let Point3D(x2, y2, z2) = rhs;
        Point3D(x1 + x2, y1 + y2, z1 + z2)
    }
}

impl Sub for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Point3D) -> Point3D {
        let Point3D(x1, y1, z1) = self;
        let Point3D(x2, y2, z2) = rhs;
        Point3D(x1 - x2, y1 - y2, z1 - z2)
    }
}

/// Represents an unknown scanner that could be at any orientation and translation
/// from our initial reference scanner.
struct Scanner {
    beacons: Vec<Point3D>,
    signature: FastMap<i32, [usize; 2]>,
}

impl Scanner {
    /// Calculate the signature as the set of Euclidean distance squared between every possible
    /// pair of beacons.
    fn parse(block: &str) -> Scanner {
        // Each beacon header results in 5 mangled numbers at the start that should be skipped.
        let beacons: Vec<_> =
            block.iter_signed().skip(5).chunk::<3>().map(Point3D::parse).collect();

        // Include indices of the points so that we can match translation and rotation for
        // points that have the same signature. Use indices so that we don't need to recalculate
        // signature when rotating and translation a beacon from unknown to known.
        let mut signature = FastMap::with_capacity(1_000);
        for i in 0..(beacons.len() - 1) {
            for j in (i + 1)..beacons.len() {
                let key = beacons[i].euclidean(&beacons[j]);
                let value = [i, j];
                signature.insert(key, value);
            }
        }

        Scanner { beacons, signature }
    }
}

/// Returns the correct orientation and translation to link a new scanner to an existing
/// reference scanner.
#[derive(Clone, Copy)]
struct Found {
    orientation: usize,
    translation: Point3D,
}

/// Represents a known scanner with the same orientation and a known translation from
/// our initial reference scanner.
pub struct Located {
    beacons: Vec<Point3D>,
    signature: FastMap<i32, [usize; 2]>,
    oriented: FastSet<Point3D>,
    translation: Point3D,
}

impl Located {
    fn from(scanner: Scanner, found: Found) -> Located {
        let Scanner { beacons, signature } = scanner;
        let Found { orientation, translation } = found;

        // Rotate and translate the beacons by the offset of this scanner from the reference, so
        // that we can build "chains" of scanners, for example A -> B -> C, where A and B overlap,
        // B and C overlap, but not A and C.
        let beacons: Vec<_> =
            beacons.iter().map(|b| b.transform(orientation) + translation).collect();
        let oriented = beacons.iter().copied().collect();

        Located { beacons, signature, oriented, translation }
    }
}

/// Convert the raw input into a vec of unknown scanners, then do all the heavy lifting of figuring
/// out the relative orientations and translations of each scanner.
///
/// First choose an arbitrary scanner that determines the reference orientation and that we
/// decide is located at the origin.
///
/// Then for each remaining unknown scanner, check if the signature indicates a potential
/// match. If confirmed, we determine the orientation and translation then add the scanner
/// to a todo list to recheck against other unknown scanners.
///
/// This works for situations such as A -> B -> C, where A and B overlap, B and C overlap, but not
/// A and C.
pub fn parse(input: &str) -> Vec<Located> {
    let mut unknown: Vec<_> = input.split("\n\n").map(Scanner::parse).collect();
    let mut todo = Vec::new();
    let mut done = Vec::new();

    let scanner = unknown.pop().unwrap();
    let found = Found { orientation: 0, translation: Point3D(0, 0, 0) };
    todo.push(Located::from(scanner, found));

    while let Some(known) = todo.pop() {
        let mut next_unknown = Vec::new();

        while let Some(scanner) = unknown.pop() {
            match check(&known, &scanner) {
                Some(found) => todo.push(Located::from(scanner, found)),
                None => next_unknown.push(scanner),
            }
        }

        done.push(known);
        unknown = next_unknown;
    }

    done
}

/// Calculate the total number of distinct beacons.
pub fn part1(input: &[Located]) -> usize {
    let mut result = FastSet::with_capacity(1_000);

    for located in input {
        for beacon in &located.beacons {
            result.insert(beacon);
        }
    }

    result.len()
}

/// Calculate the maximum manhattan distance between any two scanners.
pub fn part2(input: &[Located]) -> i32 {
    let mut result = 0;

    for first in input {
        for second in input {
            result = result.max(first.translation.manhattan(&second.translation));
        }
    }

    result
}

/// At least 66 Euclidean distances must overlap for a potential match.
fn check(known: &Located, scanner: &Scanner) -> Option<Found> {
    let mut matching = 0;

    for key in known.signature.keys() {
        if scanner.signature.contains_key(key) {
            matching += 1;
            if matching == 66 {
                // Choose any arbitrary pair of points that have a matching signature.
                let [a, b] = known.signature[key];
                let [x, y] = scanner.signature[key];
                let points =
                    [known.beacons[a], known.beacons[b], scanner.beacons[x], scanner.beacons[y]];
                return detailed_check(known, scanner, points);
            }
        }
    }

    None
}

/// The correct translation and orientation is found when we have at least 12 beacons overlapping.
fn detailed_check(known: &Located, scanner: &Scanner, points: [Point3D; 4]) -> Option<Found> {
    let [a, b, x, y] = points;
    let delta = a - b;

    for orientation in 0..24 {
        let rotate_x = x.transform(orientation);
        let rotate_y = y.transform(orientation);

        let translation = if rotate_x - rotate_y == delta {
            b - rotate_y
        } else if rotate_y - rotate_x == delta {
            b - rotate_x
        } else {
            continue;
        };

        let mut count = 0;

        for candidate in &scanner.beacons {
            let point = candidate.transform(orientation) + translation;

            if known.oriented.contains(&point) {
                count += 1;
                if count == 12 {
                    return Some(Found { orientation, translation });
                }
            }
        }
    }

    None
}
