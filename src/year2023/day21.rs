use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

pub struct Input {
    part_one: u64,
    even_full: u64,
    even_corner: u64,
    odd_full: u64,
    odd_corner: u64,
}

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = Point::new(65, 65);

    let mut todo = VecDeque::new();
    todo.push_back((start, 0));

    let mut seen: Grid<bool> = grid.default_copy();
    seen[start] = true;

    let mut part_one = 0;
    let mut even_full = 0;
    let mut even_corner = 0;
    let mut odd_full = 0;
    let mut odd_corner = 0;

    while let Some((position, cost)) = todo.pop_front() {
        if cost % 2 == 0 {
            if cost < 65 {
                part_one += 1;
            }
            even_full += 1;
            if cost > 65 {
                even_corner += 1;
            }
        } else {
            odd_full += 1;
            if cost > 65 {
                odd_corner += 1;
            }
        }

        for o in ORTHOGONAL {
            let next = position + o;

            if grid.contains(next) && grid[next] != b'#' && !seen[next] {
                todo.push_back((next, cost + 1));
                seen[next] = true;
            }
        }
    }

    Input { part_one, even_full, even_corner, odd_full, odd_corner }
}

pub fn part1(input: &Input) -> u64 {
    input.part_one
}

pub fn part2(input: &Input) -> u64 {
    let Input { even_full, even_corner, odd_full, odd_corner, .. } = input;
    let n = 202300;

    let first = (n + 1) * (n + 1) * odd_full;
    let second = n * n * even_full;
    let third = n * even_corner;
    let fourth = (n + 1) * odd_corner;

    first + second + third - fourth - n
}
