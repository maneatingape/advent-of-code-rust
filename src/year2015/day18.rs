//! # Like a GIF For Your Yard
//!
//! To solve this efficiently we use a [SWAR](https://en.wikipedia.org/wiki/SWAR) approach,
//! packing 60 lights into a `u64` taking 1 bit each.  Why 60 bits?  A naive packing of 1 row
//! by 100 bits occupies 2 ints, but requires 2*100 ints to represent the entire grid, and leaves
//! 28 bits per row idle.  But if we rearrange the bits in a 10x6 mini-grid, sharing a portion
//! of 10 different rows in a single int, we now require 17 ints to represent one row, but cut
//! the number of row operations by 10; only 17*10 ints to represent the entire grid, or 15%
//! denser.  Fewer idle bits makes the overall computation faster.  Note that it is easier
//! to create an oversized grid where the borders are all zeros, to reduce the special
//! cases in the inner loop.
//!
//! With the grid layout decided, we calculate the next generation using no conditional statements
//! with the following steps.
//!
//! 1. Pack the input bytes into register values that can be represented as binary digits.  Creating
//!    an extra empty row above and below the grid allows for less special-casing later on.
//!
//! ```none
//!     #...#    10001
//!     .#.## => 01011
//!     ###.#    11101
//! ```
//!
//! 2. Perform a half-adder and full-adder computation of each bit with its vertical neighbors into
//!    3 temporaries, using only bitwise logic, and shifting in zeroes at the edge.  For each row,
//!    the two bits carry3#sum3 represent the 2-digit sum of three bits including the central row,
//!    and a computed carry2 joined with an implied bit sum2 = sum3^cell represent the sum of
//!    only the two neighbors.
//!
//! ```none
//!     10001              carry3 = 11001 = carry2 | (cell&(above^below))
//!     01011  =>            sum3 = 00111 = above^below^cell
//!     11101              carry2 = 10001 = above&below
//!                (implied) sum2 = 01100 = sum3^cell
//! ```
//!
//! 3. Shift values to obtain the horizontal neighbors 1 bit away (or across the integer boundary),
//!    combining 6 bits from the prior adders to form four new bits p, q, r, s, which we could add
//!    into a usual four-bit number, but which is good enough for our needs as-is.  Bit s is set if
//!    there were an odd number of neighbors; bit p must be clear or we already know there are more
//!    than 3 neighbors; and exactly one of bits q and r must be set for the final 4-bit sum to
//!    have the second bit set.
//!
//! ```none
//!     a d g     full-adder(a, b, c) => j, k
//!     b - h  => half-adder(d, f)    => l, m
//!   + c f i     full-adder(g, h, i) => n, o
//!   -------
//!       j k
//!       l m  => full-adder(j, l, n) => p, q
//!   +   n o     full-adder(k, m, o) => r, s
//!   -------
//!     p q -
//!       r s
//! ```
//!
//! 4. Apply the rules using only bitwise logic.
//!
//! Consider the binary representation of a 4 bit hex digit.
//! * A cell stays on if it has 2 or 3 neighbors, binary `0010` or binary `0011`.
//! * A cell turns on if it has exactly 3 neighbors, binary `0011`.
//!
//! If we `OR` the neighbor count with the current cell, either `0000` or `0001` then the
//! binary representation of a lit cell will always be `0011`.
//!
//! Using the bits as labelled above, the next cell is `(cell|s) & (q^r) & !p`.

/// Each part runs the Game of Life for 100 iterations.
const ITERATIONS: usize = 100;

/// Pack a portion of 10 different rows into a single int, with 6 cells over 17 columns per row.
/// Using the notation `0xL~R` as shorthand for `0bLLLL_LLRR_RRRR`, unused bits as U, and row 10
/// shown as A, the resulting 64-bit number is `0xU1~2_3~45_~67~_89~A`.
const CELLS_PER_CHUNK: usize = 6;

/// All other constants used below are derived from the choice above. You can go back to
/// 100 rows of 2 ints, with 1x50 cells per int, by changing `CELLS_PER_CHUNK` to 50.
const CHUNKS_PER_INT: usize = 64 / CELLS_PER_CHUNK;
const CELLS_PER_INT: usize = CHUNKS_PER_INT * CELLS_PER_CHUNK;
const ALL_BITS_MASK: u64 = (1_u64 << CELLS_PER_INT) - 1;
const NEIGHBOR_SHIFT: usize = (CHUNKS_PER_INT - 1) * CELLS_PER_CHUNK; // 54
const BOTTOM_ROW_MASK: u64 = (1 << CELLS_PER_CHUNK) - 1; // All bits from row 10
const TOP_ROW_MASK: u64 = BOTTOM_ROW_MASK << NEIGHBOR_SHIFT; // All bits from row 1
const RIGHT_COL_MASK: u64 = ALL_BITS_MASK / BOTTOM_ROW_MASK; // All bits from column 6
const LEFT_COL_MASK: u64 = RIGHT_COL_MASK << (CELLS_PER_CHUNK - 1); // All bits from column 1
const INTS_PER_ROW: usize = 100_usize.div_ceil(CELLS_PER_CHUNK); // 17
const ROW_END_SLOP: usize = INTS_PER_ROW * CELLS_PER_CHUNK - 100; // 2 bits unused in final column
const ROW_END_MASK: u64 = RIGHT_COL_MASK * (BOTTOM_ROW_MASK - ((1 << ROW_END_SLOP) - 1)); // All bits in last column
const GROUPS_PER_ITERATION: usize = 100_usize.div_ceil(CHUNKS_PER_INT); // 10

type Lights = [[u64; INTS_PER_ROW + 2]; GROUPS_PER_ITERATION + 2];

/// Pack the lights into 1 bit each in [big-endian order](https://en.wikipedia.org/wiki/Endianness).
pub fn parse(input: &str) -> Lights {
    let mut grid = default();

    // Reserve blank int on all four borders: row 0 and 11, column 0 and 18
    for (y, row) in input.lines().enumerate() {
        for (x, col) in row.bytes().enumerate() {
            let index = x / CELLS_PER_CHUNK + 1;
            let offset = ((CELLS_PER_CHUNK - 1) - (x % CELLS_PER_CHUNK))
                + CELLS_PER_CHUNK * ((CHUNKS_PER_INT - 1) - (y % CHUNKS_PER_INT));
            let bit = (col & 1) as u64;
            grid[y / CHUNKS_PER_INT + 1][index] |= bit << offset;
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
    let mut carry3 = default();
    let mut sum3 = default();
    let mut carry2 = default();

    for _ in 0..ITERATIONS {
        for y in 1..GROUPS_PER_ITERATION + 1 {
            for x in 1..INTS_PER_ROW + 1 {
                // Compute temporaries from upper and lower neighbors.
                let cell = grid[y][x];
                let above = ((cell >> CELLS_PER_CHUNK) | (grid[y - 1][x] << NEIGHBOR_SHIFT))
                    & ALL_BITS_MASK;
                let below = ((cell << CELLS_PER_CHUNK) | (grid[y + 1][x] >> NEIGHBOR_SHIFT))
                    & ALL_BITS_MASK;
                carry2[y][x] = above & below;
                sum3[y][x] = above ^ cell ^ below;
                carry3[y][x] = carry2[y][x] | (cell & (above ^ below));
            }
        }

        for y in 1..GROUPS_PER_ITERATION + 1 {
            for x in 1..INTS_PER_ROW + 1 {
                // Prepare to merge 3 groups of sums of three into a sum of 9.  Mask off the right
                // column and shift cell one bit right to pick up 5 left neighbors per chunk, then
                // merge in the right column shifted 5 bits left of the left neighbor cell to
                // populate the final bit lane.  Vice versa for the right neighbor cells.
                let cell = grid[y][x];
                let leftcarry = ((carry3[y][x] & !RIGHT_COL_MASK) >> 1)
                    | ((carry3[y][x - 1] & RIGHT_COL_MASK) << (CELLS_PER_CHUNK - 1));
                let leftsum = ((sum3[y][x] & !RIGHT_COL_MASK) >> 1)
                    | ((sum3[y][x - 1] & RIGHT_COL_MASK) << (CELLS_PER_CHUNK - 1));
                let midcarry = carry2[y][x];
                let midsum = sum3[y][x] ^ cell;
                let rightcarry = ((carry3[y][x] & !LEFT_COL_MASK) << 1)
                    | ((carry3[y][x + 1] & LEFT_COL_MASK) >> (CELLS_PER_CHUNK - 1));
                let rightsum = ((sum3[y][x] & !LEFT_COL_MASK) << 1)
                    | ((sum3[y][x + 1] & LEFT_COL_MASK) >> (CELLS_PER_CHUNK - 1));

                // Compute p, q, r, s.
                let p = (leftcarry & rightcarry) | (midcarry & (leftcarry ^ rightcarry));
                let q = leftcarry ^ midcarry ^ rightcarry;
                let r = (leftsum & rightsum) | (midsum & (leftsum ^ rightsum));
                let s = leftsum ^ midsum ^ rightsum;

                // Calculate the next generation with no conditional statements.
                // Mask things back to 60 bits.
                grid[y][x] = (cell | s) & (q ^ r) & !p & ALL_BITS_MASK;
            }

            grid[y][INTS_PER_ROW] &= ROW_END_MASK;
        }

        // Set corner lights to always on.
        if part_two {
            grid[1][1] |= LEFT_COL_MASK & TOP_ROW_MASK;
            grid[1][INTS_PER_ROW] |= (RIGHT_COL_MASK & TOP_ROW_MASK) << ROW_END_SLOP;
            grid[GROUPS_PER_ITERATION][1] |= LEFT_COL_MASK & BOTTOM_ROW_MASK;
            grid[GROUPS_PER_ITERATION][INTS_PER_ROW] |=
                (RIGHT_COL_MASK & BOTTOM_ROW_MASK) << ROW_END_SLOP;
        }
    }

    grid.iter().flat_map(|row| row.iter()).map(|n| n.count_ones()).sum()
}

fn default() -> Lights {
    [[0; INTS_PER_ROW + 2]; GROUPS_PER_ITERATION + 2]
}
