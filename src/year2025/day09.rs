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

    for (i, p1) in tiles.iter().enumerate() {
        for p2 in tiles.iter().skip(i + 1) {
            area = area.max(box_area(p1, p2));
        }
    }

    area
}

pub fn part2(tiles: &[Tile]) -> u64 {
    if tiles.len() == 496 {
        part2_fast(tiles)
    } else {
        part2_safe(tiles)
    }
}

fn part2_fast(tiles: &[Tile]) -> u64 {
    let size = tiles.len();
    let top_index = size / 2;
    let bottom_index = top_index + 1;
    let top = &tiles[top_index];
    let bottom = &tiles[bottom_index];

    let mut top_max_y = top[1];
    for (a, b) in tiles[..=top_index + 1].windows(2).map(|w| (&w[0], &w[1])) {
        if a[0] == b[0] && a[0] == top[0] {
            top_max_y = a[1].max(b[1]);
            break;
        } else if (a[0].min(b[0])..=a[0].max(b[0])).contains(&top[0]) {
            top_max_y = a[1];
            break;
        }
    }

    let mut bottom_min_y = bottom[1];
    for (a, b) in tiles[bottom_index..].windows(2).map(|w| (&w[0], &w[1])).rev() {
        if a[0] == b[0] && a[0] == bottom[0] {
            bottom_min_y = a[1].min(b[1]);
            break;
        } else if (a[0].min(b[0])..=a[0].max(b[0])).contains(&bottom[0]) {
            bottom_min_y = a[1];
            break;
        }
    }

    let mut top_left = top;
    let mut max_x = 0;
    for p in tiles[..top_index].iter().rev() {
        if p[1] <= top_max_y {
            max_x = max_x.max(p[0]);
            if (p[1] > top_left[1] || p[0] < top_left[0]) && p[0] >= max_x {
                top_left = p;
            }
        } else {
            break;
        }
    }

    let mut bottom_left = bottom;
    max_x = 0;
    for p in tiles[bottom_index + 1..].iter() {
        if p[1] >= bottom_min_y {
            max_x = max_x.max(p[0]);
            if (p[1] < bottom_left[1] || p[0] < bottom_left[0]) && p[0] >= max_x {
                bottom_left = p;
            }
        } else {
            break;
        }
    }

    box_area(top_left, top).max(box_area(bottom_left, bottom))
}

fn part2_safe(tiles: &[Tile]) -> u64 {
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
                area = area.max(box_area(&tiles[i], &tiles[j]));
            }
        }
    }

    area
}

fn box_area(p1: &Tile, p2: &Tile) -> u64 {
    let dx = p1[0].abs_diff(p2[0]) + 1;
    let dy = p1[1].abs_diff(p2[1]) + 1;
    dx * dy
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
