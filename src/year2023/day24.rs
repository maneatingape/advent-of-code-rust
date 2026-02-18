//! # Never Tell Me The Odds
//!
//! ## Part One
//!
//! We find the intersection for each pair of hailstones by solving a pair of linear simultaneous
//! equations in 2 unknowns:
//!
//! * `a` and `g` are the x positions of the pair of hailstones.
//! * `b` and `h` are the y positions.
//! * `d` and `j` are the x velocities.
//! * `e` and `k` are the y velocities.
//! * Let `t` and `u` be the times that the first and second hailstone respectively are at the
//!   intersection point.
//!
//! Then we can write:
//!
//! * `a + dt = g + ju` => `dt - ju = g - a`
//! * `b + et = h + ku` => `et - ku = h - b`
//!
//! In matrix form:
//!
//! ```none
//!     | d  -j ||u| = | g - a |
//!     | e  -k ||t|   | h - b |
//! ```
//!
//! Solve by finding the inverse of the 2x2 matrix and premultiplying both sides. The inverse is:
//!
//! ```none
//!    ______1______ | -k  j |
//!    d(-k) - (-j)e | -e  d |
//! ```
//!
//! Then we check that both times are non-negative and that the intersection point is inside the
//! target area.
//!
//! ## Part Two
//!
//! First we choose 3 arbitrary hailstones. Then we subtract the position and velocity of
//! the first to make the other two relative.
//!
//! The two hailstones will intercept a line leaving the origin. We can determine this line
//! by intersecting the two planes that the hailstone's velocity lie in. These planes are
//! defined by a normal vector orthogonal to the plane.
//!
//! This normal vector is the [cross product](https://en.wikipedia.org/wiki/Cross_product) of
//! any two vectors that lie in the plane, in this case the velocity and also the vector from the
//! origin to the starting location of the hailstone.
//!
//! The direction but not necessarily the magnitude of the velocity is then given by the cross
//! product of the two normals.
//!
//! Given the rock direction we can calculate the times that the two hailstones are intercepted
//! then use this to determine the original position of the rock, as long as the two times
//! are different.
use crate::util::iter::*;
use crate::util::math::*;
use crate::util::parse::*;
use std::ops::RangeInclusive;

const RANGE: RangeInclusive<i64> = 200_000_000_000_000..=400_000_000_000_000;

#[derive(Clone, Copy)]
struct Vector {
    x: i128,
    y: i128,
    z: i128,
}

/// 3D vector implementation.
impl Vector {
    fn add(self, other: Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Vector { x, y, z }
    }

    fn sub(self, other: Self) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Vector { x, y, z }
    }

    fn cross(self, other: Self) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Vector { x, y, z }
    }

    // Changes the magnitude (but not direction) of the vector.
    // Prevents numeric overflow.
    fn gcd(self) -> Self {
        let gcd = self.x.gcd(self.y).gcd(self.z);
        let x = self.x / gcd;
        let y = self.y / gcd;
        let z = self.z / gcd;
        Vector { x, y, z }
    }

    fn sum(self) -> i128 {
        self.x + self.y + self.z
    }
}

pub fn parse(input: &str) -> Vec<[i64; 6]> {
    input.iter_signed().chunk::<6>().collect()
}

pub fn part1(input: &[[i64; 6]]) -> u32 {
    let mut result = 0;

    for (index, &[a, b, _, c, d, _]) in input[1..].iter().enumerate() {
        for &[e, f, _, g, h, _] in &input[..index + 1] {
            // If the determinant is zero there is no solution possible
            // which implies the trajectories are parallel.
            let determinant = d * g - c * h;
            if determinant == 0 {
                continue;
            }

            // Invert the 2x2 matrix then multiply by the respective columns to find the times.
            let t = (g * (f - b) - h * (e - a)) / determinant;
            let u = (c * (f - b) - d * (e - a)) / determinant;

            // We can pick either the first or second hailstone to find the intersection position.
            let x = a + t * c;
            let y = b + t * d;

            // Both times must be in the future and the position within the specified area.
            if t >= 0 && u >= 0 && RANGE.contains(&x) && RANGE.contains(&y) {
                result += 1;
            }
        }
    }

    result
}

pub fn part2(input: &[[i64; 6]]) -> i128 {
    // Calculations need the range of `i128`.
    let widen = |i: usize| {
        let [px, py, pz, vx, vy, vz] = input[i].map(|n| n as i128);
        let p = Vector { x: px, y: py, z: pz };
        let v = Vector { x: vx, y: vy, z: vz };
        (p, v)
    };

    // Take 3 arbitrary hailstones.
    let (p0, v0) = widen(0);
    let (p1, v1) = widen(1);
    let (p2, v2) = widen(2);

    // Subtract the positions and velocities to make them relative.
    // The first hailstone is stationary at the origin.
    let p3 = p1.sub(p0);
    let p4 = p2.sub(p0);
    let v3 = v1.sub(v0);
    let v4 = v2.sub(v0);

    // Find the normal to the plane that the second and third hailstones velocity lies in.
    // This is the cross product of their respective position and velocity.
    // The cross product `s` of these two vectors is the same direction but not necessarily the
    // same magnitude of the desired velocity of the rock.
    // Only the direction is relevant (not the magnitude) so we can normalize the vector by the
    // GCD of its components in order to prevent numeric overflow.
    let q = v3.cross(p3).gcd();
    let r = v4.cross(p4).gcd();
    let s = q.cross(r).gcd();

    // Find the times when the second and third hailstone intercept this vector.
    // If the times are different then we can extrapolate the original position of the rock.
    let t = (p3.y * s.x - p3.x * s.y) / (v3.x * s.y - v3.y * s.x);
    let u = (p4.y * s.x - p4.x * s.y) / (v4.x * s.y - v4.y * s.x);
    assert!(t != u);

    // Calculate the original position of the rock, remembering to add the first hailstone's
    // position to convert back to absolute coordinates.
    let a = p0.add(p3).sum();
    let b = p0.add(p4).sum();
    let c = v3.sub(v4).sum();
    (u * a - t * b + u * t * c) / (u - t)
}
