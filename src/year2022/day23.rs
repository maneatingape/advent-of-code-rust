//! # Unstable Diffusion
//!
//! We represent elves as bits in a integer then use bitwise operations to efficiently figure
//! out the movement for multiple elves at once.
use self::Direction::*;
use std::ops::{BitAnd, BitAndAssign, BitOr, Not};

/// The initial grid is 70 x 70. Elves stop moving when no other elf is adjacent so the grid
/// will expand at most 70 in any direction, giving 70 + 70 + 70 = 210 total.
const HEIGHT: usize = 210;

/// Duct tape two `u128`s together.
#[derive(Clone, Copy, Default)]
pub struct U256 {
    left: u128,
    right: u128,
}

impl U256 {
    fn bit_set(&mut self, offset: usize) {
        if offset < 128 {
            self.left |= 1 << (127 - offset);
        } else {
            self.right |= 1 << (255 - offset);
        }
    }

    fn count_ones(&self) -> u32 {
        self.left.count_ones() + self.right.count_ones()
    }

    fn non_zero(&self) -> bool {
        self.left != 0 || self.right != 0
    }

    /// Used to find the bounding rectangle for part one.
    fn min_set(&self) -> Option<u32> {
        if self.left != 0 {
            Some(self.left.leading_zeros())
        } else if self.right != 0 {
            Some(128 + self.right.leading_zeros())
        } else {
            None
        }
    }

    /// Used to find the bounding rectangle for part one.
    fn max_set(&self) -> Option<u32> {
        if self.right != 0 {
            Some(255 - self.right.trailing_zeros())
        } else if self.left != 0 {
            Some(127 - self.left.trailing_zeros())
        } else {
            None
        }
    }

    fn left_shift(&self) -> U256 {
        U256 { left: (self.left << 1) | (self.right >> 127), right: (self.right << 1) }
    }

    fn right_shift(&self) -> U256 {
        U256 { left: (self.left >> 1), right: (self.left << 127) | (self.right >> 1) }
    }
}

/// Syntactic sugar to provide the regular `&`, `|` and `!` bitwise operator notation.
impl BitAnd for U256 {
    type Output = U256;

    fn bitand(self, rhs: U256) -> U256 {
        U256 { left: self.left & rhs.left, right: self.right & rhs.right }
    }
}

impl BitOr for U256 {
    type Output = U256;

    fn bitor(self, rhs: U256) -> U256 {
        U256 { left: self.left | rhs.left, right: self.right | rhs.right }
    }
}

impl Not for U256 {
    type Output = U256;

    fn not(self) -> U256 {
        U256 { left: !self.left, right: !self.right }
    }
}

impl BitAndAssign for U256 {
    fn bitand_assign(&mut self, rhs: U256) {
        self.left &= rhs.left;
        self.right &= rhs.right;
    }
}

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
                grid[offset + y].bit_set(offset + x);
            }
        }
    }

    Input { grid, north: default, south: default, west: default, east: default }
}

pub fn part1(input: &Input) -> u32 {
    let mut input = *input;
    let mut order = [North, South, West, East];

    for _ in 0..10 {
        step(&mut input, &mut order);
    }

    // Find the bounding rectangle.
    let grid = input.grid;
    let elves: u32 = grid.iter().map(U256::count_ones).sum();
    let min_x = grid.iter().filter_map(U256::min_set).min().unwrap();
    let max_x = grid.iter().filter_map(U256::max_set).max().unwrap();
    let min_y = grid.iter().position(U256::non_zero).unwrap() as u32;
    let max_y = grid.iter().rposition(U256::non_zero).unwrap() as u32;

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
    let mut cur = !(grid[0].right_shift() | grid[0] | grid[0].left_shift());
    let mut next = !(grid[1].right_shift() | grid[1] | grid[1].left_shift());

    for i in start..end {
        // Calculating neighbors is relatively expensive so re-use results between rows.
        prev = cur;
        cur = next;
        next = !(grid[i + 1].right_shift() | grid[i + 1] | grid[i + 1].left_shift());

        let mut up = prev;
        let mut down = next;
        // Find neighours in vertical columns.
        let vertical = !(grid[i - 1] | grid[i] | grid[i + 1]);
        let mut left = vertical.right_shift();
        let mut right = vertical.left_shift();
        // Elves need at least 1 neighbor to propose moving.
        let mut remaining = grid[i] & !(up & down & left & right);

        // Consider each direction one at a time, removing any elves who propose it.
        for direction in &*order {
            match direction {
                North => {
                    up &= remaining;
                    remaining &= !up;
                }
                South => {
                    down &= remaining;
                    remaining &= !down;
                }
                West => {
                    left &= remaining;
                    remaining &= !left;
                }
                East => {
                    right &= remaining;
                    remaining &= !right;
                }
            }
        }

        // Copy final proposals to an array for each direction.
        north[i - 1] = up;
        south[i + 1] = down;
        west[i] = left.left_shift();
        east[i] = right.right_shift();
    }

    // Elves that propose moving to the same spot cancel each other out and no-one moves.
    // Due to the movement rules we only need to check horizontal and vertical movement into
    // the same spot (horizontal and vertical movement can never collide with each other).
    for i in start..end {
        let up = north[i];
        let down = south[i];
        let left = west[i];
        let right = east[i];
        north[i] &= !down;
        south[i] &= !up;
        west[i] &= !right;
        east[i] &= !left;
    }

    for i in start..end {
        // Stationary elves.
        let same =
            grid[i] & !(north[i - 1] | south[i + 1] | west[i].right_shift() | east[i].left_shift());
        // Moving elves.
        let change = north[i] | south[i] | west[i] | east[i];
        grid[i] = same | change;
        moved |= change.non_zero();
    }

    // Rotate the order of movement proposals for the next turn.
    order.rotate_left(1);
    moved
}
