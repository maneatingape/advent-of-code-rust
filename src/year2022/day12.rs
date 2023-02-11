use crate::util::point::*;
use core::panic;
use std::collections::VecDeque;

const ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];

type Input = (Grid, Point);

pub struct Grid {
    width: i32,
    height: i32,
    bytes: Vec<u8>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let width = (input.chars().position(|c| c.is_ascii_whitespace()).unwrap() + 1) as i32;
        let height = input.lines().count() as i32;
        let bytes = Vec::from(input.as_bytes());
        Grid { width, height, bytes }
    }

    fn contains(&self, point: Point) -> bool {
        (0..self.width - 1).contains(&point.x) && (0..self.height).contains(&point.y)
    }

    fn get(&self, point: Point) -> u8 {
        if self.contains(point) {
            self.bytes[(self.width * point.y + point.x) as usize]
        } else {
            panic!("{:?} is not contained in grid", point)
        }
    }

    fn set(&mut self, point: Point, value: u8) {
        if self.contains(point) {
            self.bytes[(self.width * point.y + point.x) as usize] = value;
        }
    }

    fn find(&self, needle: u8) -> Option<Point> {
        self.bytes
            .iter()
            .position(|&haystack| haystack == needle)
            .map(|index| {
                let x = (index as i32) % self.width;
                let y = (index as i32) / self.width;
                Point { x, y }
            })
    }
}

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
    let mut visited = Grid {
        width: grid.width,
        height: grid.height,
        bytes: vec![0; grid.bytes.len()],
    };

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
