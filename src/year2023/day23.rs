use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;
use std::collections::VecDeque;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(grid: &Grid<u8>) -> i32 {
    let start = Point::new(1, 0);
    let end = Point::new(grid.width - 2, grid.height - 1);

    let mut seen = FastSet::new();
    let mut todo = VecDeque::new();
    let mut result = 0;

    seen.insert(start);
    todo.push_back((0, start, seen));

    while let Some((cost, pos, seen)) = todo.pop_front() {
        if pos == end {
            result = result.max(cost);
            continue;
        }
        if !grid.contains(pos) {
            continue;
        }

        let b = grid[pos];

        if b == b'.' || b == b'v' {
            let next = pos + DOWN;
            let mut copy = seen.clone();

            if copy.insert(next) {
                todo.push_back((cost + 1, next, copy));
            }
        }

        if b == b'.' || b == b'^' {
            let next = pos + UP;
            let mut copy = seen.clone();

            if copy.insert(next) {
                todo.push_back((cost + 1, next, copy));
            }
        }

        if b == b'.' || b == b'>' {
            let next = pos + RIGHT;
            let mut copy = seen.clone();

            if copy.insert(next) {
                todo.push_back((cost + 1, next, copy));
            }
        }

        if b == b'.' || b == b'<' {
            let next = pos + LEFT;
            let mut copy = seen.clone();

            if copy.insert(next) {
                todo.push_back((cost + 1, next, copy));
            }
        }
    }

    result
}

pub fn part2(grid: &Grid<u8>) -> i32 {
    let start = Point::new(1, 0);
    let end = Point::new(grid.width - 2, grid.height - 1);

    let mut poi = FastSet::new();
    poi.insert(start);
    poi.insert(end);

    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point::new(x, y);
            if grid[p] != b'#' {
                let mut neighbors = 0;

                for o in ORTHOGONAL {
                    let next = p + o;
                    if grid.contains(next) && grid[next] != b'#' {
                        neighbors += 1;
                    }
                }

                if neighbors > 2 {
                    poi.insert(p);
                }
            }
        }
    }

    let mut edges = FastMap::new();

    for &start in &poi {
        edges.insert(start, bfs(grid, &poi, start));
    }

    let mut result = 0;

    let mut seen = FastSet::new();
    seen.insert(start);

    let mut todo = VecDeque::new();
    todo.push_back((start, seen, 0));

    while let Some((pos, seen, cost)) = todo.pop_front() {
        if pos == end {
            result = result.max(cost);
            continue;
        }

        for &(next, extra) in &edges[&pos] {
            if !seen.contains(&next) {
                let mut copy = seen.clone();
                copy.insert(next);

                todo.push_back((next, copy, cost + extra));
            }
        }
    }

    result
}

fn bfs(grid: &Grid<u8>, poi: &FastSet<Point>, start: Point) -> Vec<(Point, i32)> {
    let mut todo = VecDeque::new();
    let mut seen = FastSet::new();
    let mut result = Vec::new();

    todo.push_back((start, 0));
    seen.insert(start);

    while let Some((pos, cost)) = todo.pop_front() {
        if pos != start && poi.contains(&pos) {
            result.push((pos, cost));
            continue;
        }

        for o in ORTHOGONAL {
            let next = pos + o;

            if grid.contains(next) && grid[next] != b'#' && seen.insert(next) {
                todo.push_back((next, cost + 1));
            }
        }
    }

    result
}
