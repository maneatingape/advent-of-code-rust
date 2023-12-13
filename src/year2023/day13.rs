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

    for i in 1..size {
        let mut smudges = 0;

        for j in 0..i.min(size - i) {
            smudges += (axis[i - j - 1] ^ axis[i + j]).count_ones();
        }

        if smudges == target {
            return Some(i);
        }
    }

    None
}
