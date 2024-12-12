//! # Garden Groups
use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;
use std::collections::VecDeque;

const CLOCKWISE: [Point; 5] = [UP, RIGHT, DOWN, LEFT, UP];

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(grid: &Grid<u8>) -> i32 {
    let mut todo = VecDeque::new();
    let mut seen = grid.same_size_with(false);
    let mut added = grid.same_size_with(false);
    let mut result = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if seen[point] {
                continue;
            }

            let kind = grid[point];
            let mut area = 0;
            let mut perm = 0;

            todo.push_back(point);
            seen[point] = true;

            while let Some(point) = todo.pop_front() {
                area += 1;
                perm += 4;
                added[point] = true;

                for next in ORTHOGONAL.map(|o| point + o) {
                    if grid.contains(next) && grid[next] == kind {
                        if !seen[next] {
                            seen[next] = true;
                            todo.push_back(next);
                        }
                        if added[next] {
                            perm -= 2;
                        }
                    }
                }
            }

            result += area * perm;
        }
    }

    result
}

pub fn part2(grid: &Grid<u8>) -> u32 {
    let mut seen = grid.same_size_with(false);
    let mut todo = VecDeque::new();
    let mut corner = FastMap::new();
    let mut middle = FastMap::new();
    let mut result = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if seen[point] {
                continue;
            }

            let kind = grid[point];
            let mut size = 0;
            let mut sides = 0;

            todo.push_back(point);
            seen[point] = true;

            while let Some(point) = todo.pop_front() {
                size += 1;
                let x = 2 * point.x;
                let y = 2 * point.y;

                *corner.entry(Point::new(x, y)).or_insert(0) += 1;
                *corner.entry(Point::new(x + 2, y)).or_insert(0) += 1;
                *corner.entry(Point::new(x, y + 2)).or_insert(0) += 1;
                *corner.entry(Point::new(x + 2, y + 2)).or_insert(0) += 1;

                *middle.entry(Point::new(x + 1, y)).or_insert(0) += 1;
                *middle.entry(Point::new(x, y + 1)).or_insert(0) += 1;
                *middle.entry(Point::new(x + 2, y + 1)).or_insert(0) += 1;
                *middle.entry(Point::new(x + 1, y + 2)).or_insert(0) += 1;

                for next in ORTHOGONAL.map(|o| point + o) {
                    if grid.contains(next) && grid[next] == kind && !seen[next] {
                        seen[next] = true;
                        todo.push_back(next);
                    }
                }
            }

            for (&point, _) in corner.iter().filter(|(_, &v)| v < 4) {
                let freq = CLOCKWISE.map(|c| *middle.get(&(point + c)).unwrap_or(&2));
                let count = freq.windows(2).filter(|w| w[0] < 2 && w[1] < 2).count();

                if count == 1 {
                    sides += 1;
                } else if count == 4 {
                    sides += 2;
                }
            }

            corner.clear();
            middle.clear();
            result += size * sides;
        }
    }

    result
}
