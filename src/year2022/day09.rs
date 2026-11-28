//! # Rope Bridge
//!
//! This solution relies on the [`Point`] utility module. Two dimensional problems are common in
//! Advent of Code, so having a decent `Point` (or `Coord` or `Pos`) implementation is convenient.
use crate::util::grid::*;
use crate::util::parse::*;
use crate::util::point::*;

type Input = (u32, u32);

/// Converts input lines into a pair of [`Point`] and integer amount, to indicate direction and
/// magnitude respectively. Then determines the maximum extent of the head so that we can allocate
/// a two-dimensional grid.
pub fn parse(input: &str) -> Input {
    let steps = input.bytes().filter(u8::is_ascii_alphabetic).map(Point::from);
    let amounts = input.iter_signed();
    let pairs: Vec<_> = steps.zip(amounts).collect();

    // Determine maximum extents.
    let (x1, y1, x2, y2, _) =
        pairs.iter().fold((0, 0, 0, 0, ORIGIN), |(x1, y1, x2, y2, point), &(step, amount)| {
            let next = point + step * amount;
            (x1.min(next.x), y1.min(next.y), x2.max(next.x), y2.max(next.y), next)
        });

    let mut part_one = 1;
    let mut part_two = 2;
    let start = Point::new(-x1, -y1);
    let mut rope = [start; 10];
    let mut grid = Grid::new(x2 - x1 + 1, y2 - y1 + 1, 0_u8);

    // Knots only record their position after the second knot moves, so mark the starting square
    // upfront for both parts.
    grid[start] = 3;

    for (step, amount) in pairs {
        for _ in 0..amount {
            let head = rope[0];
            rope[0] += step;

            // The second knot only moves when the head pulls apart from it. As the head only moves
            // horizontally or vertically, the second knot always lands on the head's old position.
            let delta = rope[0] - rope[1];
            if delta.x.abs() <= 1 && delta.y.abs() <= 1 {
                continue;
            }

            rope[1] = head;
            part_one += u32::from(!grid[head] & 1);
            grid[head] |= 1;

            // Two knots are considered "apart" if they are not diagonally adjacent, that is
            // the absolute distance in either x or y axes is greater than 1.
            for i in 2..10 {
                let delta = rope[i - 1] - rope[i];
                if delta.x.abs() <= 1 && delta.y.abs() <= 1 {
                    break;
                }
                rope[i] += Point::new(delta.x.signum(), delta.y.signum());
            }

            part_two += u32::from(!grid[rope[9]] & 2);
            grid[rope[9]] |= 2;
        }
    }

    (part_one, part_two >> 1)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
