//! # Settlers of The North Pole
//!
//! This problem is a cellular automaton similar to the well known
//! [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life). To solve part two
//! we look for a [cycle](https://en.wikipedia.org/wiki/Cycle_detection) then
//! extrapolate forward a billion generations.
//!
//! To efficiently compute the next generation a [SWAR](https://en.wikipedia.org/wiki/SWAR)
//! approach is used. The count of trees and lumberyards is packed into a `u64` so that we can
//! process 8 acres at a time. Lumberyards are stored in the high nibble of each byte
//! and trees in the low nibble. For example:
//!
//! ```none
//!     .#.#...|
//!     .....#|# => 11 11 21 11 21 02 21 02 => 0x1111211121022102
//!     .|..|...
//! ```
//!
//! The total number of adjacent trees or lumberyards is then calculated in two passes.
//! First the horizontal sum of each row is computed by bit shifting left and right by 8.
//! Then the vertical sum of 3 horizontal sums gives the total.
//!
//! Bitwise logic then computes the next generation in batches of 8 acres at a time.
use crate::util::hash::*;
use std::hash::{Hash, Hasher};

/// Bitwise logic galore.
const OPEN: u64 = 0x00;
const TREE: u64 = 0x01;
const LUMBERYARD: u64 = 0x10;
const EDGE: u64 = 0xffff000000000000;
const LOWER: u64 = 0x0f0f0f0f0f0f0f0f;
const UPPER: u64 = 0xf0f0f0f0f0f0f0f0;
const THIRTEENS: u64 = 0x0d0d0d0d0d0d0d0d;
const FIFTEENS: u64 = 0x0f0f0f0f0f0f0f0f;

/// New type wrapper so that we can use a custom hash function.
#[derive(PartialEq, Eq)]
pub struct Key {
    area: [u64; 350],
}

/// Hash only two cells as a reasonable tradeoff between speed and collision resistance.
impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.area[100].hash(state);
        self.area[200].hash(state);
    }
}

/// Pack the input into an array of `u64`.
/// The input is 50 acres wide, so requires `ceil(50 / 8) = 7` elements for each row.
pub fn parse(input: &str) -> Key {
    let mut area = [0; 350];

    for (y, line) in input.lines().map(str::as_bytes).enumerate() {
        for (x, byte) in line.iter().enumerate() {
            let acre = match byte {
                b'|' => TREE,
                b'#' => LUMBERYARD,
                _ => OPEN,
            };
            let index = (y * 7) + (x / 8);
            let offset = 56 - 8 * (x % 8);
            area[index] |= acre << offset;
        }
    }

    Key { area }
}

/// Compute 10 generations.
pub fn part1(input: &Key) -> u32 {
    let mut area = input.area;
    let mut rows = [0; 364];

    for _ in 0..10 {
        step(&mut area, &mut rows);
    }

    resource_value(&area)
}

/// Compute generations until a cycle is detected.
pub fn part2(input: &Key) -> u32 {
    let mut area = input.area;
    let mut rows = [0; 364];
    let mut seen = FastMap::with_capacity(1_000);

    for minute in 1.. {
        step(&mut area, &mut rows);

        if let Some(previous) = seen.insert(Key { area }, minute) {
            // Find the index of the state after 1 billion repetitions.
            let offset = 1_000_000_000 - previous;
            let cycle_width = minute - previous;
            let remainder = offset % cycle_width;
            let target = previous + remainder;

            let (result, _) = seen.iter().find(|&(_, &i)| i == target).unwrap();
            return resource_value(&result.area);
        }
    }

    unreachable!()
}

fn step(area: &mut [u64], rows: &mut [u64]) {
    // Compute the horizontal sum of each column with its immediate neighbors.
    for y in 0..50 {
        // Shadow slices at correct starting offset for convenience. We pad `rows` on the top and
        // bottom then shift index by 7 to avoid having to check for edge conditions.
        let area = &area[7 * y..];
        let rows = &mut rows[7 * (y + 1)..];

        rows[0] = horizontal_sum(0, area[0], area[1]);
        rows[1] = horizontal_sum(area[0], area[1], area[2]);
        rows[2] = horizontal_sum(area[1], area[2], area[3]);
        rows[3] = horizontal_sum(area[2], area[3], area[4]);
        rows[4] = horizontal_sum(area[3], area[4], area[5]);
        rows[5] = horizontal_sum(area[4], area[5], area[6]);
        rows[6] = horizontal_sum(area[5], area[6], 0);

        // The grid is 50 wide so the last 6 bytes in each row are unused and must be set to zero.
        rows[6] &= EDGE;
    }

    for i in 0..350 {
        // Sum of all adjacent trees and lumberyards, not including center acre.
        let acre = area[i];
        let sum = rows[i] + rows[i + 7] + rows[i + 14] - acre;

        // Add 13 so that any values 3 and higher overflow into high nibble.
        let mut to_tree = (sum & LOWER) + THIRTEENS;
        // Clear low nibble as this is irrelevant.
        to_tree &= UPPER;
        // To become a tree, we must be open space.
        to_tree &= !(acre | (acre << 4));
        // Shift result back to low nibble.
        to_tree >>= 4;

        // Check for any values 3 or higher.
        let mut to_lumberyard = ((sum >> 4) & LOWER) + THIRTEENS;
        // Clear low nibble.
        to_lumberyard &= UPPER;
        // To become a lumberyard, we must already be a tree.
        to_lumberyard &= acre << 4;
        // Spread result to both high and low nibble. We will later XOR this to flip correct bits.
        to_lumberyard |= to_lumberyard >> 4;

        // We must be a lumberyard.
        let mut to_open = acre & UPPER;
        // Check for at least one adjacent tree.
        to_open &= (sum & LOWER) + FIFTEENS;
        // Check for at least one adjacent lumberyard.
        to_open &= ((sum >> 4) & LOWER) + FIFTEENS;
        // Flip bit as we will later XOR.
        to_open ^= acre & UPPER;

        // Flip relevant bits to transition to next state.
        area[i] = acre ^ (to_tree | to_lumberyard | to_open);
    }
}

/// Convenience method that also takes correct byte from left and right neighbors.
#[inline]
fn horizontal_sum(left: u64, middle: u64, right: u64) -> u64 {
    (left << 56) + (middle >> 8) + middle + (middle << 8) + (right >> 56)
}

/// Each tree or lumberyard is represented by a single bit.
fn resource_value(area: &[u64]) -> u32 {
    let trees: u32 = area.iter().map(|n| (n & LOWER).count_ones()).sum();
    let lumberyards: u32 = area.iter().map(|n| (n & UPPER).count_ones()).sum();
    trees * lumberyards
}
