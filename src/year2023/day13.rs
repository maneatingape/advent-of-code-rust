use crate::util::grid::*;
use crate::util::point::*;

type Input = Vec<(Grid<u8>, Vec<Point>)>;

pub fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|block| {
            let grid: Grid<_> = Grid::parse(block);
            let mut points = Vec::with_capacity(1000);

            for y in 0..grid.height {
                for x in 0..grid.width {
                    let point = Point::new(x, y);
                    if grid[point] == b'#' {
                        points.push(point);
                    }
                }
            }

            (grid, points)
        })
        .collect()
}

pub fn part1(input: &Input) -> i32 {
    reflect(input, 0)
}

pub fn part2(input: &Input) -> i32 {
    reflect(input, 1)
}

fn reflect(input: &Input, target: i32) -> i32 {
    input
        .iter()
        .map(|(grid, points)| {
            for x in 1..grid.width {
                let mut smudges = 0;

                for &p in points {
                    let reflected = Point::new(2 * x - p.x - 1, p.y);
                    if grid.contains(reflected) && grid[reflected] == b'.' {
                        smudges += 1;
                        if smudges > target {
                            break;
                        }
                    }
                }

                if smudges == target {
                    return x;
                }
            }

            for y in 1..grid.height {
                let mut smudges = 0;

                for &p in points {
                    let reflected = Point::new(p.x, 2 * y - p.y - 1);
                    if grid.contains(reflected) && grid[reflected] == b'.' {
                        smudges += 1;
                        if smudges > target {
                            break;
                        }
                    }
                }

                if smudges == target {
                    return 100 * y;
                }
            }

            unreachable!()
        })
        .sum()
}
