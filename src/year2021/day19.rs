use crate::util::chunk::*;
use crate::util::parse::*;
use std::collections::HashSet;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point3D([i32; 3]);

impl Point3D {
    fn parse(line: &&str) -> Point3D {
        let mut iter = line.iter_signed().chunk::<3>();
        Point3D(iter.next().unwrap())
    }

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

    fn manhattan(&self, other: &Point3D) -> i32 {
        let [x1, y1, z1] = self.0;
        let [x2, y2, z2] = other.0;
        (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
    }
}

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

pub struct Scanner {
    beacons: HashSet<Point3D>,
    deltas: HashSet<Point3D>,
    offset: Point3D,
}

impl Scanner {
    fn parse(lines: &[&str]) -> Scanner {
        let beacons: HashSet<_> = lines.iter().skip(1).map(Point3D::parse).collect();

        let mut deltas = HashSet::with_capacity(1_000);
        for (i, a) in beacons.iter().enumerate() {
            for (j, b) in beacons.iter().enumerate() {
                if i != j {
                    deltas.insert(*a - *b);
                }
            }
        }

        let offset = Point3D([0, 0, 0]);
        Scanner { beacons, deltas, offset }
    }

    fn rotations(&self) -> Vec<Scanner> {
        let mut scanners = Vec::new();
        for _ in 0..24 {
            scanners.push(Scanner {
                beacons: HashSet::with_capacity(30),
                deltas: HashSet::with_capacity(1_000),
                offset: Point3D([0, 0, 0]),
            });
        }

        for point in self.beacons.iter() {
            for (i, rotation) in point.rotations().into_iter().enumerate() {
                scanners[i].beacons.insert(rotation);
            }
        }

        for point in self.deltas.iter() {
            for (i, rotation) in point.rotations().into_iter().enumerate() {
                scanners[i].deltas.insert(rotation);
            }
        }

        scanners
    }

    fn possible_overlap(&self, other: &Scanner) -> bool {
        self.deltas.intersection(&other.deltas).count() >= 12 * 11
    }

    fn definite_overlap(&self, other: &Scanner) -> Option<Scanner> {
        for first in self.beacons.iter() {
            for second in other.beacons.iter() {
                let offset = *first - *second;
                let candidate: HashSet<_> = other.beacons.iter().map(|&p| p + offset).collect();
                if candidate.intersection(&self.beacons).count() >= 12 {
                    let located = Scanner {
                        beacons: candidate,
                        deltas: other.deltas.clone(),
                        offset,
                    };
                    return Some(located);
                }
            }
        }
        None
    }
}

pub fn parse(input: &str) -> Vec<Scanner> {
    let lines: Vec<_> = input.lines().collect();

    let mut scanners: Vec<_> = lines
        .split(|line| line.is_empty())
        .map(Scanner::parse)
        .collect();

    locate(&mut scanners)
}

pub fn part1(input: &[Scanner]) -> usize {
    let mut result = HashSet::with_capacity(1_000);

    for scanner in input.iter() {
        for beacon in scanner.beacons.iter() {
            result.insert(beacon);
        }
    }

    result.len()
}

pub fn part2(input: &[Scanner]) -> i32 {
    let mut result = 0;

    for first in input.iter() {
        for second in input.iter() {
            result = result.max(first.offset.manhattan(&second.offset));
        }
    }

    result
}

fn locate(scanners: &mut Vec<Scanner>) -> Vec<Scanner> {
    let mut done = Vec::new();
    let mut todo = Vec::from([scanners.pop().unwrap()]);
    let mut unknown: Vec<_> = scanners.iter().map(|s| s.rotations()).collect();

    while let Some(known) = todo.pop() {
        let mut next_unknown = Vec::new();

        while let Some(candidates) = unknown.pop() {
            if let Some(located) = check(&known, &candidates) {
                todo.push(located);
            } else {
                next_unknown.push(candidates);
            }
        }

        done.push(known);
        unknown = next_unknown;
    }

    done
}

fn check(known: &Scanner, candidates: &[Scanner]) -> Option<Scanner> {
    for rotation in candidates.iter() {
        if known.possible_overlap(rotation) {
            if let Some(located) = known.definite_overlap(rotation) {
                return Some(located);
            }
        }
    }
    None
}
