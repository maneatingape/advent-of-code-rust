//! # Memory Reallocation
//!
//! Looking at the input and the reallocation rules, when there is at most one bank with
//! 15 blocks, it is easy to see that no other bank will exceed 15 after reallocation.
//! However, some input files have early scenarios where there are a couple of banks with
//! 15, which can result in the next round or two needing to process 16 or even 17 blocks.
//! The overflow issue only occurs early; in the long run, the banks settle into a pattern
//! where overflow does not interfere.
//!
//! With that in mind, it is possible to design a compact layout that stores each bank in
//! one nibble of a u64 in the common case, but with manual iterations managing the overflow
//! as needed.
//!
//! For all but the few manual overflow cases, it is very fast to find the highest nibble
//! using bitwise logic. To detect the cycle a [`FastMap`] stores each previously seen
//! memory layout along with the cycle that it first appeared.
use crate::util::hash::*;
use crate::util::parse::*;

type Input = (u32, u32);

/// Reallocate a bank and set to zero by rotating this mask the correct number of bits.
const REMOVE: u64 = 0x0fffffffffffffff;
/// The highest number of banks when overflow is not a concern is 15, so each bank will
/// add at most 1 to each of the banks that come after it.
const SPREAD: [u64; 16] = [
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
    let mut memory: u64 = input.iter_unsigned::<u64>().fold(0, |acc, n| (acc << 4) + n);
    // Store previously seen configurations for cycle detection.
    let mut seen = FastMap::with_capacity(20_000);
    let mut cycles = 0;

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
        let max = (memory.rotate_left(offset + 4) & 0xf) as usize;

        // Common case: no overflow
        if max < 15 || mask.count_ones() == 1 {
            // Empty the largest memory bank and reallocate its contents to the following banks.
            memory = (memory & REMOVE.rotate_right(offset)) + SPREAD[max].rotate_right(offset);
            cycles += 1;

            // Check if we've seen this configuration before
            if let Some(previous) = seen.insert(memory, cycles) {
                break (cycles, cycles - previous);
            }
        } else {
            // Overflow case.  This can happen in the early steps of the system, but resolves
            // fairly quickly - in practice, the worst known input file had a total of 10 overflow
            // cycles, with at most two adjacent overflows per encounter, with all overflows
            // before cycle 200, well before the first repeated configuration.  Thus, it is
            // okay to not cache these states in seen.
            let mut array = [0; 16];

            for i in (0..16).rev() {
                array[i] = memory & 0xf;
                memory >>= 4;
            }

            while array.iter().any(|&n| n >= 15) {
                let max = *array.iter().max().unwrap();
                let first = array.iter().position(|&n| n == max).unwrap();

                array[first] = 0;
                (0..max as usize).for_each(|i| array[(first + i + 1) % 16] += 1);

                cycles += 1;
            }

            memory = array.iter().fold(0, |acc, n| (acc << 4) | n);
        }
    }
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
