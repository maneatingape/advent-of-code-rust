//! # Planet of Discord
//!
//! ## Part One
//!
//! The "biodiversity rating" is a very strong hint to store the 5x5 grid as bits in an integer.
//! To make calculating the rating a no-op, we store the grid:
//!
//! ```none
//!     abcde
//!     fghij
//!     klmno
//!     pqrst
//!     uvwxy
//! ```
//!
//! packed into a `u32` as `yxwvutsrqponmlkjihgfedcba`.
//!
//! Then for each position we create a bitmask for up to 4 potential neighbors.
//! For example the bitmask for position `a` is `100010` and for position `h` is `1000101000100`.
//!
//! For each generation we bitwise `AND` each position with its mask, then use the [`count_ones`]
//! intrinsic to efficiently find the number of neighbors.
//!
//! ## Part Two
//!
//! The multiple levels can be represented as an array, the outer layer as the previous element
//! and the inner layer as the next. We add two more bitmasks for the outer and inner layers
//! respectively, then calculate the total sum for the 16 tiles on the outer edges with the
//! previous layer and the 9 inner tiles with the next layer. The center tile is a no-op and
//! always zero.
//!
//! [`count_ones`]: u32::count_ones
use crate::util::hash::*;

const LEVEL: [u32; 25] = [
    0b0000000000000000000100010,
    0b0000000000000000001000101,
    0b0000000000000000010001010,
    0b0000000000000000100010100,
    0b0000000000000001000001000,
    0b0000000000000010001000001,
    0b0000000000000100010100010,
    0b0000000000001000101000100,
    0b0000000000010001010001000,
    0b0000000000100000100010000,
    0b0000000001000100000100000,
    0b0000000010001010001000000,
    0b0000000100010100010000000,
    0b0000001000101000100000000,
    0b0000010000010001000000000,
    0b0000100010000010000000000,
    0b0001000101000100000000000,
    0b0010001010001000000000000,
    0b0100010100010000000000000,
    0b1000001000100000000000000,
    0b0001000001000000000000000,
    0b0010100010000000000000000,
    0b0101000100000000000000000,
    0b1010001000000000000000000,
    0b0100010000000000000000000,
];

const OUTER: [u32; 25] = [
    0b0000000000000100010000000,
    0b0000000000000000010000000,
    0b0000000000000000010000000,
    0b0000000000000000010000000,
    0b0000000000010000010000000,
    0b0000000000000100000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000010000000000000,
    0b0000000000000100000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000010000000000000,
    0b0000000000000100000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000010000000000000,
    0b0000000100000100000000000,
    0b0000000100000000000000000,
    0b0000000100000000000000000,
    0b0000000100000000000000000,
    0b0000000100010000000000000,
];

const INNER: [u32; 25] = [
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000011111,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000100001000010000100001,
    0b0000000000000000000000000,
    0b1000010000100001000010000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b1111100000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
    0b0000000000000000000000000,
];

/// Parse the initial grid, placing the top left bug into the least significant bit of the result.
pub fn parse(input: &str) -> u32 {
    input
        .bytes()
        .rev()
        .filter(|b| !b.is_ascii_whitespace())
        .fold(0, |acc, b| (acc << 1) | (b & 1) as u32)
}

pub fn part1(input: &u32) -> u32 {
    let mut grid = *input;
    let mut seen = FastSet::new();

    // `insert` returns false if the element is already present
    while seen.insert(grid) {
        let mut next = 0;

        for (i, level) in LEVEL.iter().enumerate() {
            let mask = 1 << i;
            let bug = grid & mask;
            let adjacent = (grid & level).count_ones();

            if adjacent == 1 || (bug == 0 && adjacent == 2) {
                next |= mask;
            }
        }

        grid = next;
    }

    grid
}

pub fn part2(input: &u32) -> u32 {
    part2_testable(input, 200)
}

pub fn part2_testable(input: &u32, minutes: usize) -> u32 {
    // The bugs can expand by at most 1 level during each minute, so we need
    // 1 + 2 * (200 + 1 buffer for convenience) = 403 total.
    let mut start = 200;
    let mut end = 203;
    let mut grid = &mut [0; 403];
    let mut next = &mut [0; 403];

    // Place the initial level exactly in the middle.
    grid[201] = *input;

    for _ in 0..minutes {
        for i in start..end {
            let outer = grid[i - 1];
            let level = grid[i];
            let inner = grid[i + 1];

            let mut acc = 0;

            macro_rules! repeat {
                ($other:ident, $mask:ident, $($i:literal)*) => ($(
                    let mask = 1 << $i;
                    let bug = level & mask;
                    let adjacent = (level & LEVEL[$i]).count_ones()
                        + ($other & $mask[$i]).count_ones();

                    if adjacent == 1 || (bug == 0 && adjacent == 2) {
                        acc |= mask;
                    }
                )*)
            }

            repeat!(outer, OUTER, 0 1 2 3 4 5 9 10 14 15 19 20 21 22 23 24);
            repeat!(inner, INNER, 6 7 8 11 13 16 17 18);
            next[i] = acc;
        }

        // As an optimization only expand if there are bugs in the level.
        if next[start] != 0 {
            start -= 1;
        }
        if next[end - 1] != 0 {
            end += 1;
        }

        (grid, next) = (next, grid);
    }

    grid[start..end].iter().map(|&n| n.count_ones()).sum()
}
