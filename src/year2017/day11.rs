//! # Hex Ed
//!
//! Hex grid parsing and navigation uses
//! [Axial Coordinates](https://www.redblobgames.com/grids/hexagons/#coordinates-cube)
//! exactly as described in the excellent [Red Blob Games](https://www.redblobgames.com/) blog.
//!
//! As mentioned in the blog, the Manhattan distance to the center has the formula
//! `(q.abs() + r.abs() + s.abs()) / 2`
type Input = (i32, i32);

pub fn parse(input: &str) -> Input {
    let mut iter = input.bytes();
    let mut q: i32 = 0;
    let mut r: i32 = 0;
    let mut part_one = 0;
    let mut part_two = 0;

    while let Some(first) = iter.next() {
        if first == b'n' {
            match iter.next().unwrap_or(0) {
                b'e' => {
                    iter.next(); // Consume trailing delimeter.
                    q += 1;
                    r -= 1;
                }
                b'w' => {
                    iter.next(); // Consume trailing delimeter.
                    q -= 1;
                }
                _ => r -= 1,
            }
        } else {
            match iter.next().unwrap_or(0) {
                b'e' => {
                    iter.next(); // Consume trailing delimeter.
                    q += 1;
                }
                b'w' => {
                    iter.next(); // Consume trailing delimeter.
                    q -= 1;
                    r += 1;
                }
                _ => r += 1,
            }
        }

        // q + r + s = 0, so we can always determine s given the other two.
        let s = q + r;
        // Manhattan distance to the center.
        part_one = (q.abs() + r.abs() + s.abs()) / 2;
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
