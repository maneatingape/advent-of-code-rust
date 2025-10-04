//! # Point of Incidence
//!
//! We store each row of a grid as a binary number. For example `#.##..##.` becomes `101100110`.
//! Then to count smudges we bitwise XOR the respective rows together and count one bits
//! using the [`count_ones`] function.
//!
//! For example:
//! ```none
//!     ..##..###     001100111 ^ 000100111 = 00100000 => 1
//!    v#####.##.v => 111110110 ^ 111110110 = 00000000 => 0
//!    ^#####.##.^
//!     ...#..###
//! ````
//!
//! To handle columns we transpose the grid then convert into integers the same way. For part one
//! we look for a reflection axis with 0 smudges and for part two 1 smudge, allowing the same
//! code to be reused.
//!
//! [`count_ones`]: u32::count_ones
use crate::util::grid::*;
use crate::util::point::*;

type Input = Vec<(Vec<u32>, Vec<u32>)>;

pub fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|block| {
            let grid: Grid<_> = Grid::parse(block);
            let mut rows = Vec::with_capacity(grid.height as usize);
            let mut columns = Vec::with_capacity(grid.width as usize);

            for y in 0..grid.height {
                let mut n = 0;

                for x in 0..grid.width {
                    n = (n << 1) | (grid[Point::new(x, y)] == b'#') as u32;
                }

                rows.push(n);
            }

            for x in 0..grid.width {
                let mut n = 0;

                for y in 0..grid.height {
                    n = (n << 1) | (grid[Point::new(x, y)] == b'#') as u32;
                }

                columns.push(n);
            }

            (rows, columns)
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    reflect(input, 0)
}

pub fn part2(input: &Input) -> usize {
    reflect(input, 1)
}

fn reflect(input: &Input, target: u32) -> usize {
    input
        .iter()
        .map(|(rows, columns)| {
            if let Some(x) = reflect_axis(columns, target) {
                x
            } else if let Some(y) = reflect_axis(rows, target) {
                100 * y
            } else {
                unreachable!()
            }
        })
        .sum()
}

fn reflect_axis(axis: &[u32], target: u32) -> Option<usize> {
    let size = axis.len();

    (1..size).find(|&i| {
        let mut smudges = 0;

        // Only consider rows/columns within the boundary of the grid.
        for j in 0..i.min(size - i) {
            smudges += (axis[i - j - 1] ^ axis[i + j]).count_ones();
        }

        smudges == target
    })
}
