use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

const CENTER: Point = Point::new(65, 65);
const CORNERS: [Point; 4] =
    [Point::new(0, 0), Point::new(130, 0), Point::new(0, 130), Point::new(130, 130)];

type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);

    let (even_inner, even_outer, odd_inner, odd_outer) = bfs(&grid, &[CENTER], 130);
    let part_one = even_inner;
    let even_full = even_inner + even_outer;
    let odd_full = odd_inner + odd_outer;
    let remove_corners = odd_outer;

    let (even_inner, ..) = bfs(&grid, &CORNERS, 64);
    let add_corners = even_inner;

    let n = 202300;
    let first = n * n * even_full;
    let second = (n + 1) * (n + 1) * odd_full;
    let third = n * add_corners;
    let fourth = (n + 1) * remove_corners;
    let part_two = first + second + third - fourth;

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}

fn bfs(grid: &Grid<u8>, starts: &[Point], limit: u32) -> (u64, u64, u64, u64) {
    let mut grid = grid.clone();
    let mut todo = VecDeque::new();

    let mut even_inner = 0;
    let mut even_outer = 0;
    let mut odd_inner = 0;
    let mut odd_outer = 0;

    for &start in starts {
        grid[start] = b'#';
        todo.push_back((start, 0));
    }

    while let Some((position, cost)) = todo.pop_front() {
        if cost % 2 == 1 {
            if position.manhattan(CENTER) <= 65 {
                odd_inner += 1;
            } else {
                odd_outer += 1;
            }
        } else if cost <= 64 {
            even_inner += 1;
        } else {
            even_outer += 1;
        }

        if cost < limit {
            for next in ORTHOGONAL.map(|o| position + o) {
                if grid.contains(next) && grid[next] != b'#' {
                    grid[next] = b'#';
                    todo.push_back((next, cost + 1));
                }
            }
        }
    }

    (even_inner, even_outer, odd_inner, odd_outer)
}
