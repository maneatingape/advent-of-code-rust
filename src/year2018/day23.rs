//! # Experimental Emergency Teleportation
//!
//! Part two implements a 3D version of binary search. Starting with a single cube that encloses all
//! nanobots, each cube is further split into 8 smaller cubes until we find the answer.
//! Cubes are stored in a [`MinHeap`] ordered by:
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
//! [`MinHeap`]: crate::util::heap
use crate::util::heap::*;
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

struct Cube {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

impl Cube {
    fn new(x1: i32, x2: i32, y1: i32, y2: i32, z1: i32, z2: i32) -> Cube {
        Cube { x1, x2, y1, y2, z1, z2 }
    }

    /// Split the cube into 8 non-overlapping sub-cubes.
    /// Since each cube size is always of power of two, we can safely divide by 2.
    fn split(&self) -> [Cube; 8] {
        let Cube { x1, x2, y1, y2, z1, z2 } = *self;

        // Lower and upper halves of the new sub-cubes.
        let lx = self.x1.midpoint(self.x2);
        let ly = self.y1.midpoint(self.y2);
        let lz = self.z1.midpoint(self.z2);
        let ux = lx + 1;
        let uy = ly + 1;
        let uz = lz + 1;

        // 8 possible permutations of lower and upper halves for each axis.
        [
            Cube::new(x1, lx, y1, ly, z1, lz),
            Cube::new(ux, x2, y1, ly, z1, lz),
            Cube::new(x1, lx, uy, y2, z1, lz),
            Cube::new(ux, x2, uy, y2, z1, lz),
            Cube::new(x1, lx, y1, ly, uz, z2),
            Cube::new(ux, x2, y1, ly, uz, z2),
            Cube::new(x1, lx, uy, y2, uz, z2),
            Cube::new(ux, x2, uy, y2, uz, z2),
        ]
    }

    // Compute the Manattan distance from the faces of the cube to the octohedron shaped region
    // within range of the Nanbot.
    fn in_range(&self, nb: &Nanobot) -> bool {
        let x = (self.x1 - nb.x).max(0) + (nb.x - self.x2).max(0);
        let y = (self.y1 - nb.y).max(0) + (nb.y - self.y2).max(0);
        let z = (self.z1 - nb.z).max(0) + (nb.z - self.z2).max(0);
        x + y + z <= nb.r
    }

    /// Find the corner closest to the origin, considering each axis independently.
    fn closest(&self) -> i32 {
        let x = self.x1.abs().min(self.x2.abs());
        let y = self.y1.abs().min(self.y2.abs());
        let z = self.z1.abs().min(self.z2.abs());
        x + y + z
    }

    /// All axes are the same so choose `x` arbitrarily.
    fn size(&self) -> i32 {
        self.x2 - self.x1 + 1
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
    // Start with a single cube that encloses all nanobots. Cubes faces are aligned to powers of 2,
    // for example 0..4, 8..16, -32..0
    const SIZE: i32 = 1 << 29;
    let mut heap = MinHeap::with_capacity(1_000);
    heap.push((0, 0, 0), Cube::new(-SIZE, SIZE - 1, -SIZE, SIZE - 1, -SIZE, SIZE - 1));

    while let Some((_, cube)) = heap.pop() {
        if cube.size() == 1 {
            return cube.closest();
        }

        for next in cube.split() {
            let in_range = input.iter().filter(|nb| next.in_range(nb)).count();
            let key = (input.len() - in_range, next.closest(), next.size());
            heap.push(key, next);
        }
    }

    unreachable!()
}
