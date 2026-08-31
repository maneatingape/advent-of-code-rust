//! # Point of Incidence
//!
//! We store each row of a grid as a binary number. For example `#.##..##.` becomes `101100110`.
//! Then to count smudges we bitwise XOR the respective rows together and count one bits
//! using the [`count_ones`] function.
//!
//! For example:
//! ```none
//!  ..##..###     001100111 ^ 000100111 = 00100000 => 1
//! v#####.##.v => 111110110 ^ 111110110 = 00000000 => 0
//! ^#####.##.^
//!  ...#..###
//! ```
//!
//! To handle columns we transpose the grid then convert into integers the same way. For part one
//! we look for a reflection axis with 0 smudges and for part two 1 smudge, allowing the same
//! code to be reused.
//!
//! [`count_ones`]: u32::count_ones
type Input = Vec<(Vec<u32>, Vec<u32>)>;

pub fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|block| {
            let grid: Vec<_> = block.lines().map(str::as_bytes).collect();
            let (width, height) = (grid[0].len(), grid.len());
            let bit = |x: usize, y: usize| u32::from(grid[y][x] == b'#');

            let rows =
                (0..height).map(|y| (0..width).fold(0, |n, x| (n << 1) | bit(x, y))).collect();
            let columns =
                (0..width).map(|x| (0..height).fold(0, |n, y| (n << 1) | bit(x, y))).collect();

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
            reflect_axis(columns, target)
                .unwrap_or_else(|| 100 * reflect_axis(rows, target).unwrap())
        })
        .sum()
}

fn reflect_axis(axis: &[u32], target: u32) -> Option<usize> {
    let size = axis.len();

    (1..size).find(|&i| {
        // Only consider rows/columns within the boundary of the grid.
        let smudges: u32 =
            (0..i.min(size - i)).map(|j| (axis[i - j - 1] ^ axis[i + j]).count_ones()).sum();

        smudges == target
    })
}
