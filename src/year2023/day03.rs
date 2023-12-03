use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::parse::*;
use crate::util::point::*;

pub struct Input {
    grid: Grid<u8>,
    seen: Grid<usize>,
    numbers: Vec<u32>,
}

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let width = grid.width as usize;

    let mut seen: Grid<usize> = grid.default_copy();
    let mut numbers = vec![0];
    let mut current = 0;

    for (i, b) in grid.bytes.iter().enumerate() {
        if current > 0 && (!b.is_ascii_digit() || i % width == 0) {
            numbers.push(current);
            current = 0;
        }
        if b.is_ascii_digit() {
            seen.bytes[i] = numbers.len();
            current = 10 * current + (b.to_decimal() as u32);
        }
    }

    if current > 0 {
        numbers.push(current);
    }

    Input { grid, seen, numbers }
}

pub fn part1(input: &Input) -> u32 {
    let Input { grid, seen, numbers } = input;
    let mut adjacent = FastSet::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point::new(x, y);
            let b = grid[p];

            if !b.is_ascii_digit() && b != b'.' {
                for next in DIAGONAL.iter().copied().map(|d| p + d) {
                    if seen[next] != 0 {
                        adjacent.insert(seen[next]);
                    }
                }
            }
        }
    }

    adjacent.iter().map(|&i| numbers[i]).sum()
}

pub fn part2(input: &Input) -> u32 {
    let Input { grid, seen, numbers } = input;
    let mut adjacent = FastSet::new();
    let mut result = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point::new(x, y);
            let b = grid[p];

            if b == b'*' {
                for next in DIAGONAL.iter().copied().map(|d| p + d) {
                    if seen[next] != 0 {
                        adjacent.insert(seen[next]);
                    }
                }
            }

            if adjacent.len() == 2 {
                result += adjacent.iter().map(|&i| numbers[i]).product::<u32>();
            }

            adjacent.clear();
        }
    }

    result
}
