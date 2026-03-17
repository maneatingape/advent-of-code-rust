//! # Hex Ed
//!
//! Hex grid parsing and navigation uses
//! [Axial Coordinates](https://www.redblobgames.com/grids/hexagons/#coordinates-cube)
//! exactly as described in the excellent [Red Blob Games](https://www.redblobgames.com/) blog.
//!
//! As mentioned in the blog, the Manhattan distance to the center has the formula
//! `(q.abs() + r.abs() + s.abs()) / 2`.
type Input = (i32, i32);

pub fn parse(input: &str) -> Input {
    let mut iter = input.bytes();
    let mut q: i32 = 0;
    let mut r: i32 = 0;
    let mut part_one = 0;
    let mut part_two = 0;

    while let Some(first) = iter.next() {
        let second = iter.next().unwrap_or(0);
        let (dq, dr) = match (first, second) {
            (b'n', b'e') => (1, -1),
            (b'n', b'w') => (-1, 0),
            (b'n', _) => (0, -1),
            (b's', b'e') => (1, 0),
            (b's', b'w') => (-1, 1),
            (b's', _) => (0, 1),
            _ => unreachable!(),
        };

        // Update axial coordinates.
        q += dq;
        r += dr;

        // Two-letter directions (ne, nw, se, sw) need the trailing delimiter consumed.
        if second == b'e' || second == b'w' {
            iter.next();
        }

        // Manhattan distance to the center.
        // q + r + s = 0, so we can always determine s given the other two.
        part_one = (q.abs() + r.abs() + (q + r).abs()) / 2;
        // Keep track of furthest distance.
        part_two = part_two.max(part_one);
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}
