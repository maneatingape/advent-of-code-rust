use self::Direction::*;
use std::ops::{BitAnd, BitAndAssign, BitOr, Not};

const HEIGHT: usize = 210;
const RANGE: std::ops::Range<usize> = 1..(HEIGHT - 1);

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

    fn min_set(&self) -> Option<u32> {
        if self.left != 0 {
            Some(self.left.leading_zeros())
        } else if self.right != 0 {
            Some(128 + self.right.leading_zeros())
        } else {
            None
        }
    }

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
        U256 {
            left: (self.left << 1) | (self.right >> 127),
            right: (self.right << 1),
        }
    }

    fn right_shift(&self) -> U256 {
        U256 {
            left: (self.left >> 1),
            right: (self.left << 127) | (self.right >> 1),
        }
    }
}

impl BitAnd for U256 {
    type Output = U256;

    fn bitand(self, rhs: U256) -> U256 {
        U256 {
            left: self.left & rhs.left,
            right: self.right & rhs.right,
        }
    }
}

impl BitOr for U256 {
    type Output = U256;

    fn bitor(self, rhs: U256) -> U256 {
        U256 {
            left: self.left | rhs.left,
            right: self.right | rhs.right,
        }
    }
}

impl Not for U256 {
    type Output = U256;

    fn not(self) -> U256 {
        U256 {
            left: !self.left,
            right: !self.right,
        }
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

pub fn parse(input: &str) -> Input {
    let offset = 70;
    let raw: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let default = [U256::default(); HEIGHT];
    let mut grid = default.clone();

    for (y, row) in raw.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == b'#' {
                grid[offset + y].bit_set(offset + x);
            }
        }
    }

    Input {
        grid,
        north: default.clone(),
        south: default.clone(),
        west: default.clone(),
        east: default.clone(),
    }
}

pub fn part1(input: &Input) -> u32 {
    let mut input = input.clone();
    let mut order = [North, South, West, East];

    for _ in 0..10 {
        step(&mut input, &mut order);
    }

    let grid = input.grid;
    let elves: u32 = grid.iter().map(|r| r.count_ones()).sum();
    let min_x = grid.iter().flat_map(|r| r.min_set()).min().unwrap();
    let max_x = grid.iter().flat_map(|r| r.max_set()).max().unwrap();
    let min_y = grid.iter().position(|r| r.non_zero()).unwrap() as u32;
    let max_y = grid.iter().rposition(|r| r.non_zero()).unwrap() as u32;

    (max_x - min_x + 1) * (max_y - min_y + 1) - elves
}

pub fn part2(input: &Input) -> u32 {
    let mut input = input.clone();
    let mut order = [North, South, West, East];
    let mut moved = true;
    let mut count = 0;

    while moved {
        moved = step(&mut input, &mut order);
        count += 1
    }

    count
}

fn step(input: &mut Input, order: &mut [Direction]) -> bool {
    let Input { grid, north, south, west, east } = input;
    let mut moved = false;

    let mut prev;
    let mut cur = !(grid[0].right_shift() | grid[0] | grid[0].left_shift());
    let mut next = !(grid[1].right_shift() | grid[1] | grid[1].left_shift());

    for i in RANGE {
        prev = cur;
        cur = next;
        next = !(grid[i + 1].right_shift() | grid[i + 1] | grid[i + 1].left_shift());

        let mut up = prev;
        let mut down = next;
        let horizontal = !(grid[i - 1] | grid[i] | grid[i + 1]);
        let mut left = horizontal.right_shift();
        let mut right = horizontal.left_shift();
        let mut remaining = grid[i] & !(up & down & left & right);

        for direction in order.iter() {
            match direction {
                North => {
                    up &= remaining;
                    remaining &= !up;
                },
                South => {
                    down &= remaining;
                    remaining &= !down;
                },
                West => {
                    left &= remaining;
                    remaining &= !left;
                },
                East => {
                    right &= remaining;
                    remaining &= !right;
                },
            }
        }

        north[i - 1] = up;
        south[i + 1] = down;
        west[i] = left.left_shift();
        east[i] = right.right_shift();
    }

    for i in RANGE {
        let up = north[i];
        let down = south[i];
        let left = west[i];
        let right = east[i];
        north[i] &= !down;
        south[i] &= !up;
        west[i] &= !right;
        east[i] &= !left;
    }

    for i in RANGE {
        let same = grid[i] & !(north[i - 1] | south[i + 1] | west[i].right_shift() | east[i].left_shift());
        let change = north[i] | south[i] | west[i] | east[i];
        grid[i] = same | change;
        moved |= change.non_zero();
    }

    order.rotate_left(1);
    moved
}
