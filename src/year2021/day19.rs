//! # Beacon Scanner
//!
//! A brute force approach is:
//! * Choose an arbitrary starting scanner, then add its beacons to a "known" set.
//! * For each remaining scanner, then for each of its possible 24 rotations, check its beacons
//!   by translating against every other beacon in the known set.
//! * If we find a match of 12 or more overlapping beacons, then merge the beacons into the known
//!   set.
//!
//! This approach will work but is a little slow as the number of potential comparisions is quite
//! high. We can speed things up by first creating a "signature" for each beacon similar to how
//! a hash is computed for an item in a hash map. Ideally this signature should be the same no
//! matter what the rotation of the beacons, as this will reduce the number of comparisons by a
//! factor of 24.
//!
//! The set of euclidean distance squared between all beacons is a good choice, as it's invariant
//! under rotation, quick to calculate and a good discriminant. To check for an overlap of 12
//! beacons, we look for an overlap of a least 12 * 11 / 2 = 66 distances. (12 pairs is 12 * 11
//! different distances but divided by 2 since the distance from a -> b is the same as b -> a).
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
pub struct Point3D([i32; 3]);

impl Point3D {
    fn parse(line: &&str) -> Point3D {
        let mut iter = line.iter_signed().chunk::<3>();
        Point3D(iter.next().unwrap())
    }

    /// There are 24 possible 3D rotations of each point in increments of 90 degrees.
    fn rotations(&self) -> [Point3D; 24] {
        let [x, y, z] = self.0;
        [
            Point3D([x, y, z]),
            Point3D([x, z, -y]),
            Point3D([x, -z, y]),
            Point3D([x, -y, -z]),
            Point3D([-x, -z, -y]),
            Point3D([-x, y, -z]),
            Point3D([-x, -y, z]),
            Point3D([-x, z, y]),
            Point3D([y, z, x]),
            Point3D([y, -x, z]),
            Point3D([y, x, -z]),
            Point3D([y, -z, -x]),
            Point3D([-y, x, z]),
            Point3D([-y, z, -x]),
            Point3D([-y, -z, x]),
            Point3D([-y, -x, -z]),
            Point3D([z, x, y]),
            Point3D([z, y, -x]),
            Point3D([z, -y, x]),
            Point3D([z, -x, -y]),
            Point3D([-z, y, x]),
            Point3D([-z, -x, y]),
            Point3D([-z, x, -y]),
            Point3D([-z, -y, -x]),
        ]
    }

    /// No need to take the square root as it's faster and easier to just use the integer
    /// value of the distance squared directly.
    fn euclidean(&self, other: &Point3D) -> i32 {
        let [dx, dy, dz] = (*self - *other).0;
        dx * dx + dy * dy + dz * dz
    }

    fn manhattan(&self, other: &Point3D) -> i32 {
        let [dx, dy, dz] = (*self - *other).0;
        dx.abs() + dy.abs() + dz.abs()
    }
}

/// Implement operators for points so that we can write `a + b` or `a - b`.
impl Add for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Point3D {
        let [x1, y1, z1] = self.0;
        let [x2, y2, z2] = rhs.0;
        Point3D([x1 + x2, y1 + y2, z1 + z2])
    }
}

impl Sub for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Point3D) -> Point3D {
        let [x1, y1, z1] = self.0;
        let [x2, y2, z2] = rhs.0;
        Point3D([x1 - x2, y1 - y2, z1 - z2])
    }
}

/// Represents an unknown scanner that could at any orientation and an unknown translation
/// from our initial reference scanner.
pub struct Scanner {
    beacons: Vec<Point3D>,
    signature: FastSet<i32>,
}

impl Scanner {
    /// Calculate the signature as the set of euclidean distance squared between every possible
    /// pair of beacons.
    fn parse(lines: &[&str]) -> Scanner {
        let beacons: Vec<_> = lines.iter().skip(1).map(Point3D::parse).collect();

        let mut signature = FastSet::with_capacity(1_000);
        for i in 0..(beacons.len() - 1) {
            for j in (i + 1)..beacons.len() {
                signature.insert(beacons[i].euclidean(&beacons[j]));
            }
        }

        Scanner { beacons, signature }
    }
}

/// Represents a known scanner with the same orientation and a known translation from
/// our initial reference scanner.
pub struct Located {
    signature: FastSet<i32>,
    deltas: FastSet<Point3D>,
    beacons: FastSet<Point3D>,
    offset: Point3D,
}

impl Located {
    /// Delta are *not* invariant under rotation, so we can use them to determine the correct
    /// orientation of other scanners relative to this one.
    fn from(relative_beacons: &[Point3D], signature: FastSet<i32>, offset: Point3D) -> Located {
        let mut deltas = FastSet::with_capacity(1_000);
        for (i, a) in relative_beacons.iter().enumerate() {
            for (j, b) in relative_beacons.iter().enumerate() {
                if i != j {
                    deltas.insert(*a - *b);
                }
            }
        }

        // Translate the beacons by the offset of this scanner from the reference, so that
        // we can build "chains" of scanners, for example A -> B -> C, where A and B overlap,
        // B and C overlap, but not A and C.
        let mut beacons = FastSet::with_capacity(30);
        for &point in relative_beacons {
            beacons.insert(point + offset);
        }

        Located { signature, deltas, beacons, offset }
    }
}

/// Convert the raw input into a vec of unkown scanners, then do all the heavy lifting of figuring
/// out the relative orientations and translations of each scanner.
pub fn parse(input: &str) -> Vec<Located> {
    let lines: Vec<_> = input.lines().collect();
    let mut scanners: Vec<_> = lines.split(|line| line.is_empty()).map(Scanner::parse).collect();
    locate(&mut scanners)
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
            result = result.max(first.offset.manhattan(&second.offset));
        }
    }

    result
}

/// First choose an arbitrary scanner that determines the reference orientation and that we
/// decide is located at the origin.
///
/// Then for each remaining unknown scanner, check if the signature indicates a potential
/// match. If confirmed, we determine the orientation and translation then add the scanner
/// to a todo list to recheck against other unknown scanners.
///
/// This works for situations such as A -> B -> C, where A and B overlap, B and C overlap, but not
/// A and C.
fn locate(unknown: &mut Vec<Scanner>) -> Vec<Located> {
    let mut done = Vec::new();
    let mut todo = Vec::new();

    let Scanner { beacons, signature } = unknown.pop().unwrap();
    todo.push(Located::from(&beacons, signature, Point3D([0, 0, 0])));

    while let Some(known) = todo.pop() {
        let mut next_unknown = Vec::new();

        while let Some(scanner) = unknown.pop() {
            if let Some(located) = check(&known, &scanner) {
                todo.push(located);
            } else {
                next_unknown.push(scanner);
            }
        }

        done.push(known);
        *unknown = next_unknown;
    }

    done
}

fn check(known: &Located, scanner: &Scanner) -> Option<Located> {
    // At least 66 euclidean distances must overlap
    let matching: FastSet<_> = known.signature.intersection(&scanner.signature).copied().collect();
    if matching.len() < 66 {
        return None;
    }

    // We only need to double check beacons that form part of the matching signature.
    let mut beacons_of_interest = FastSet::new();
    for i in 0..(scanner.beacons.len() - 1) {
        for j in (i + 1)..scanner.beacons.len() {
            if matching.contains(&scanner.beacons[i].euclidean(&scanner.beacons[j])) {
                beacons_of_interest.insert(scanner.beacons[i]);
                beacons_of_interest.insert(scanner.beacons[j]);
            }
        }
    }

    // The confirmation takes place in two parts. First we confirm that the rotated beacons of
    // interest can be oriented the same way as our known scanner. Then we check that the beacons
    // can be translated so that at least 12 beacons overlap.
    let candidates: Vec<_> = beacons_of_interest.iter().map(Point3D::rotations).collect();

    for i in 0..24 {
        let next: Vec<_> = candidates.iter().map(|&rotations| rotations[i]).collect();
        if check_deltas(known, &next) {
            if let Some(offset) = check_offsets(known, &next) {
                let oriented: Vec<_> = scanner.beacons.iter().map(|p| p.rotations()[i]).collect();
                let located = Located::from(&oriented, scanner.signature.clone(), offset);
                return Some(located);
            }
        }
    }

    None
}

/// Check for at least 66 overlapping delta. Unlike the signature, deltas are *not* invariant
/// under rotation, so we can use them to determine the correct orientation of the unknown
/// scanner.
fn check_deltas(known: &Located, next: &[Point3D]) -> bool {
    let max_no = (next.len() * (next.len() - 1)) - (12 * 11);
    let mut no = 0;

    for (i, first) in next.iter().enumerate() {
        for (j, second) in next.iter().enumerate() {
            if i != j {
                let delta = *first - *second;
                if !known.deltas.contains(&delta) {
                    no += 1;
                    if no > max_no {
                        return false;
                    }
                }
            }
        }
    }

    true
}

/// Now that we know the correct orientation, try every possible combination of beacons pairs.
/// The correct translation offset is found when we have at least 12 beacons overlapping.
fn check_offsets(known: &Located, next: &[Point3D]) -> Option<Point3D> {
    for first in &known.beacons {
        for second in next {
            let offset = *first - *second;
            let mut candidates = FastSet::with_capacity(30);
            for &point in next {
                candidates.insert(point + offset);
            }

            if known.beacons.intersection(&candidates).count() >= 12 {
                return Some(offset);
            }
        }
    }

    None
}
