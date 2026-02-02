//! # Like a GIF For Your Yard
//!
//! To solve this efficiently we use a [SWAR](https://en.wikipedia.org/wiki/SWAR) approach,
//! packing 50 lights into a `u64` taking 1 bit each, drawing inspiration from a solution from
//! [u/terje_wiig_mathisen/](https://github.com/TerjeWiigMathisen/aoc-2015-day18).
//!
//! But rather than taking the bits consecutively,
//! split the odd columns into one int and the even into another - that way, when it is time to
//! check neighbors, all left and right neighbors of the odd lanes are already available with just
//! one shift of all the even lanes, and vice versa.  We calculate the next generation using no
//! conditional statements with the following steps.
//!
//! 1. Pack the input bytes into register values that can be represented as binary digits, split
//!    into odd and even lanes.  An extra empty row at the bottom reduces later special-casing.
//!
//! ```none
//!                    .even .odd
//!             column: 024   135
//!     #...#.          101   000
//!     .#.##. =>       001   110
//!     ###.#.          111   100
//! ```
//!
//! 2. Perform half-adder and full-adder computation of each bit with its vertical neighbors, using
//!    only bitwise logic.  Just two bit-wise additions provides data for three 2-bit column sums,
//!    since the left and right neighbors are one bit apart in the opposite parity integer and
//!    already added in parallel.  Visually, for cell e, we are computing a+b+c, d+f, and g+h+i
//!    of its neighbors.
//!
//! ```none
//!                   .odd   .even
//!     ..adg..       ..d..  ..ag..
//!     ..beh..  =>   ..e..  ..bh..
//!     ..cfi..       ..f..  ..ci..
//!                 l,m=d+f  j,k=a+b+c  n,o=g+h+i
//! ```
//!
//! 3. Taking the 3 two-bit column sums learned in the last step, perform two more full-adders to
//!    form four new bits p, q, r, s, which we could add into a usual four-bit number, but which
//!    are good enough for our needs as-is.  Bit s is set if there were an odd number of neighbors;
//!    bit p must be clear or we already know there are more than 3 neighbors; and exactly one of
//!    bits q and r must be set for the final 4-bit sum to have the second bit set.
//!
//! ```none
//!     a d g
//!     b - h
//!   + c f i
//!   -------
//!       j k
//!       l m
//!   +   n o
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
//! Using the bits as labelled above, the next cell is `(e|s) & (q^r) & !p`, masked back
//! to the 50 bits per integer.

const MASK: u64 = (1 << 50) - 1;
const LEFT_CORNER: u64 = 1 << 49;
const RIGHT_CORNER: u64 = 1 << 0;

/// 100 lights of a single row, split by column parity.
#[derive(Clone, Copy, Default)]
pub struct Row {
    even: u64,
    odd: u64,
}

/// Pack the lights into 1 bit each in [big-endian order](https://en.wikipedia.org/wiki/Endianness).
pub fn parse(input: &str) -> Vec<Row> {
    let mut grid = Vec::with_capacity(101);
    let pack = |acc, b| (acc << 1) | u64::from(b & 1);

    for line in input.lines() {
        let even = line.bytes().step_by(2).fold(0, pack);
        let odd = line.bytes().skip(1).step_by(2).fold(0, pack);
        grid.push(Row { even, odd });
    }

    grid.push(Row::default());
    grid
}

pub fn part1(input: &[Row]) -> u32 {
    game_of_life(input, false)
}

pub fn part2(input: &[Row]) -> u32 {
    game_of_life(input, true)
}

fn game_of_life(input: &[Row], part_two: bool) -> u32 {
    let mut grid = input.to_vec();

    for _ in 0..100 {
        let mut above;
        let mut row = Row::default();
        let mut below = grid[0];

        for y in 0..100 {
            // Advance row
            (above, row, below) = (row, below, grid[y + 1]);

            // Even columns.
            let (j, k) = full_adder(above.odd, row.odd, below.odd);
            let (l, m) = half_adder(above.even, below.even);
            let (n, o) = (j >> 1, k >> 1);

            let (p, q) = full_adder(j, l, n);
            let (r, s) = full_adder(k, m, o);
            grid[y].even = (grid[y].even | s) & (q ^ r) & !p & MASK;

            // Odd columns.
            let (j, k) = full_adder(above.even, row.even, below.even);
            let (l, m) = half_adder(above.odd, below.odd);
            let (n, o) = (j << 1, k << 1);

            let (p, q) = full_adder(j, l, n);
            let (r, s) = full_adder(k, m, o);
            grid[y].odd = (grid[y].odd | s) & (q ^ r) & !p & MASK;
        }

        // Set corner lights to always on.
        if part_two {
            grid[0].even |= LEFT_CORNER;
            grid[0].odd |= RIGHT_CORNER;
            grid[99].even |= LEFT_CORNER;
            grid[99].odd |= RIGHT_CORNER;
        }
    }

    grid.iter().map(|row| row.even.count_ones() + row.odd.count_ones()).sum()
}

#[inline]
fn half_adder(a: u64, b: u64) -> (u64, u64) {
    (a & b, a ^ b)
}

#[inline]
fn full_adder(a: u64, b: u64, c: u64) -> (u64, u64) {
    (a & b | c & (a ^ b), a ^ b ^ c)
}
