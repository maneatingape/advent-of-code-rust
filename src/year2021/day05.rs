use crate::util::collection::*;
use crate::util::parse::*;

type Input = [u32; 4];

pub fn parse(input: &str) -> Vec<Input> {
    input.iter_unsigned().chunked::<4>().collect()
}

pub fn part1(input: &[Input]) -> usize {
    let orthogonal: Vec<_> = input
        .iter()
        .copied()
        .filter(|[x1, y1, x2, y2]| x1 == x2 || y1 == y2)
        .collect();
    vents(&orthogonal)
}

pub fn part2(input: &[Input]) -> usize {
    vents(input)
}

fn vents(input: &[Input]) -> usize {
    let mut grid = [0u8; 1_000_000];

    for &[x1, y1, x2, y2] in input {
        let (x1, y1, x2, y2) = (x1 as i32, y1 as i32, x2 as i32, y2 as i32);
        let count = (y2 - y1).abs().max((x2 - x1).abs());
        let delta = (y2 - y1).signum() * 1000 + (x2 - x1).signum();
        let mut index = y1 * 1000 + x1;

        for _ in 0..=count {
            grid[index as usize] += 1;
            index += delta;
        }
    }

    grid.iter().filter(|&&n| n > 1).count()
}
