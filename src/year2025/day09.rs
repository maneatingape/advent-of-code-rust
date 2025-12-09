use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;
use std::collections::VecDeque;

const OUTSIDE: i64 = 0;
const INSIDE: i64 = 1;
const UNKNOWN: i64 = 2;

type Tile = [u64; 2];

pub fn parse(input: &str) -> Vec<Tile> {
    input.iter_unsigned::<u64>().chunk::<2>().collect()
}

pub fn part1(tiles: &[Tile]) -> u64 {
    let mut area = 0;

    for (i, &[x1, y1]) in tiles.iter().enumerate() {
        for &[x2, y2] in tiles.iter().skip(i + 1) {
            let dx = x1.abs_diff(x2) + 1;
            let dy = y1.abs_diff(y2) + 1;
            area = area.max(dx * dy);
        }
    }

    area
}

pub fn part2(tiles: &[Tile]) -> u64 {
    let size = tiles.len();
    let shrink_x = shrink(tiles, 0);
    let shrink_y = shrink(tiles, 1);
    let shrunk: Vec<_> = tiles.iter().map(|&[x, y]| (shrink_x[&x], shrink_y[&y])).collect();

    let mut area = 0;
    let mut todo = VecDeque::from([ORIGIN]);
    let mut grid = Grid::new(shrink_x.len() as i32, shrink_y.len() as i32, UNKNOWN);

    for i in 0..size {
        let (x1, y1, x2, y2) = minmax(shrunk[i], shrunk[(i + 1) % size]);

        for x in x1..x2 + 1 {
            for y in y1..y2 + 1 {
                grid[Point::new(x, y)] = INSIDE;
            }
        }
    }

    while let Some(point) = todo.pop_front() {
        for next in ORTHOGONAL.map(|o| point + o) {
            if grid.contains(next) && grid[next] == UNKNOWN {
                grid[next] = OUTSIDE;
                todo.push_back(next);
            }
        }
    }

    for y in 1..grid.height {
        for x in 1..grid.width {
            let point = Point::new(x, y);
            let value = i64::from(grid[point] != OUTSIDE);
            grid[point] = value + grid[point + UP] + grid[point + LEFT] - grid[point + UP + LEFT];
        }
    }

    for i in 0..size {
        for j in i + 1..size {
            let (x1, y1, x2, y2) = minmax(shrunk[i], shrunk[j]);

            let expected = (x2 - x1 + 1) as i64 * (y2 - y1 + 1) as i64;
            let actual = grid[Point::new(x2, y2)]
                - grid[Point::new(x1 - 1, y2)]
                - grid[Point::new(x2, y1 - 1)]
                + grid[Point::new(x1 - 1, y1 - 1)];

            if expected == actual {
                let [x1, y1] = tiles[i];
                let [x2, y2] = tiles[j];
                let dx = x1.abs_diff(x2) + 1;
                let dy = y1.abs_diff(y2) + 1;
                area = area.max(dx * dy);
            }
        }
    }

    area
}

fn shrink(tiles: &[Tile], index: usize) -> FastMap<u64, i32> {
    let mut axis: Vec<_> = tiles.iter().map(|tile| tile[index]).collect();
    axis.push(u64::MIN);
    axis.push(u64::MAX);
    axis.sort_unstable();
    axis.dedup();
    axis.iter().enumerate().map(|(i, &n)| (n, i as i32)).collect()
}

#[inline]
fn minmax((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32, i32, i32) {
    (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2))
}
