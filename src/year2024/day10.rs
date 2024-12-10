//! # Hoof It
//!
//! [Depth first search](https://en.wikipedia.org/wiki/Depth-first_search) for both parts.
//! Part two is simpler than part one as we don't need to keep track of already visited points.
//! Reverse search was slightly faster as my input contained fewer peaks `9` than valleys `0`.
use crate::util::grid::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(grid: &Grid<u8>) -> u32 {
    solve(grid, false)
}

pub fn part2(grid: &Grid<u8>) -> u32 {
    solve(grid, true)
}

fn solve(grid: &Grid<u8>, distinct: bool) -> u32 {
    let mut result = 0;
    let mut seen = grid.same_size_with(-1);

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if grid[point] == b'9' {
                let id = y * grid.width + x;
                result += dfs(grid, distinct, &mut seen, id, point);
            }
        }
    }

    result
}

fn dfs(grid: &Grid<u8>, distinct: bool, seen: &mut Grid<i32>, id: i32, point: Point) -> u32 {
    let mut result = 0;

    for next in ORTHOGONAL.map(|o| point + o) {
        if grid.contains(next) && grid[next] + 1 == grid[point] && (distinct || seen[next] != id) {
            seen[next] = id;

            if grid[next] == b'0' {
                result += 1;
            } else {
                result += dfs(grid, distinct, seen, id, next);
            }
        }
    }

    result
}
