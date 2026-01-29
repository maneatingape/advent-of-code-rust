//! # Like a GIF For Your Yard
//!
//! To solve this efficiently we use a [SWAR](https://en.wikipedia.org/wiki/SWAR) approach,
//! packing 50 lights into a `u64` taking 1 bit each (if the grid were bigger, we ccould use all
//! 64 bits; but with only two integers per row, it is easier to reuse common masks by keeping
//! the integers balanced). We calculate the next generation using no conditional statements with
//! the following steps.
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
//!    and a computed carry2 joined with an implied bit sum2 = sum3^orig represent the sum of
//!    only the two neighbors.
//!
//! ```none
//!     10001              carry3 = 11001 = carry2 | (orig&(above^below))
//!     01011  =>            sum3 = 00111 = above^below^orig
//!     11101              carry2 = 10001 = above&below
//!                (implied) sum2 = 01100 = sum3^orig
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
//! Using the bits as labelled above, the next cell is `(orig|s) & (q^r) & !p`.
type Lights = [[u64; 2]; 102];

/// Since rows are 100 lights wide, it's easier to just uniformly split between two u64.
const CELLS_PER_INT: usize = 100 / 2;

/// Pack the lights into 1 bit each in [big-endian order](https://en.wikipedia.org/wiki/Endianness).
pub fn parse(input: &str) -> Lights {
    let mut grid = default();

    // Reserve blank row above and below for less special-casing.
    grid[0][0] = 0;
    grid[0][1] = 0;
    for (y, row) in input.lines().enumerate() {
        for (x, col) in row.bytes().enumerate() {
            let index = x / CELLS_PER_INT;
            let offset = (CELLS_PER_INT - 1) - (x % CELLS_PER_INT);
            let bit = (col & 1) as u64;
            grid[y + 1][index] |= bit << offset;
        }
    }
    grid[101][0] = 0;
    grid[101][1] = 0;

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

    for _ in 0..100 {
        for y in 1..101 {
            for x in 0..2 {
                // Compute temporaries from upper and lower neighbors.
                let cell = grid[y][x];
                let above = grid[y - 1][x];
                let below = grid[y + 1][x];
                carry2[y][x] = above & below;
                sum3[y][x] = above ^ cell ^ below;
                carry3[y][x] = carry2[y][x] | (cell & (above ^ below));
            }
        }

        for y in 1..101 {
            for x in 0..2 {
                // Prepare to merge 3 groups of sums of three into a sum of 9.  Shift one bit right
                // to move the left neighbor into the current bit lane; vice versa for the right
                // neighbor; this gets 49 of the 50 bits in place.
                let cell = grid[y][x];
                let mut leftcarry = carry3[y][x] >> 1;
                let mut leftsum = sum3[y][x] >> 1;
                let midcarry = carry2[y][x];
                let midsum = sum3[y][x] ^ cell;
                let mut rightcarry = carry3[y][x] << 1;
                let mut rightsum = sum3[y][x] << 1;

                // Pull in final bit lane from the neighboring cell.
                if x == 0 {
                    rightcarry |= carry3[y][1] >> 49;
                    rightsum |= sum3[y][1] >> 49;
                } else {
                    leftcarry |= carry3[y][0] << 49;
                    leftsum |= sum3[y][0] << 49;
                }

                // Compute p, q, r, s.
                let p = (leftcarry & rightcarry) | (midcarry & (leftcarry ^ rightcarry));
                let q = leftcarry ^ midcarry ^ rightcarry;
                let r = (leftsum & rightsum) | (midsum & (leftsum ^ rightsum));
                let s = leftsum ^ midsum ^ rightsum;

                // Calculate the next generation with no conditional statements.
                // Mask things back to 50 bits.
                grid[y][x] = (cell | s) & (q ^ r) & !p & ((1 << CELLS_PER_INT) - 1);
            }
        }

        // Set corner lights to always on.
        if part_two {
            grid[1][0] |= 1 << (CELLS_PER_INT - 1);
            grid[1][1] |= 1;
            grid[100][0] |= 1 << (CELLS_PER_INT - 1);
            grid[100][1] |= 1;
        }
    }

    grid.iter().flat_map(|row| row.iter()).map(|n| n.count_ones()).sum()
}

fn default() -> Lights {
    [[0; 2]; 102]
}
