//! # Reactor Reboot
//!
//! The key to solving this problem efficiently is the
//! [inclusion-exclusion principle](https://en.wikipedia.org/wiki/Inclusion%E2%80%93exclusion_principle).
//!
//! Looking at a two dimensional example
//! ```none
//!    ┌──────────────┐A            Volume of A: 144
//!    │              │             Volume of B: 66
//!    │ ┌─────────┐B │             Volume of C: 18
//!    │ │         │  │
//!    │ │ ┌────┐C │  │
//!    │ │ │    │  │  │
//!    │ │ └────┘  │  │
//!    │ └─────────┘  │
//!    └──────────────┘
//! ```
//!
//! Using the inclusion-exclusion principle the remaining size of A is:
//!
//! 144 (initial size) - 66 (overlap with B) - 18 (overlap with C) + 18
//! (overlap between B and C) = 78
//!
//! If there were any triple overlaps we would subtract those, add quadruple, and so on until
//! there are no more overlaps remaining.
//!
//! The complexity of this approach depends on how many cubes overlap. In my input most
//! cubes overlapped with zero others, a few with one and rarely with more than one.
use crate::util::iter::*;
use crate::util::parse::*;

/// Wraps a cube with on/off information.
pub struct RebootStep {
    on: bool,
    cube: Cube,
}

impl RebootStep {
    fn from((command, points): (&str, [i32; 6])) -> RebootStep {
        let on = command == "on";
        let cube = Cube::from(points);
        RebootStep { on, cube }
    }
}

/// Technically this is actually a [rectangular cuboid](https://en.wikipedia.org/wiki/Cuboid#Rectangular_cuboid)
/// but that was longer to type!
#[derive(Clone, Copy)]
pub struct Cube {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

impl Cube {
    /// Keeping the coordinates in ascending order per axis makes calculating intersections
    /// and volume easier.
    fn from(points: [i32; 6]) -> Cube {
        let [a, b, c, d, e, f] = points;
        let x1 = a.min(b);
        let x2 = a.max(b);
        let y1 = c.min(d);
        let y2 = c.max(d);
        let z1 = e.min(f);
        let z2 = e.max(f);
        Cube { x1, x2, y1, y2, z1, z2 }
    }

    /// Returns a `Some` of the intersection if two cubes overlap or `None` if they don't.
    fn intersect(&self, other: &Cube) -> Option<Cube> {
        let x1 = self.x1.max(other.x1);
        let x2 = self.x2.min(other.x2);
        let y1 = self.y1.max(other.y1);
        let y2 = self.y2.min(other.y2);
        let z1 = self.z1.max(other.z1);
        let z2 = self.z2.min(other.z2);
        (x1 <= x2 && y1 <= y2 && z1 <= z2).then_some(Cube { x1, x2, y1, y2, z1, z2 })
    }

    /// Returns the volume of a cube, converting to `i64` to prevent overflow.
    fn volume(&self) -> i64 {
        let w = (self.x2 - self.x1 + 1) as i64;
        let h = (self.y2 - self.y1 + 1) as i64;
        let d = (self.z2 - self.z1 + 1) as i64;
        w * h * d
    }
}

pub fn parse(input: &str) -> Vec<RebootStep> {
    let first = input.split_ascii_whitespace().step_by(2);
    let second = input.iter_signed().chunk::<6>();
    first.zip(second).map(RebootStep::from).collect()
}

/// We re-use the logic between part one and two, by first intersecting all cubes with
/// the specified range. Any cubes that lie completely outside the range will be filtered out.
pub fn part1(input: &[RebootStep]) -> i64 {
    let region = Cube { x1: -50, x2: 50, y1: -50, y2: 50, z1: -50, z2: 50 };

    let filtered: Vec<_> = input
        .iter()
        .filter_map(|RebootStep { on, cube }| {
            region.intersect(cube).map(|next| RebootStep { on: *on, cube: next })
        })
        .collect();

    part2(&filtered)
}

pub fn part2(input: &[RebootStep]) -> i64 {
    let mut total = 0;
    let mut candidates = Vec::new();
    // Only "on" cubes contribute to volume.
    // "off" cubes are considered when subtracting volume
    let on_cubes = input.iter().enumerate().filter_map(|(i, rs)| rs.on.then_some((i, rs.cube)));

    for (i, cube) in on_cubes {
        // Only consider cubes after this one in input order.
        // Previous cubes have already had all possible intersections subtracted from their
        // volume, so no longer need to be considered.
        // We check both "on" and "off" cubes when calculating overlaps to subtract volume.
        candidates.extend(input[(i + 1)..].iter().filter_map(|rs| cube.intersect(&rs.cube)));

        // Apply the inclusion/exclusion principle recursively, considering overlaps of
        // increasingly higher order until there are no more overlaps remaining.
        total += cube.volume() + subsets(&cube, -1, &candidates);
        candidates.clear();
    }

    total
}

// Apply inclusion/exclusion principle. The sign of the result alternates with each level,
// so that we subtract single overlaps, then add double, subtract triple, and so on...
fn subsets(cube: &Cube, sign: i64, candidates: &[Cube]) -> i64 {
    let mut total = 0;

    for (i, other) in candidates.iter().enumerate() {
        if let Some(next) = cube.intersect(other) {
            // Subtle nuance here. Similar to the main input we only need to consider higher level
            // overlaps of inputs *after* this one, as any overlaps with previous cubes
            // have already been considered.
            total += sign * next.volume() + subsets(&next, -sign, &candidates[(i + 1)..]);
        }
    }

    total
}
