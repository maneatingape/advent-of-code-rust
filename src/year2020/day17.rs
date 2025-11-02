//! # Conway Cubes
//!
//! Solving this problem reveals an interesting insight that the active cells are very sparse.
//! Of the total possible volume of the cube formed by the maximum extents in part one, only
//! roughly 8% is active and of the hypercube from part two only 3% is active for my input.
//!
//! To speed things up our high-level strategy will flip from a "pull" model where we check the
//! surrounding neighbors of each cell, to a "push" model where we update the neighbors of each
//! active cell instead.
//!
//! A `HashSet` is generally a good choice for very sparse infinite grids, however for this
//! problem we'll pack all dimensions into a single `vec` to achieve a five times increase
//! in lookup speed.
use crate::util::grid::*;
use crate::util::point::*;

/// Use our utility [`Grid`] method to parse the input.
///
/// [`Grid`]: crate::util::grid::Grid
pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

/// Part one cells form a cube.
pub fn part1(input: &Grid<u8>) -> usize {
    #[cfg(not(feature = "simd"))]
    let result = scalar::three_dimensions(input);

    #[cfg(feature = "simd")]
    let result = simd::three_dimensions(input);

    result
}

/// Part two forms a hypercube.
pub fn part2(input: &Grid<u8>) -> usize {
    #[cfg(not(feature = "simd"))]
    let result = scalar::four_dimensions(input);

    #[cfg(feature = "simd")]
    let result = simd::four_dimensions(input);

    result
}

#[cfg(not(feature = "simd"))]
mod scalar {
    use super::*;

    /// x and y dimensions are in the plane of the input. Each dimension can expand by at most two
    /// in each axis per round (one positive and one negative). Adding padding at the edges to avoid
    /// boundary checks gives a maximum width of 8 + 2 * (6 + 1) = 22 for the x and y dimensions and
    /// 1 + 2 * (6 + 1) = 15 for the z and w dimensions.
    const X: i32 = 22;
    const Y: i32 = 22;
    const Z: i32 = 15;
    const W: i32 = 15;

    /// Pack a four dimensional array into a one dimensional vec to avoid the speed penalty of
    /// following multiple pointers and increase memory locality for caching.
    const STRIDE_X: i32 = 1;
    const STRIDE_Y: i32 = X * STRIDE_X;
    const STRIDE_Z: i32 = Y * STRIDE_Y;
    const STRIDE_W: i32 = Z * STRIDE_Z;

    pub(super) fn three_dimensions(input: &Grid<u8>) -> usize {
        let size = X * Y * Z;
        let base = STRIDE_X + STRIDE_Y + STRIDE_Z;
        boot_process(input, size, base, &[0])
    }

    pub(super) fn four_dimensions(input: &Grid<u8>) -> usize {
        let size = X * Y * Z * W;
        let base = STRIDE_X + STRIDE_Y + STRIDE_Z + STRIDE_W;
        boot_process(input, size, base, &[-1, 0, 1])
    }

    /// Re-use logic between both parts.
    fn boot_process(input: &Grid<u8>, size: i32, base: i32, fourth_dimension: &[i32]) -> usize {
        let dimension = [-1, 0, 1];
        let mut neighbors = Vec::new();

        // Pre-calculate either the 26 or 80 offsets formed by the combination of dimensions.
        for x in dimension {
            for y in dimension {
                for z in dimension {
                    for w in fourth_dimension {
                        let offset = x * STRIDE_X + y * STRIDE_Y + z * STRIDE_Z + w * STRIDE_W;
                        if offset != 0 {
                            neighbors.push(offset as usize);
                        }
                    }
                }
            }
        }

        let mut active = Vec::with_capacity(5_000);
        let mut candidates = Vec::with_capacity(5_000);
        let mut next_active = Vec::with_capacity(5_000);

        // To prevent negative array indices offset the starting cells by seven units in each
        // dimension. This allows six for growth, plus one for padding to prevent needing edge checks.
        for x in 0..input.width {
            for y in 0..input.height {
                if input[Point::new(x, y)] == b'#' {
                    let index = 7 * base + x + y * STRIDE_Y;
                    active.push(index as usize);
                }
            }
        }

        for _ in 0..6 {
            let mut state: Vec<u8> = vec![0; size as usize];

            for &cube in &active {
                for &offset in &neighbors {
                    // Earlier we converted the offsets from signed `i32` to unsigned `usize`. To
                    // achieve subtraction for negative indices, we use a `wrapping_add` that performs
                    // [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) arithmetic.
                    let index = cube.wrapping_add(offset);
                    state[index] += 1;

                    if state[index] == 3 {
                        candidates.push(index);
                    }
                }
            }

            // Active cubes remain active with both two and three neighbors.
            for &cube in &active {
                if state[cube] == 2 {
                    next_active.push(cube);
                }
            }

            // Check that the neighbor count for inactive cubes hasn't exceeded three.
            for &cube in &candidates {
                if state[cube] == 3 {
                    next_active.push(cube);
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

    #[expect(clippy::needless_range_loop)]
    pub(super) fn three_dimensions(input: &Grid<u8>) -> usize {
        // Each dimension can expand by at most two in each axis per round. Adding padding at the
        // edges to avoid boundary checks gives a maximum width of 8 + 2 * (6 + 1) = 22 for the x
        // and y dimensions and 1 + 2 * (6 + 1) = 15 for the z and w dimensions.
        let mut current = [[[0; 32]; 22]; 15];
        let mut next = current;
        // Temporary intermediate state.
        let mut first = current;

        // Set initial cubes offset from the edges to allow for growth.
        for x in 0..input.width {
            for y in 0..input.height {
                if input[Point::new(x, y)] == b'#' {
                    current[7][y as usize + 7][x as usize + 7] = 1;
                }
            }
        }

        let zero: Vector = Simd::splat(0);
        let one: Vector = Simd::splat(1);
        let three: Vector = Simd::splat(3);

        for round in 0..6 {
            // Each round state boundary expands by 1 in both positive and negative direction.
            let edge = 5 - round;

            // Sum xs and ys.
            for z in (1 + edge)..(14 - edge) {
                for y in (1 + edge)..(21 - edge) {
                    let above = xs_sum(&current[z][y - 1]);
                    let row = xs_sum(&current[z][y]);
                    let below = xs_sum(&current[z][y + 1]);

                    (row + above + below).copy_to_slice(&mut first[z][y]);
                }
            }

            // Sum zs and calculate next state.
            for z in (1 + edge)..(14 - edge) {
                for y in (1 + edge)..(21 - edge) {
                    let above = from_slice(&first[z - 1][y]);
                    let row = from_slice(&first[z][y]);
                    let below = from_slice(&first[z + 1][y]);
                    let state = from_slice(&current[z][y]);

                    let total = row + above + below - state;
                    let result = (state | total).simd_eq(three).select(one, zero);
                    result.copy_to_slice(&mut next[z][y]);
                }
            }

            (current, next) = (next, current);
        }

        let mut result = 0;

        for z in 1..14 {
            for y in 1..21 {
                for x in 1..21 {
                    result += current[z][y][x] as usize;
                }
            }
        }

        result
    }

    #[expect(clippy::needless_range_loop)]
    pub(super) fn four_dimensions(input: &Grid<u8>) -> usize {
        // Same size logic as part one with added `w` dimension.
        let mut current = [[[[0; 32]; 22]; 15]; 15];
        let mut next = current;
        // Temporary intermediate state.
        let mut first = current;
        let mut second = current;

        // Set initial cubes offset from the edges to allow for growth.
        for x in 0..input.width {
            for y in 0..input.height {
                if input[Point::new(x, y)] == b'#' {
                    current[7][7][y as usize + 7][x as usize + 7] = 1;
                }
            }
        }

        let zero: Vector = Simd::splat(0);
        let one: Vector = Simd::splat(1);
        let three: Vector = Simd::splat(3);

        for round in 0..6 {
            // Each round state boundary expands by 1 in both positive and negative direction.
            let edge = 5 - round;

            // Sum xs and ys.
            for w in (1 + edge)..(14 - edge) {
                for z in (1 + edge)..(14 - edge) {
                    for y in (1 + edge)..(21 - edge) {
                        let above = xs_sum(&current[w][z][y - 1]);
                        let row = xs_sum(&current[w][z][y]);
                        let below = xs_sum(&current[w][z][y + 1]);

                        (row + above + below).copy_to_slice(&mut first[w][z][y]);
                    }
                }
            }

            // Sum zs.
            for w in (1 + edge)..(14 - edge) {
                for z in (1 + edge)..(14 - edge) {
                    for y in (1 + edge)..(21 - edge) {
                        let above = from_slice(&first[w][z - 1][y]);
                        let row = from_slice(&first[w][z][y]);
                        let below = from_slice(&first[w][z + 1][y]);

                        (row + above + below).copy_to_slice(&mut second[w][z][y]);
                    }
                }
            }

            // Sum ws and calculate next state.
            for w in (1 + edge)..(14 - edge) {
                for z in (1 + edge)..(14 - edge) {
                    for y in (1 + edge)..(21 - edge) {
                        let above = from_slice(&second[w - 1][z][y]);
                        let row = from_slice(&second[w][z][y]);
                        let below = from_slice(&second[w + 1][z][y]);
                        let state = from_slice(&current[w][z][y]);

                        let total = row + above + below - state;
                        let result = (state | total).simd_eq(three).select(one, zero);
                        result.copy_to_slice(&mut next[w][z][y]);
                    }
                }
            }

            (current, next) = (next, current);
        }

        let mut result = 0;

        for w in 1..14 {
            for z in 1..14 {
                for y in 1..21 {
                    for x in 1..21 {
                        result += current[w][z][y][x] as usize;
                    }
                }
            }
        }

        result
    }

    #[inline]
    fn from_slice(slice: &[u8]) -> Vector {
        Simd::from_slice(slice)
    }

    /// Create SIMD vector of the sum of left, right and center lanes.
    #[inline]
    fn xs_sum(slice: &[u8]) -> Vector {
        let center = Simd::from_slice(slice);
        let left = center.shift_elements_left::<1>(0);
        let right = center.shift_elements_right::<1>(0);

        center + left + right
    }
}
