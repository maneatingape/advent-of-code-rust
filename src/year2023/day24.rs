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
//! * `e` and `k` are the x velocities.
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
//! The position and velocity of the rock is found by solving 6 linear simultaneous equations in
//! 6 unknowns using [Gaussian elimination](https://en.wikipedia.org/wiki/Gaussian_elimination).
//!
//! We start with the same parametric approach in time:
//!
//! * `p` and `pₕ` are the positions of the rock and an arbitrary hailstone.
//! * `v` and `vₕ` are the velocities.
//! * `p + tv = pₕ + tvₕ` => `p - pₕ = t(v - vₕ)`
//!
//! The key insight is that the two vectors `p - pₕ` and `v - vₕ` differ only by a constant so their
//! [vector cross product](https://en.wikipedia.org/wiki/Cross_product) must be zero. Using a
//! similar notation as part one where `c` and `f` are the z position and velocity of the hailstone
//! and `q`, `r` and `s` the velocity of the rock, this implies:
//!
//! 1. `(y - b) * (f - s) - (z - c) * (e - r) = 0`
//! 2. `(z - c) * (d - a) - (x - a) * (f - s) = 0`
//! 3. `(x - a) * (e - r) - (y - b) * (d - q) = 0`
//!
//! Multiplying out equation 1 gives:
//!
//! 4. `fy - sy - bf + bs - ez + rz + ce - cr = 0`
//!
//! This equation has two non-linear terms `sy` and `rz` so we can't solve just yet.
//! If we take a second and third hailstones we can derive the equivalent equations:
//!
//! 5. `ly - sy - hl + hs - kz + rz + ik - ir = 0`
//! 6. `ry - sy - nr + ns - qz + rz + ok - or = 0`
//!
//! Subtracting 4 from both 5 and 6 removes the non-linear terms and leaves:
//!
//! 7. `(l - f)y + bf - hl + (h - b)s + (e - k)z + ik - ce + (c - i)r = 0`
//! 8. `(r - f)y + bf - nr + (n - b)s + (e - q)z + oq - ce + (c - o)r = 0`
//!
//! Reordering gives:
//!
//! 9. `0x + (l - f)y + (e - k)z + 0q + (c - i)r + (h - b)s = ce - bf + hl - ik`
//! 10. `0x + (r - f)y + (e - q)z + 0q + (c - o)r + (n - b)s = ce - bf + nr - oq`
//!
//! We can do the same for equations 2 and 3 to end up with 6 linear equations that can
//! be solved using Gaussian elimination.
use crate::util::iter::*;
use crate::util::math::*;
use crate::util::parse::*;
use std::ops::RangeInclusive;

const RANGE: RangeInclusive<i64> = 200_000_000_000_000..=400_000_000_000_000;

pub fn parse(input: &str) -> Vec<[i64; 6]> {
    input.iter_signed().chunk::<6>().collect()
}

pub fn part1(input: &[[i64; 6]]) -> u32 {
    let mut result = 0;

    for first in 1..input.len() {
        for second in 0..first {
            let [a, b, _, d, e, _] = input[first];
            let [g, h, _, j, k, _] = input[second];

            // If the determinant is zero there is no solution possible
            // which implies the trajectories are parallel.
            let determinant = e * j - d * k;
            if determinant == 0 {
                continue;
            }

            // Invert the 2x2 matrix then multiply by the respective columns to find the times.
            let t = (j * (h - b) - k * (g - a)) / determinant;
            let u = (d * (h - b) - e * (g - a)) / determinant;

            // We can pick either the first or second hailstone to find the intersection position.
            let x = a + t * d;
            let y = b + t * e;

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
    let widen = |i: usize| input[i].map(|n| n as i128);
    let [a, b, c, d, e, f] = widen(0);
    let [g, h, i, j, k, l] = widen(1);
    let [m, n, o, p, q, r] = widen(2);

    // Coefficients for the 6 simulataneous linear equations.
    // Columns are px, py, pz, vx, vy, vz of the rock equal to a constant.
    let mut matrix = [
        [0, l - f, e - k, 0, c - i, h - b, e * c - b * f + h * l - k * i],
        [0, r - f, e - q, 0, c - o, n - b, e * c - b * f + n * r - q * o],
        [f - l, 0, j - d, i - c, 0, a - g, a * f - d * c + j * i - g * l],
        [f - r, 0, p - d, o - c, 0, a - m, a * f - d * c + p * o - m * r],
        [k - e, d - j, 0, b - h, g - a, 0, d * b - a * e + g * k - j * h],
        [q - e, d - p, 0, b - n, m - a, 0, d * b - a * e + m * q - p * n],
    ];

    // Use Gaussian elimination to solve for the 6 unknowns.
    // Forward elimination, processing columns from left to right.
    // This will leave a matrix in row echelon form.
    for pivot in 0..6 {
        // Make leading coefficient of each row positive to make subsequent calculations easier.
        for row in &mut matrix[pivot..] {
            if row[pivot] < 0 {
                // Flip signs of each coefficient.
                row.iter_mut().for_each(|n| *n = -*n);
            }
        }

        loop {
            // Reduce by GCD each time otherwise coefficients will overflow even a `i128`.
            for row in &mut matrix[pivot..] {
                let mut factor = 0;

                for &next in &row[pivot..] {
                    if next != 0 {
                        if factor == 0 {
                            factor = next.abs();
                        } else {
                            factor = factor.gcd(next.abs());
                        }
                    }
                }

                row[pivot..].iter_mut().for_each(|c| *c /= factor);
            }

            let column = matrix.map(|row| row[pivot]);

            // If only one non-zero coefficient remaining in the column then we're done.
            if column[pivot..].iter().filter(|&&c| c > 0).count() == 1 {
                // Move this row into the pivot location
                let index = column.iter().rposition(|&c| c > 0).unwrap();
                matrix.swap(pivot, index);
                break;
            }

            // Find the row with the lowest non-zero leading coefficient.
            let min = *column[pivot..].iter().filter(|&&c| c > 0).min().unwrap();
            let index = column.iter().rposition(|&c| c == min).unwrap();

            // Subtract as many multiples of this minimum row from each other row as possible
            // to shrink the coefficients of our column towards zero.
            for row in pivot..6 {
                if row != index && column[row] != 0 {
                    let factor = column[row] / min;

                    for col in pivot..7 {
                        matrix[row][col] -= factor * matrix[index][col];
                    }
                }
            }
        }
    }

    // Back substitution, processing columns from right to left.
    // This will leave the matrix in reduced row echelon form.
    // The solved unknowns are then in the 7th column.
    for pivot in (0..6).rev() {
        // We're explicitly told that the results are integers so integer division is safe
        // and will not mangle result.
        matrix[pivot][6] /= matrix[pivot][pivot];

        for row in 0..pivot {
            matrix[row][6] -= matrix[pivot][6] * matrix[row][pivot];
        }
    }

    // x + y + z
    matrix[0][6] + matrix[1][6] + matrix[2][6]
}
