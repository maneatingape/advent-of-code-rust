//! # Trench Map
//!
//! This is a cellular automata problem, similar to Conway's Game of Life, except that the rules
//! are encoded in the enhancement algorithm string, instead of being statically specified. Each
//! round the initial square area of cells expands by at most one in each direction, so we can store
//! the cell in a fixed size array with enough space on either side to expand into.
//!
//! The interesting nuance is handling the edge cells when all 9 cells are empty (index 0) or all
//! 9 cell are active (index 511). The sample data encodes a blank cell in both scenarios.
//! My input encoded an active cell for index 0 and a blank cell for index 511, meaning that each
//! turn the edge cells toggle from set to unset.
//!
//! The algorithm keeps track of the bounds of the expanding square and supplies a `default` value,
//! that in the example case is always zero, but in the real data toggles between zero and one.
//!
//! A faster SIMD approach processes cells 16 at a time.
use crate::util::grid::*;
use crate::util::point::*;

type Input = (Vec<u8>, Grid<u8>);

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();

    let algorithm = prefix.bytes().map(|b| u8::from(b == b'#')).collect();
    let grid = Grid::parse(suffix);

    (algorithm, grid)
}

pub fn part1(input: &Input) -> u32 {
    #[cfg(not(feature = "simd"))]
    let result = scalar::enhance(input, 2);

    #[cfg(feature = "simd")]
    let result = simd::enhance(input, 2);

    result
}

pub fn part2(input: &Input) -> u32 {
    #[cfg(not(feature = "simd"))]
    let result = scalar::enhance(input, 50);

    #[cfg(feature = "simd")]
    let result = simd::enhance(input, 50);

    result
}

#[cfg(not(feature = "simd"))]
mod scalar {
    use super::*;

    pub(super) fn enhance(input: &Input, steps: i32) -> u32 {
        let (algorithm, grid) = input;

        // Offset the initial square by `step` + 1 buffer cells in both dimensions.
        // The square expands by at most one in each step so this is enough room to stay within bounds.
        let extra = steps + 1;
        let offset = Point::new(extra, extra);
        let mut pixels = Grid::new(grid.width + 2 * extra, grid.height + 2 * extra, 0);

        for y in 0..grid.height {
            for x in 0..grid.width {
                let point = Point::new(x, y);
                pixels[point + offset] = u8::from(grid[point] == b'#');
            }
        }

        let mut next = pixels.clone();
        let mut default = 0;
        let mut start = extra;
        let mut end = extra + grid.width;

        for _ in 0..steps {
            for y in (start - 1)..(end + 1) {
                // If the pixel is within current bounds then return it, or else use the `default`
                // edge value specified by the enhancement algorithm.
                let helper = |sx, sy, shift| {
                    let result = if sx < end && start <= sy && sy < end {
                        pixels[Point::new(sx, sy)]
                    } else {
                        default
                    };
                    (result as usize) << shift
                };

                // If the edge pixels are 1 then the initial edge will look like
                // [##a]
                // [##b]
                // [##c]
                // or 11a11b11c when encoded as an index.
                let mut index = if default == 1 { 0b11011011 } else { 0b00000000 };

                for x in (start - 1)..(end + 1) {
                    // Keeps a sliding window of the index, updated as we evaluate the row from
                    // left to right. Shift the index left by one each turn, updating the values from
                    // the three new rightmost pixels entering the window.
                    index = ((index << 1) & 0b110110110)
                        + helper(x + 1, y - 1, 6)
                        + helper(x + 1, y, 3)
                        + helper(x + 1, y + 1, 0);

                    next[Point::new(x, y)] = algorithm[index];
                }
            }

            // Swap grids then calculate the next value for edge pixels beyond the boundary.
            (pixels, next) = (next, pixels);
            default = if default == 0 { algorithm[0] } else { algorithm[511] };

            // Boundaries expand by one each turn
            start -= 1;
            end += 1;
        }

        pixels.bytes.iter().map(|&b| b as u32).sum()
    }
}

#[cfg(feature = "simd")]
mod simd {
    use super::*;
    use std::simd::Simd;
    use std::simd::num::SimdUint as _;

    const LANE_WIDTH: usize = 16;
    type Vector = Simd<u16, LANE_WIDTH>;

    pub(super) fn enhance(input: &Input, steps: i32) -> u32 {
        let (algorithm, grid) = input;

        // Offset the initial square by `steps` + 1 buffer cells in both dimensions.
        // The square expands by at most one in each step so this is enough room to stay within bounds.
        let extra = steps + 1;
        let offset = Point::new(extra, extra);
        let mut pixels =
            Grid::new(grid.width + 2 * extra + LANE_WIDTH as i32, grid.height + 2 * extra, 0);

        for y in 0..grid.height {
            for x in 0..grid.width {
                let point = Point::new(x, y);
                pixels[point + offset] = u8::from(grid[point] == b'#');
            }
        }

        let mut next = pixels.clone();
        let mut default = 0;
        let mut start = extra - 1;
        let mut end = extra + grid.width + 1;

        for _ in 0..steps {
            // Edge pixels on the infinite grid flip flop between on and off.
            for y in (start - 1)..(end + 1) {
                pixels[Point::new(start - 1, y)] = default;
                pixels[Point::new(start, y)] = default;
                pixels[Point::new(end - 1, y)] = default;
                pixels[Point::new(end, y)] = default;
            }

            for x in (start..end).step_by(LANE_WIDTH) {
                let edge = Simd::splat(if default == 0 { 0b000 } else { 0b111 });
                let mut above = edge;
                let mut row = edge;

                for y in start..end {
                    let below = if y < end - 2 { from_grid(&pixels, x, y + 1) } else { edge };

                    let indices = (above << 6) | (row << 3) | below;
                    above = row;
                    row = below;

                    let base = (pixels.width * y + x) as usize;
                    for (i, j) in indices.to_array().into_iter().enumerate() {
                        next.bytes[base + i] = algorithm[j as usize];
                    }
                }
            }

            // Swap grids then calculate the next value for edge pixels beyond the boundary.
            (pixels, next) = (next, pixels);
            default = if default == 0 { algorithm[0] } else { algorithm[511] };

            // Boundaries expand by one each turn.
            start -= 1;
            end += 1;
        }

        // Only count pixels inside the boundary.
        let mut result = 0;

        for y in 1..end - 1 {
            for x in 1..end - 1 {
                result += pixels[Point::new(x, y)] as u32;
            }
        }

        result
    }

    #[inline]
    fn from_grid(grid: &Grid<u8>, x: i32, y: i32) -> Vector {
        let index = (grid.width * y + x) as usize;

        let row = Simd::from_slice(&grid.bytes[index..]);
        let left = row.shift_elements_right::<1>(grid[Point::new(x - 1, y)]);
        let right = row.shift_elements_left::<1>(grid[Point::new(x + LANE_WIDTH as i32, y)]);

        let result = (left << 2) | (row << 1) | right;
        result.cast()
    }
}
