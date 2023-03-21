//! # Dumbo Octopus
//!
//! This puzzle resembles the [`Day 9`] flood fill a little. Since there are only 100 octopuses
//! a fixed size array is used both to track current energy levels and a second array to track
//! if an octopus has flashed this turn. Each time an octopus flashes it bumps its neighbors
//! energy levels, which can propagate recursively through the entire grid.
//!
//! [`Day 9`]: crate::year2021::day09
type Input = [u8; 100];

pub fn parse(input: &str) -> Input {
    let mut grid = [0; 100];
    input
        .as_bytes()
        .iter()
        .filter(|b| b.is_ascii_digit())
        .enumerate()
        .for_each(|(i, b)| grid[i] = b - 48);
    grid
}

pub fn part1(input: &Input) -> u32 {
    let mut grid = *input;
    let mut total = 0;

    for _ in 0..100 {
        total += step(&mut grid);
    }

    total
}

pub fn part2(input: &Input) -> u32 {
    let mut grid = *input;
    let mut total = 0;
    let mut steps = 0;

    while total < 100 {
        total = step(&mut grid);
        steps += 1
    }

    steps
}

fn step(grid: &mut [u8; 100]) -> u32 {
    let mut total = 0;
    let mut flashed = [false; 100];
    for x in 0..10 {
        for y in 0..10 {
            total += bump(grid, &mut flashed, x, y);
        }
    }
    total
}

fn bump(grid: &mut [u8; 100], flashed: &mut [bool; 100], x: i32, y: i32) -> u32 {
    if !(0..10).contains(&x) || !(0..10).contains(&y) {
        return 0;
    }

    let index = (y * 10 + x) as usize;
    if flashed[index] {
        return 0;
    }

    grid[index] += 1;
    if grid[index] <= 9 {
        return 0;
    }

    grid[index] = 0;
    flashed[index] = true;

    let mut count = 1;
    count += bump(grid, flashed, x - 1, y - 1);
    count += bump(grid, flashed, x, y - 1);
    count += bump(grid, flashed, x + 1, y - 1);
    count += bump(grid, flashed, x - 1, y);
    count += bump(grid, flashed, x + 1, y);
    count += bump(grid, flashed, x - 1, y + 1);
    count += bump(grid, flashed, x, y + 1);
    count += bump(grid, flashed, x + 1, y + 1);
    count
}
