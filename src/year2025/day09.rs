use crate::util::parse::*;

type Tile = [u64; 2];

pub fn parse(input: &str) -> Vec<Tile> {
    input.iter_unsigned::<u64>().chunk::<2>().collect()
}

pub fn part1(tiles: &[Tile]) -> u64 {
    let mut area = 0;

    for (i, &[x1, y1]) in tiles.iter().enumerate() {
        for &[x2, y2] in tiles.iter().skip(i + 1) {
            area = area.max(rect_area(x1, y1, x2, y2));
        }
    }

    area
}
  #[inline]
  fn rect_area(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 {
      (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
  }

pub fn part2(tiles: &[Tile]) -> u64 {
    let size = tiles.len();

    // Find top K longest edges and collect candidate vertices
    let mut edge_lengths: Vec<(u64, usize)> = (0..size)
        .map(|i| {
            let j = (i + 1) % size;
            let dx = tiles[i][0].abs_diff(tiles[j][0]);
            let dy = tiles[i][1].abs_diff(tiles[j][1]);
            (dx.max(dy), i)
        })
        .collect();
    edge_lengths.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    let mut candidates: Vec<usize> = Vec::with_capacity(8);
    for &(_, i) in edge_lengths.iter().take(4) {
        candidates.push(i);
        candidates.push((i + 1) % size);
    }
    candidates.sort_unstable();
    candidates.dedup();

    // Build edge AABBs for collision detection
    let edges: Vec<_> = (0..size)
        .map(|i| {
            let j = (i + 1) % size;
            let [x1, y1] = tiles[i];
            let [x2, y2] = tiles[j];
            (x1.min(x2), x1.max(x2), y1.min(y2), y1.max(y2))
        })
        .collect();

    // Check candidates paired with all vertices
    let mut area = 0;

    for &c in &candidates {
        let [cx, cy] = tiles[c];

        for (i, &[x, y]) in tiles.iter().enumerate() {
            if i == c {
                continue;
            }

            let (min_x, max_x) = if cx < x { (cx, x) } else { (x, cx) };
            let (min_y, max_y) = if cy < y { (cy, y) } else { (y, cy) };

            let valid = !edges.iter().any(|&(ex1, ex2, ey1, ey2)| {
                min_x < ex2 && max_x > ex1 && min_y < ey2 && max_y > ey1
            });

            if valid {
                area = area.max(rect_area(cx, cy, x, y));
            }
        }
    }

    area
}
