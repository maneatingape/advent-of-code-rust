use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

type Input = (Grid, Point);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);
    let start = grid.find(b'E');
    (grid, start.unwrap())
}

pub fn part1(input: &Input) -> u32 {
    bfs(input, b'S')
}

pub fn part2(input: &Input) -> u32 {
    bfs(input, b'a')
}

fn bfs(input: &Input, end: u8) -> u32 {
    let (grid, start) = input;
    let mut todo = VecDeque::from([(*start, 0)]);
    let mut visited = grid.empty_copy();

    while let Some((point, cost)) = todo.pop_front() {
        if grid.get(point) == end {
            return cost;
        }
        for next in ORTHOGONAL.iter().map(|&x| x + point) {
            if grid.contains(next)
                && visited.get(next) == 0
                && height(grid, point) - height(grid, next) <= 1
            {
                todo.push_back((next, cost + 1));
                visited.set(next, 1);
            }
        }
    }

    unreachable!()
}

fn height(grid: &Grid, point: Point) -> i32 {
    match grid.get(point) {
        b'S' => 'a' as i32,
        b'E' => 'z' as i32,
        b => b as i32,
    }
}
