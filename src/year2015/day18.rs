//! # Like a GIF For Your Yard
//!
//! To solve this efficiently we use a [SWAR](https://en.wikipedia.org/wiki/SWAR) approach,
//! packing 16 lights into a `u64` taking 4 bits each. We calculate the next generation using no
//! conditional statements with the following steps.
//!
//! 1. Pack the input bytes into register values that can be represented as hex digits.
//!
//! ```none
//!     #...#    10001
//!     .#.#. => 01010
//!     ###.#    11101
//! ```
//!
//! 2. Add left and right neighbors to each column horizontally, shifting in zeroes at the edge.
//!
//! ```none
//!     11011
//!     11211
//!     23221
//! ```
//!
//! 3. Add 3 rows together to give the total sum including the light itself:
//!
//! ```none
//!     45443
//! ```
//!
//! 4. Subtract the middle row to get neighbors only.
//!
//! ```none
//!     44433
//! ```
//!
//! 5. Apply the rules using only bitwise logic.
//!
//! Consider the binary representation of a 4 bit hex digit.
//! * A cell stays on if it has 2 or 3 neigbours, binary `0010` or binary `0011`.
//! * A cell turns on if it has exactly 3 neighbors, binary `0011`.
//!
//! If we `OR` the neighbor count with the current cell, either `0000` or `0001` then the
//! binary representation of a lit cell will always be `0011`.
//!
//! Labelling the bits `abcd` then the next cell is `!a & !b & c & d`.
type Lights = [[u64; 7]; 100];

/// Pack the lights into 4 bits each in [big-endian order](https://en.wikipedia.org/wiki/Endianness).
pub fn parse(input: &str) -> Lights {
    let mut grid = default();

    for (y, row) in input.lines().enumerate() {
        for (x, col) in row.bytes().enumerate() {
            let index = x / 16;
            let offset = 4 * (15 - (x % 16));
            let bit = (col & 1) as u64;
            grid[y][index] |= bit << offset;
        }
    }

    grid
}

pub fn part1(input: &Lights) -> u32 {
    game_of_life(input, false)
}

pub fn part2(input: &Lights) -> u32 {
    game_of_life(input, true)
}

fn game_of_life(input: &Lights, part_two: bool) -> u32 {
    let mut grid = *input;
    let mut temp = default();
    let mut next = default();

    for _ in 0..100 {
        for y in 0..100 {
            for x in 0..7 {
                // Add left and right neighbors from this block.
                let mut sum = grid[y][x] + (grid[y][x] >> 4) + (grid[y][x] << 4);

                // Add immediate right or left neighbor from previous or next block.
                if x > 0 {
                    sum += grid[y][x - 1] << 60;
                }
                if x < 6 {
                    sum += grid[y][x + 1] >> 60;
                }

                temp[y][x] = sum;
            }
        }

        for y in 0..100 {
            for x in 0..7 {
                // Get neighbor count by summing the rows above and below the light
                // then subtracting the light itself.
                let mut sum = temp[y][x] - grid[y][x];

                if y > 0 {
                    sum += temp[y - 1][x];
                }
                if y < 99 {
                    sum += temp[y + 1][x];
                }

                // Calculate the next generation with no conditional statements.
                let a = sum >> 3;
                let b = sum >> 2;
                let c = sum >> 1;
                let d = sum | grid[y][x];

                next[y][x] = (!a & !b & c & d) & 0x1111111111111111;
            }

            // 100 = 16 * 6 + 4 = so only use the first 4 places of the last element.
            next[y][6] &= 0x1111000000000000;
        }

        // Set corner lights to always on.
        if part_two {
            next[0][0] |= 1 << 60;
            next[0][6] |= 1 << 48;
            next[99][0] |= 1 << 60;
            next[99][6] |= 1 << 48;
        }

        (grid, next) = (next, grid);
    }

    grid.iter().map(|row| row.iter().map(|n| n.count_ones()).sum::<u32>()).sum()
}

fn default() -> Lights {
    [[0; 7]; 100]
}
