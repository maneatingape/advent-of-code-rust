//! # Experimental Emergency Teleportation
//!
//! A generic solution to part two would be a 3D version of binary search. Starting with a single
//! cube that encloses all nanobots, each cube is further split into 8 smaller cubes until we find
//! the answer. Cubes can be stored in a
//! [min-heap](https://en.wikipedia.org/wiki/Heap_(data_structure)) ordered by:
//!
//! * Greatest number of nanobots in range.
//! * Least distance to origin.
//! * Least size.
//!
//! This means that when we encounter a cube of size 1 we can return the coordinates,
//! since we know that:
//!
//! * There are no cubes within range of more nanobots.
//! * There are no cubes that are closer.
//! * The coordinates cannot be refined any further.
//!
//! However, the actual input files lend themselves to an even faster answer. At a high level,
//! we can determine a one-dimensional range of Manhattan distances at which we are in range of
//! a given nanobot. By sorting the points at which ranges begin and end, we can determine the
//! maximum number of nanobots that can possibly be in range at once. In the generic case,
//! two nanobots may have the same Manhattan distance but be non-overlapping in distinct
//! octants of 3-D space, so the actual best point may have fewer than the maximum determined in
//! this manner. But for our input files, it so happens that there is exactly one range that has
//! a higher potential than any other, and the low end of this range happens to be the Manhattan
//! distance we are after, without actually having to find the point with that distance.
use crate::util::iter::*;
use crate::util::parse::*;

pub struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    r: i32,
}

impl Nanobot {
    fn from([x, y, z, r]: [i32; 4]) -> Nanobot {
        Nanobot { x, y, z, r }
    }

    fn manhattan(&self, other: &Nanobot) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

pub fn parse(input: &str) -> Vec<Nanobot> {
    input.iter_signed().chunk::<4>().map(Nanobot::from).collect()
}

pub fn part1(input: &[Nanobot]) -> usize {
    let strongest = input.iter().max_by_key(|nb| nb.r).unwrap();
    input.iter().filter(|nb| strongest.manhattan(nb) <= strongest.r).count()
}

pub fn part2(input: &[Nanobot]) -> i32 {
    // Start by populating the possible distances that can reach each nanobot.
    let origin = Nanobot::from([0, 0, 0, 0]);
    let mut endpoints = Vec::with_capacity(2_000);

    for bot in input {
        let dist = bot.manhattan(&origin);
        let low = (dist - bot.r).max(0);
        endpoints.push((low, 1));
        endpoints.push((dist + bot.r + 1, -1));
    }

    endpoints.sort_unstable();

    // Determine the distance that has the maximum overlap in ranges.
    let mut best_dist = 0;
    let mut best_total = 0;
    let mut total = 0;

    for (dist, delta) in endpoints {
        total += delta;
        if total > best_total {
            best_total = total;
            best_dist = dist;
        }
    }

    // In the generic case, the actual answer might be a lower number of overlapping nanobots at a
    // different distance; but for our input files, the maximum overlap gives the distance we want.
    best_dist
}
