//! # Hydrothermal Venture
//!
//! No subtlety with this solution, we create an 1 dimensional arrray of 1 million `u8` elements
//! to store all possible points then increment values for each line. This assumes that no lines
//! cross more than 255 times. This approach is much faster but less flexible than using a
//! `HashMap` to store mappings of point to values.
//!
//! To avoid the overhead of a nested 2 dimensional array, each point `(x, y)` is mapped to
//! an index `y * 1000 + x`. For each line direction the index delta is calculated using
//! the handy [`signum`] function.
//!
//! [`signum`]: i32::signum
use crate::util::iter::*;
use crate::util::parse::*;

type Vent = [u32; 4];

pub fn parse(input: &str) -> [usize; 2] {
    let all: Vec<_> = input.iter_unsigned().chunk::<4>().collect();
    let (orthogonal, diagonal): (Vec<_>, Vec<_>) =
        all.iter().partition(|[x1, y1, x2, y2]| x1 == x2 || y1 == y2);

    let mut grid = vec![0_u8; 1_000_000];
    let first = vents(&orthogonal, &mut grid);
    let second = vents(&diagonal, &mut grid);

    [first, second]
}

pub fn part1(input: &[usize]) -> usize {
    input[0]
}

pub fn part2(input: &[usize]) -> usize {
    input[0] + input[1]
}

fn vents(input: &[Vent], grid: &mut [u8]) -> usize {
    let mut result = 0;

    for &[x1, y1, x2, y2] in input {
        let (x1, y1, x2, y2) = (x1 as i32, y1 as i32, x2 as i32, y2 as i32);
        let count = (y2 - y1).abs().max((x2 - x1).abs());
        let delta = (y2 - y1).signum() * 1000 + (x2 - x1).signum();
        let mut index = y1 * 1000 + x1;

        for _ in 0..=count {
            if grid[index as usize] == 1 {
                result += 1;
            }
            grid[index as usize] += 1;
            index += delta;
        }
    }

    result
}
