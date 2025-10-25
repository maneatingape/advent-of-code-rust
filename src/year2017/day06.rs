//! # Memory Reallocation
//!
//! Looking at the input and the reallocation rules, we make an assertion:
//!
//! * No memory bank will ever exceed 15 blocks.
//!
//! This has a nice effect that we can store the entire memory layout packed into a single
//! `u64` with each memory bank represented by a nibble.
//!
//! This makes it very fast to find the highest nibble using bitwise logic. To detect the cycle
//! a [`FastMap`] stores each previously seen memory layout along with the cycle that it first
//! appeared.
use crate::util::hash::*;
use crate::util::parse::*;

type Input = (u32, u32);

/// Reallocate a bank and set to zero by rotating this mask the correct number of bits.
const REMOVE: usize = 0x0fffffffffffffff;
/// The highest number of banks possible is 15, so each bank will add at most 1 to each of
/// the banks that come after it.
const SPREAD: [usize; 16] = [
    0x0000000000000000,
    0x0100000000000000,
    0x0110000000000000,
    0x0111000000000000,
    0x0111100000000000,
    0x0111110000000000,
    0x0111111000000000,
    0x0111111100000000,
    0x0111111110000000,
    0x0111111111000000,
    0x0111111111100000,
    0x0111111111110000,
    0x0111111111111000,
    0x0111111111111100,
    0x0111111111111110,
    0x0111111111111111,
];

pub fn parse(input: &str) -> Input {
    // Accumulate the input into a single `u64`.
    let mut memory: usize = input.iter_unsigned::<usize>().fold(0, |acc, n| (acc << 4) + n);
    // Store previously seen configurations for cycle detection.
    let mut seen = FastMap::with_capacity(20_000);
    let mut cycles = 0;

    seen.insert(memory, cycles);

    loop {
        // Find the highest nibble in the integer.
        // We check each of the 4 bits for all nibbles in descending order by bitwise ANDing with
        // the mask.
        // If the mask is zero, then this implies that no nibbles have that bit set so we leave
        // the mask unchanged.
        // If some nibbles have that bit set, then we will "narrow" the mask to only consider
        // those nibbles.
        let mask = (0..4).fold(0x8888888888888888, |mask, shift| {
            let result = (memory << shift) & mask;
            if result == 0 { mask } else { result }
        });

        // The mask will have a 1 bit set for each of the joint highest values.
        // Choose the lowest index which is the most significant bit set.
        let offset = mask.leading_zeros();
        let max = memory.rotate_left(offset + 4) & 0xf;

        // Empty the largest memory bank and reallocate its contents to the following banks.
        memory = (memory & REMOVE.rotate_right(offset)) + SPREAD[max].rotate_right(offset);
        cycles += 1;

        // Check if we've seen this configuration before
        if let Some(previous) = seen.insert(memory, cycles) {
            break (cycles, cycles - previous);
        }
    }
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
