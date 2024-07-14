//! # Sporifica Virus
//!
//! Brute force solution using a fixed size grid, relying on the properties of the input to never
//! exceed the bounds. Some bit manipulation tricks are used to speeds things up slightly.
use crate::util::grid::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(input: &Grid<u8>) -> usize {
    simulate(input, 10_000, 2)
}

pub fn part2(input: &Grid<u8>) -> usize {
    simulate(input, 10_000_000, 1)
}

fn simulate(input: &Grid<u8>, bursts: usize, delta: usize) -> usize {
    // Assume that the carrier will never go outside the range [0, 512] in both x and y axis.
    // starting at the center (256, 256).
    let full = 512;
    let half = 256;
    // Right, Down, Left, Up
    let offsets = [1, full, 0_usize.wrapping_sub(1), 0_usize.wrapping_sub(full)];

    // Copy input
    let mut grid = vec![1; full * full];
    let offset = half - (input.width / 2) as usize;

    for x in 0..input.width {
        for y in 0..input.height {
            if input[Point::new(x, y)] == b'#' {
                let index = full * (offset + y as usize) + (offset + x as usize);
                grid[index] = 3; // Infected
            }
        }
    }

    let mut index = full * half + half; // Center
    let mut direction = 3; // Up
    let mut result = 0;

    for _ in 0..bursts {
        // Change state by adding either `2` for part one (flipping between clean and infected)
        // or `1` for part two (using all four states).
        // Clean => 1
        // Weakened => 2
        // Infected => 3
        // Flagged => 0
        let current = grid[index] as usize;
        let next = (current + delta) & 0x3;
        grid[index] = next as u8;
        // Infected nodes result in a value of 4 >> 2 = 1, all other nodes result in 0.
        result += (next + 1) >> 2;
        // Change direction by adding an index modulo 4 depending on node type.
        // Clean => 1 + 2 => 3 (left)
        // Weakened => 2 + 2 => 0 (straight)
        // Infected => 3 + 2 => 1 (right)
        // Flagged => 0 + 2 => 2 (reverse)
        direction = (direction + current + 2) & 0x3;
        index = index.wrapping_add(offsets[direction]);
    }

    result
}
