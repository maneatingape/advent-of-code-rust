use std::ops::{BitAnd, BitOr, Not};

#[derive(Clone, Copy, Default)]
pub struct U256 {
    left: u128,
    right: u128,
}

impl U256 {
    fn bit_set(&mut self, offset: usize) {
        if offset < 128 {
            self.right |= 1 << offset;
        } else {
            self.left |= 1 << (offset - 128);
        }
    }

    fn non_zero(&self) -> bool {
        self.left != 0 || self.right != 0
    }

    fn left_roll(&self, width: usize) -> U256 {
        if width <= 128 {
            let mask = !(1 << width);
            let right = ((self.right << 1) & mask) | (self.right >> (width - 1));
            U256 { left: self.left, right }
        } else {
            let mask = !(1 << (width - 128));
            let left = ((self.left << 1) & mask) | (self.right >> 127);
            let right = (self.right << 1) | (self.left >> (width - 129));
            U256 { left, right }
        }
    }

    fn right_roll(&self, width: usize) -> U256 {
        if width <= 128 {
            let right = (self.right >> 1) | ((self.right & 1) << (width - 1));
            U256 { left: self.left, right }
        } else {
            let left = (self.left >> 1) | ((self.right & 1) << (width - 129));
            let right = (self.right >> 1) | ((self.left & 1) << 127);
            U256 { left, right }
        }
    }
}

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

#[derive(Clone)]
pub struct State {
    width: usize,
    height: usize,
    across: Vec<U256>,
    down: Vec<U256>,
}

pub fn parse(input: &str) -> State {
    let raw: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let width = raw[0].len();
    let height = raw.len();
    let mut across = Vec::new();
    let mut down = Vec::new();

    for row in raw {
        let mut next_across = U256::default();
        let mut next_down = U256::default();

        for (offset, &col) in row.iter().enumerate() {
            match col {
                b'>' => next_across.bit_set(offset),
                b'v' => next_down.bit_set(offset),
                _ => (),
            }
        }

        across.push(next_across);
        down.push(next_down);
    }

    State { width, height, across, down }
}

pub fn part1(input: &State) -> usize {
    let State { width, height, mut across, mut down } = input.clone();
    let mut changed = true;
    let mut count = 0;

    while changed {
        changed = false;
        count += 1;

        for i in 0..height {
            let candidates = across[i].left_roll(width);
            let moved = candidates & !(across[i] | down[i]);
            changed |= moved.non_zero();
            let stay = across[i] & !moved.right_roll(width);
            across[i] = moved | stay;
        }

        let last_mask = across[0] | down[0];
        let mut moved = down[height - 1] & !last_mask;

        for i in 0..(height - 1) {
            changed |= moved.non_zero();
            let mask = across[i + 1] | down[i + 1];
            let stay = down[i] & mask;
            let next_moved = down[i] & !mask;
            down[i] = moved | stay;
            moved = next_moved;
        }

        changed |= moved.non_zero();
        let stay = down[height - 1] & last_mask;
        down[height - 1] = moved | stay;
    }

    count
}

pub fn part2(_input: &State) -> &'static str {
    "n/a"
}
