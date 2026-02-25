//! # Particle Swarm
//!
//! ## Part One
//!
//! The particle that remains closest to the origin as time goes to infinity has the lowest
//! acceleration, measured via its manhattan value. If more than one particle shares the same
//! lowest acceleration then ties are broken by velocity then by position.
//!
//! ## Part Two
//!
//! The input is constructed so that all collisions happen within 40 ticks so a simple brute force
//! solution is much faster than more elegant alternatives, for example solving the quadratic
//! equation describing each particle's position.
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    #[inline]
    fn new(cs: [i32; 3]) -> Self {
        Vector { x: cs[0], y: cs[1], z: cs[2] }
    }

    #[inline]
    fn manhattan(self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    #[inline]
    fn tick(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Copy, Clone)]
pub struct Particle {
    id: usize,
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
}

impl Particle {
    #[inline]
    fn tick(&mut self) {
        self.velocity.tick(self.acceleration);
        self.position.tick(self.velocity);
    }
}

pub fn parse(input: &str) -> Vec<Particle> {
    input
        .iter_signed()
        .chunk::<3>()
        .chunk::<3>()
        .enumerate()
        .map(|(id, cs)| Particle {
            id,
            position: Vector::new(cs[0]),
            velocity: Vector::new(cs[1]),
            acceleration: Vector::new(cs[2]),
        })
        .collect()
}

pub fn part1(input: &[Particle]) -> usize {
    let mut candidates = Vec::new();
    let mut min = i32::MAX;

    // Find particles with the lowest acceleration.
    for particle in input {
        let next = particle.acceleration.manhattan();

        if next < min {
            candidates.clear();
            min = next;
        }
        if next == min {
            candidates.push(*particle);
        }
    }

    // Ensure all acceleration, velocity and position vectors are "aligned", that is the
    // sign of each component is the same, for example a particle with a negative x acceleration
    // should also have a negative x velocity and negative x position.
    for _ in 0..1000 {
        candidates.iter_mut().for_each(Particle::tick);
    }

    // Tie break by velocity then by position.
    candidates
        .iter()
        .min_by_key(|p| {
            (p.acceleration.manhattan(), p.velocity.manhattan(), p.position.manhattan())
        })
        .unwrap()
        .id
}

pub fn part2(input: &[Particle]) -> usize {
    let mut particles = input.to_vec();
    let mut collisions = FastMap::with_capacity(input.len());
    let mut alive = vec![true; input.len()];

    for _ in 1..40 {
        for (i, particle) in particles.iter_mut().enumerate() {
            // Only consider particles that haven't collided in a previous tick.
            // Multiple particles can collide in the same tick.
            if alive[i] {
                particle.tick();

                if let Some(j) = collisions.insert(particle.position, i) {
                    alive[i] = false;
                    alive[j] = false;
                }
            }
        }

        collisions.clear();
    }

    alive.iter().filter(|&&a| a).count()
}
