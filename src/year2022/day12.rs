use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

type Input = (Grid<u8>, Point);

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
    let mut visited = grid.default_copy::<bool>();

    while let Some((point, cost)) = todo.pop_front() {
        if grid[point] == end {
            return cost;
        }
        for next in ORTHOGONAL.iter().map(|&x| x + point) {
            if grid.contains(next)
                && !visited[next]
                && height(grid, point) - height(grid, next) <= 1
            {
                todo.push_back((next, cost + 1));
                visited[next] = true;
            }
        }
    }

    unreachable!()
}

fn height(grid: &Grid<u8>, point: Point) -> i32 {
    match grid[point] {
        b'S' => 'a' as i32,
        b'E' => 'z' as i32,
        b => b as i32,
    }
}
