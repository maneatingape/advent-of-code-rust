//! # Conway Cubes
//!
//! Solving this problem reveals an interesting insight that the active cells are very sparse.
//! Of the total possible volume of the cube formed by the maximum extents in part one, only
//! roughly 8% is active and of the hypercube from part two only 3% is active for my input.
//!
//! To speed things up our high level strategy will flip from a "pull" model where we check the
//! surroundings neighbors of each cell, to a "push" model where we update the neighbors of each
//! active cell instead.
//!
//! A `HashSet` is generally a good choice for very sparse infinite grids, however for this
//! problem we'll pack all dimensions into a single `vec` to achieve a five times increase
//! in lookup speed.
use crate::util::grid::*;
use crate::util::point::*;

/// x and y dimensions are in the plane of the input. Each dimension can expand at most two in each
/// axis per round (one positive and one negative). Adding padding at the edges to avoid boundary
/// checks gives a maximum width of 8 + 2 * (6 + 1) = 22 for the x and y dimensions and
/// 1 + 2 * (6 + 1) = 15 for the z and w dimensions.
mod size {
    pub const X: i32 = 22;
    pub const Y: i32 = 22;
    pub const Z: i32 = 15;
    pub const W: i32 = 15;
}

/// Pack a four dimensional array into a one dimensional vec to avoid the speed penalty of
/// following multiple pointers and increase memory locality for caching.
mod stride {
    use super::size;
    pub const X: i32 = 1;
    pub const Y: i32 = size::X * X;
    pub const Z: i32 = size::Y * Y;
    pub const W: i32 = size::Z * Z;
}

/// Use our utility [`Grid`] method to parse the input.
///
/// [`Grid`]: crate::util::grid::Grid
pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

/// Part one cells form a cube.
pub fn part1(input: &Grid<u8>) -> usize {
    let size = size::X * size::Y * size::Z;
    let base = stride::X + stride::Y + stride::Z;
    boot_process(input, size, base, &[0])
}

/// Part two form a hypercube.
pub fn part2(input: &Grid<u8>) -> usize {
    let size = size::X * size::Y * size::Z * size::W;
    let base = stride::X + stride::Y + stride::Z + stride::W;
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
                    let offset = x * stride::X + y * stride::Y + z * stride::Z + w * stride::W;
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
            if input[Point { x, y }] == b'#' {
                let index = 7 * base + x + y * stride::Y;
                active.push(index as usize);
            }
        }
    }

    for _ in 0..6 {
        let mut state: Vec<u8> = vec![0; size as usize];

        for &cube in active.iter() {
            for &offset in neighbors.iter() {
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
        for &cube in active.iter() {
            if state[cube] == 2 {
                next_active.push(cube);
            }
        }

        // Check that the neighbor count for inactive cubes hasn't exceeded three.
        for &cube in candidates.iter() {
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
