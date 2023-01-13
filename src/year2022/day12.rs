use crate::util::point::*;
use std::collections::{HashMap, HashSet, VecDeque};

type Grid = HashMap<Point, char>;
type Input = (Grid, Point);

pub fn parse(input: &str) -> Input {
    let mut map: Grid = HashMap::new();
    let mut start: Option<Point> = None;

    for (y, line) in input.lines().enumerate() {
        for (x, value) in line.chars().enumerate() {
            let key = Point { x: x as i32, y: y as i32 };
            map.insert(key, value);
            if value == 'E' {
                start = Some(key);
            }
        }
    }

    (map, start.unwrap())
}

pub fn part1(input: &Input) -> u32 {
    bfs(input, 'S').unwrap()
}

pub fn part2(input: &Input) -> u32 {
    bfs(input, 'a').unwrap()
}

fn bfs(input: &Input, end: char) -> Option<u32> {
    let (grid, start) = input;
    let mut todo = VecDeque::from([(*start, 0)]);
    let mut visited = HashSet::from([*start]);

    while let Some((point, cost)) = todo.pop_front() {
        if grid.get(&point) == Some(&end) {
            return Some(cost);
        }
        for next in ORTHOGONAL.iter().map(|&x| x + point) {
            if grid.contains_key(&next)
                && !visited.contains(&next)
                && height(grid, &point) - height(grid, &next) <= 1
            {
                todo.push_back((next, cost + 1));
                visited.insert(next);
            }
        }
    }

    None
}

fn height(grid: &Grid, point: &Point) -> i32 {
    match grid.get(point).unwrap() {
        'S' => 'a' as i32,
        'E' => 'z' as i32,
        c => *c as i32,
    }
}
