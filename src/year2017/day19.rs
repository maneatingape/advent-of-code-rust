//! # A Series of Tubes
//!
//! Uses the utility [`Grid`] to parse the input, then [`Point`] to follow the path.
//!
//! [`Grid`]: crate::util::grid
//! [`Point`]: crate::util::point
use crate::util::grid::*;
use crate::util::point::*;

type Input = (String, u32);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);

    let mut position = grid.find(b'|').unwrap();
    let mut direction = DOWN;

    let mut part_one = String::new();
    let mut part_two = 0;

    loop {
        let next = grid[position];

        match next {
            b'+' => {
                let left = direction.counter_clockwise();
                let right = direction.clockwise();
                direction = if grid[position + right] == b' ' { left } else { right };
            }
            b' ' => break,
            _ if next.is_ascii_alphabetic() => part_one.push(next as char),
            _ => (),
        }

        position += direction;
        part_two += 1;
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> &str {
    &input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
