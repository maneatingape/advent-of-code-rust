//! # Unstable Diffusion
//!
//! We represent elves as bits in an integer then use bitwise operations to efficiently figure
//! out the movement for multiple elves at once.
use Direction::*;

#[cfg(not(feature = "simd"))]
use scalar::U256;
#[cfg(feature = "simd")]
use simd::U256;

/// The initial grid is 70 x 70. Elves stop moving when no other elf is adjacent so the grid
/// will expand at most 70 in any direction, giving 70 + 70 + 70 = 210 total.
const HEIGHT: usize = 210;

enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Clone, Copy)]
pub struct Input {
    grid: [U256; HEIGHT],
    north: [U256; HEIGHT],
    south: [U256; HEIGHT],
    west: [U256; HEIGHT],
    east: [U256; HEIGHT],
}

/// Converts the ASCII grid into a bit per elf.
pub fn parse(input: &str) -> Input {
    // Enough buffer so that elves won't overflow the edges of the grid.
    let offset = 70;
    let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
    let default = [U256::default(); HEIGHT];
    let mut grid = default;

    for (y, row) in raw.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == b'#' {
                grid[offset + y].set_bit(offset + x);
            }
        }
    }

    Input { grid, north: default, south: default, west: default, east: default }
}

pub fn part1(input: &Input) -> usize {
    let mut input = *input;
    let mut order = [North, South, West, East];

    for _ in 0..10 {
        step(&mut input, &mut order);
    }

    // Find the total number of elves and the bounding rectangle.
    let grid = input.grid;
    let elves = grid.iter().flat_map(U256::as_array).map(u8::count_ones).sum::<u32>() as usize;

    // Vertical bounds.
    let min_y = grid.iter().position(U256::non_zero).unwrap();
    let max_y = grid.iter().rposition(U256::non_zero).unwrap();

    // Horizontal bounds.
    let array = grid.iter().fold(U256::default(), |acc, &n| acc.or(n)).as_array();
    let left = array.iter().position(|&e| e != 0).unwrap();
    let right = array.iter().rposition(|&e| e != 0).unwrap();

    let min_x = 8 * left + array[left].leading_zeros() as usize;
    let max_x = 8 * right + (7 - array[right].trailing_zeros()) as usize;

    // Empty ground tiles.
    (max_x - min_x + 1) * (max_y - min_y + 1) - elves
}

pub fn part2(input: &Input) -> u32 {
    let mut input = *input;
    let mut order = [North, South, West, East];
    let mut moved = true;
    let mut count = 0;

    while moved {
        moved = step(&mut input, &mut order);
        count += 1;
    }

    count
}

fn step(input: &mut Input, order: &mut [Direction]) -> bool {
    let Input { grid, north, south, west, east } = input;
    // Optimization to avoid processing empty rows.
    let start = grid.iter().position(U256::non_zero).unwrap() - 1;
    let end = grid.iter().rposition(U256::non_zero).unwrap() + 2;

    let mut moved = false;

    let mut prev;
    // Find horizontal neighbors in each row. To make movement calculations easier
    // we invert so that a bit is 1 is movement is *possible*.
    let mut cur = grid[0].shr().or(grid[0]).or(grid[0].shl()).not();
    let mut next = grid[1].shr().or(grid[1]).or(grid[1].shl()).not();

    for i in start..end {
        // Calculating neighbors is relatively expensive so re-use results between rows.
        prev = cur;
        cur = next;
        next = grid[i + 1].shr().or(grid[i + 1]).or(grid[i + 1].shl()).not();

        let mut up = prev;
        let mut down = next;
        // Find neighbors in vertical columns.
        let vertical = grid[i - 1].or(grid[i]).or(grid[i + 1]).not();
        let mut left = vertical.shr();
        let mut right = vertical.shl();
        // Elves need at least 1 neighbor to propose moving.
        let mut remaining = grid[i].and(up.and(down).and(left).and(right).not());

        // Consider each direction one at a time, removing any elves who propose it.
        for direction in &*order {
            match direction {
                North => {
                    up = up.and(remaining);
                    remaining = remaining.and(up.not());
                }
                South => {
                    down = down.and(remaining);
                    remaining = remaining.and(down.not());
                }
                West => {
                    left = left.and(remaining);
                    remaining = remaining.and(left.not());
                }
                East => {
                    right = right.and(remaining);
                    remaining = remaining.and(right.not());
                }
            }
        }

        // Copy final proposals to an array for each direction.
        north[i - 1] = up;
        south[i + 1] = down;
        west[i] = left.shl();
        east[i] = right.shr();
    }

    // Elves that propose moving to the same spot cancel each other out and no-one moves.
    // Due to the movement rules we only need to check horizontal and vertical movement into
    // the same spot (horizontal and vertical movement can never collide with each other).
    for i in start..end {
        let up = north[i];
        let down = south[i];
        let left = west[i];
        let right = east[i];
        north[i] = north[i].and(down.not());
        south[i] = south[i].and(up.not());
        west[i] = west[i].and(right.not());
        east[i] = east[i].and(left.not());
    }

    for i in start..end {
        // Stationary elves.
        let same =
            grid[i].and(north[i - 1].or(south[i + 1]).or(west[i].shr()).or(east[i].shl()).not());
        // Moving elves.
        let change = north[i].or(south[i]).or(west[i]).or(east[i]);
        grid[i] = same.or(change);
        moved |= change.non_zero();
    }

    // Rotate the order of movement proposals for the next turn.
    order.rotate_left(1);
    moved
}

#[cfg(not(feature = "simd"))]
mod scalar {
    /// Duct tape two `u128`s together.
    #[derive(Clone, Copy, Default)]
    pub(super) struct U256 {
        left: u128,
        right: u128,
    }

    impl U256 {
        pub(super) fn set_bit(&mut self, offset: usize) {
            if offset < 128 {
                self.left |= 1 << (127 - offset);
            } else {
                self.right |= 1 << (255 - offset);
            }
        }

        pub(super) fn as_array(&self) -> [u8; 32] {
            let mut result = [0; 32];
            result[..16].copy_from_slice(&self.left.to_be_bytes());
            result[16..].copy_from_slice(&self.right.to_be_bytes());
            result
        }

        pub(super) fn non_zero(&self) -> bool {
            self.left != 0 || self.right != 0
        }

        pub(super) fn shl(self) -> U256 {
            U256 { left: (self.left << 1) | (self.right >> 127), right: (self.right << 1) }
        }

        pub(super) fn shr(self) -> U256 {
            U256 { left: (self.left >> 1), right: (self.left << 127) | (self.right >> 1) }
        }

        pub(super) fn and(self, rhs: U256) -> U256 {
            U256 { left: self.left & rhs.left, right: self.right & rhs.right }
        }

        pub(super) fn or(self, rhs: U256) -> U256 {
            U256 { left: self.left | rhs.left, right: self.right | rhs.right }
        }

        pub(super) fn not(self) -> U256 {
            U256 { left: !self.left, right: !self.right }
        }
    }
}

#[cfg(feature = "simd")]
mod simd {
    use std::simd::*;

    #[derive(Clone, Copy, Default)]
    pub(super) struct U256 {
        v: Simd<u8, 32>,
    }

    impl U256 {
        pub(super) fn set_bit(&mut self, offset: usize) {
            self.v[offset / 8] |= 1 << (7 - offset % 8);
        }

        pub(super) fn as_array(&self) -> [u8; 32] {
            self.v.to_array()
        }

        pub(super) fn non_zero(&self) -> bool {
            self.v != Simd::splat(0)
        }

        pub(super) fn shl(self) -> U256 {
            U256 { v: (self.v << 1) | (self.v.shift_elements_left::<1>(0) >> 7) }
        }

        pub(super) fn shr(self) -> U256 {
            U256 { v: (self.v >> 1) | (self.v.shift_elements_right::<1>(0) << 7) }
        }

        pub(super) fn and(self, rhs: U256) -> U256 {
            U256 { v: self.v & rhs.v }
        }

        pub(super) fn or(self, rhs: U256) -> U256 {
            U256 { v: self.v | rhs.v }
        }

        pub(super) fn not(self) -> U256 {
            U256 { v: !self.v }
        }
    }
}
