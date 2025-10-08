//! # Lobby Layout
//!
//! Hex grid parsing and navigation uses
//! [Axial Coordinates](https://www.redblobgames.com/grids/hexagons/#coordinates-cube)
//! exactly as described in the excellent [Red Blob Games](https://www.redblobgames.com/) blog.
//!
//! Part two uses exactly the same approach as [`day 17`] and most of the code is identical.
//!
//! As the black tiles are very sparse (about 8% for my input) it's faster to switch from
//! a "pull" model where we check the surrounding neighbors of each tile, to a "push" model
//! where we update the neighbors of each black tile instead.
//!
//! The SIMD alterative approach is much faster, processing 32 lanes at a time. As a further
//! optimisation we skip rows and columns that the active state hasn't reached. The approach
//! is very similar to [`day 11`].
//!
//! [`day 17`]: crate::year2020::day17
//! [`day 11`]: crate::year2020::day11
use crate::util::hash::*;

#[derive(PartialEq, Eq, Hash)]
pub struct Hex {
    q: i32,
    r: i32,
}

pub fn parse(input: &str) -> FastSet<Hex> {
    let mut tiles = FastSet::new();

    for line in input.lines() {
        let mut iter = line.bytes();
        let mut q = 0;
        let mut r = 0;

        while let Some(b) = iter.next() {
            match b {
                b'e' => q += 1,
                b'w' => q -= 1,
                b'n' => {
                    if b'e' == iter.next().unwrap() {
                        q += 1;
                    }
                    r -= 1;
                }
                b's' => {
                    if b'e' != iter.next().unwrap() {
                        q -= 1;
                    }
                    r += 1;
                }
                _ => unreachable!(),
            }
        }

        let tile = Hex { q, r };
        if tiles.contains(&tile) {
            tiles.remove(&tile);
        } else {
            tiles.insert(tile);
        }
    }

    tiles
}

pub fn part1(input: &FastSet<Hex>) -> usize {
    input.len()
}

pub fn part2(input: &FastSet<Hex>) -> usize {
    #[cfg(not(feature = "simd"))]
    let result = scalar::simulate(input);

    #[cfg(feature = "simd")]
    let result = simd::simulate(input);

    result
}

#[cfg(not(feature = "simd"))]
mod scalar {
    use super::*;
    use std::array::from_fn;

    pub(super) fn simulate(input: &FastSet<Hex>) -> usize {
        // Determine bounds
        let (q1, q2, r1, r2) =
            input.iter().fold((i32::MAX, i32::MIN, i32::MAX, i32::MIN), |(q1, q2, r1, r2), hex| {
                (q1.min(hex.q), q2.max(hex.q), r1.min(hex.r), r2.max(hex.r))
            });

        // Create array with enough space to allow expansion for 100 generations.
        // 2 * (100 generations + 1 buffer) + Origin = 203 extra in each dimension
        let width = q2 - q1 + 203;
        let height = r2 - r1 + 203;
        let neighbors: [i32; 6] = [-1, 1, -width, width, 1 - width, width - 1];
        let neighbors: [usize; 6] = from_fn(|i| neighbors[i] as usize);

        let mut active = Vec::with_capacity(5_000);
        let mut candidates = Vec::with_capacity(5_000);
        let mut next_active = Vec::with_capacity(5_000);

        // Create initial active state, offsetting tiles so that all indices are positive.
        for hex in input {
            let index = width * (hex.r - r1 + 101) + (hex.q - q1 + 101);
            active.push(index as usize);
        }

        for _ in 0..100 {
            let mut state: Vec<u8> = vec![0; (width * height) as usize];

            for &tile in &active {
                for &offset in &neighbors {
                    // Earlier we converted the offsets from signed `i32` to unsigned `usize`. To
                    // achieve subtraction for negative indices, we use a `wrapping_add` that performs
                    // [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) arithmetic.
                    let index = tile.wrapping_add(offset);
                    state[index] += 1;

                    if state[index] == 2 {
                        candidates.push(index);
                    }
                }
            }

            // Active tiles remain active with both one and two neighbors.
            for &tile in &active {
                if state[tile] == 1 {
                    next_active.push(tile);
                }
            }

            // Check that the neighbor count for inactive tiles hasn't exceeded two.
            for &tile in &candidates {
                if state[tile] == 2 {
                    next_active.push(tile);
                }
            }

            // Swap to make next generation the current generation.
            (active, next_active) = (next_active, active);
            candidates.clear();
            next_active.clear();
        }

        active.len()
    }
}

#[cfg(feature = "simd")]
mod simd {
    use super::*;
    use std::simd::cmp::SimdPartialEq as _;
    use std::simd::*;

    const LANE_WIDTH: usize = 32;
    type Vector = Simd<u8, LANE_WIDTH>;

    pub(super) fn simulate(input: &FastSet<Hex>) -> usize {
        // Determine bounds
        let (q1, q2, r1, r2) =
            input.iter().fold((i32::MAX, i32::MIN, i32::MAX, i32::MIN), |(q1, q2, r1, r2), hex| {
                (q1.min(hex.q), q2.max(hex.q), r1.min(hex.r), r2.max(hex.r))
            });

        // Create array with enough space to allow expansion for 100 generations.
        // 2 * (100 generations + 1 buffer) + Origin = 203 extra in each dimension
        let width = 2 + ((q2 - q1 + 201) as usize).next_multiple_of(LANE_WIDTH);
        let height = (r2 - r1 + 203) as usize;

        // Create initial active state, offsetting tiles so that all indices are positive.
        let mut current = vec![0; width * height];
        let mut next = vec![0; width * height];

        for hex in input {
            let index = width * (hex.r - r1 + 101) as usize + (hex.q - q1 + 101) as usize;
            current[index] = 1;
        }

        let zero: Vector = Simd::splat(0);
        let one: Vector = Simd::splat(1);
        let two: Vector = Simd::splat(2);

        for round in 0..100 {
            // The active state boundary expands by 1 horizontally and vertically each round.
            let edge = 100 - round;

            for x in (edge..width - edge).step_by(LANE_WIDTH) {
                for y in edge..height - edge {
                    let index = width * y + x;
                    let tiles = Simd::from_slice(&current[index..]);

                    let above = left_center(&current, index - width);
                    let row = left_right(&current, index);
                    let below = center_right(&current, index + width);
                    let total = row + above + below;

                    // Black => Black (one neighbor).
                    let first = (tiles.simd_eq(one) & total.simd_eq(one)).select(one, zero);
                    // White => Black and Black => Black (two neighbors).
                    let second = total.simd_eq(two).select(one, zero);

                    let result = first + second;
                    result.copy_to_slice(&mut next[index..]);
                }
            }

            (current, next) = (next, current);
        }

        current.iter().map(|&b| b as usize).sum()
    }

    #[inline]
    fn left_center(current: &[u8], index: usize) -> Vector {
        let center = Simd::from_slice(&current[index..]);
        let left = center.shift_elements_left::<1>(current[index + LANE_WIDTH]);
        left + center
    }

    #[inline]
    fn left_right(current: &[u8], index: usize) -> Vector {
        let center = Simd::from_slice(&current[index..]);
        let left = center.shift_elements_left::<1>(current[index + LANE_WIDTH]);
        let right = center.shift_elements_right::<1>(current[index - 1]);
        left + right
    }

    #[inline]
    fn center_right(current: &[u8], index: usize) -> Vector {
        let center = Simd::from_slice(&current[index..]);
        let right = center.shift_elements_right::<1>(current[index - 1]);
        center + right
    }
}
