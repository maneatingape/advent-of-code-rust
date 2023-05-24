use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;
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

    fn euclidean(&self, other: &Point3D) -> i32 {
        let [dx, dy, dz] = (*self - *other).0;
        dx * dx + dy * dy + dz * dz
    }

    fn manhattan(&self, other: &Point3D) -> i32 {
        let [dx, dy, dz] = (*self - *other).0;
        dx.abs() + dy.abs() + dz.abs()
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
    beacons: Vec<Point3D>,
    signature: FastSet<i32>,
}

impl Scanner {
    fn parse(lines: &[&str]) -> Scanner {
        let beacons: Vec<_> = lines.iter().skip(1).map(Point3D::parse).collect();

        let mut signature: FastSet<_> = FastSetBuilder::with_capacity(1_000);
        for i in 0..(beacons.len() - 1) {
            for j in (i + 1)..beacons.len() {
                signature.insert(beacons[i].euclidean(&beacons[j]));
            }
        }

        Scanner { beacons, signature }
    }
}

pub struct Located {
    signature: FastSet<i32>,
    deltas: FastSet<Point3D>,
    beacons: FastSet<Point3D>,
    offset: Point3D,
}

impl Located {
    fn from(relative_beacons: Vec<Point3D>, signature: FastSet<i32>, offset: Point3D) -> Located {
        let mut deltas = FastSetBuilder::with_capacity(1_000);
        for (i, a) in relative_beacons.iter().enumerate() {
            for (j, b) in relative_beacons.iter().enumerate() {
                if i != j {
                    deltas.insert(*a - *b);
                }
            }
        }

        let mut beacons = FastSetBuilder::with_capacity(30);
        for &point in relative_beacons.iter() {
            beacons.insert(point + offset);
        }

        Located { signature, deltas, beacons, offset }
    }
}

pub fn parse(input: &str) -> Vec<Located> {
    let lines: Vec<_> = input.lines().collect();

    let mut scanners: Vec<_> = lines.split(|line| line.is_empty()).map(Scanner::parse).collect();

    locate(&mut scanners)
}

pub fn part1(input: &[Located]) -> usize {
    let mut result = FastSetBuilder::with_capacity(1_000);

    for located in input.iter() {
        for beacon in located.beacons.iter() {
            result.insert(beacon);
        }
    }

    result.len()
}

pub fn part2(input: &[Located]) -> i32 {
    let mut result = 0;

    for first in input.iter() {
        for second in input.iter() {
            result = result.max(first.offset.manhattan(&second.offset));
        }
    }

    result
}

fn locate(unknown: &mut Vec<Scanner>) -> Vec<Located> {
    let mut done = Vec::new();
    let mut todo = Vec::new();

    let Scanner { beacons, signature } = unknown.pop().unwrap();
    todo.push(Located::from(beacons, signature, Point3D([0, 0, 0])));

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
    let matching: FastSet<_> = known.signature.intersection(&scanner.signature).copied().collect();
    if matching.len() < 66 {
        return None;
    }

    let mut beacons_of_interest = FastSetBuilder::empty();
    for i in 0..(scanner.beacons.len() - 1) {
        for j in (i + 1)..scanner.beacons.len() {
            if matching.contains(&scanner.beacons[i].euclidean(&scanner.beacons[j])) {
                beacons_of_interest.insert(scanner.beacons[i]);
                beacons_of_interest.insert(scanner.beacons[j]);
            }
        }
    }

    let candidates: Vec<_> = beacons_of_interest.iter().map(|p| p.rotations()).collect();

    for i in 0..24 {
        let next: Vec<_> = candidates.iter().map(|&rotations| rotations[i]).collect();
        if check_deltas(known, &next) {
            if let Some(offset) = check_offsets(known, &next) {
                let oriented: Vec<_> = scanner.beacons.iter().map(|p| p.rotations()[i]).collect();
                let located = Located::from(oriented, scanner.signature.clone(), offset);
                return Some(located);
            }
        }
    }

    None
}

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

fn check_offsets(known: &Located, next: &[Point3D]) -> Option<Point3D> {
    for first in known.beacons.iter() {
        for second in next.iter() {
            let offset = *first - *second;
            let mut candidates = FastSetBuilder::with_capacity(30);
            for &point in next.iter() {
                candidates.insert(point + offset);
            }

            if known.beacons.intersection(&candidates).count() >= 12 {
                return Some(offset);
            }
        }
    }

    None
}
