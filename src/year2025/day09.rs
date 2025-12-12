use crate::util::iter::*;
use crate::util::parse::*;

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

fn box_area(p1: &Tile, p2: &Tile) -> u64 {
    let dx = p1[0].abs_diff(p2[0]) + 1;
    let dy = p1[1].abs_diff(p2[1]) + 1;
    dx * dy
}

pub fn part2(tiles: &[Tile]) -> u64 {
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
