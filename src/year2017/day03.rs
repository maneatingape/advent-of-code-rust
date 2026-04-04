//! # Spiral Memory
//!
//! ## Part One
//!
//! Consider the layout as a sequence of hollow donuts. We find the donut that contains the value
//! which gives one component of the Manhattan value. The second component is the distance from
//! the center of each edge.
//!
//! For example, say the target value is 20. We find the donut then subtract the inner donuts
//! to make the values relative then calculate the values modulo the edge size.
//!
//! ```none
//!                                                      <------------
//!     17  16  15  14  13      7   6   5   4   3        3   2  [1]  0   3  ^
//!     18   5   4   3  12      8               2     |  0               2  |
//!     19   6   1   2  11 =>   9               1 =>  | [1]             [1] |
//!     20   7   8   9  10     10               0     |  2               0  |
//!     21  22  23  24  25     11  12  13  14  15     v  3   0  [1]  2   3
//!                                                          ------------>
//! ```
//!
//! The first component is the horizontal or vertical distance from the center to the ring,
//! in this case 2 steps. The second component is the distance from the target number to the
//! center of each edge, in this case 2 - 1 = 1.
//!
//! ## Part Two
//!
//! We use the [`Point`] utility to move in the spiral direction. Values are stored in a hashmap
//! defaulting to zero if the value doesn't exist yet.
//!
//! Note that [OEIS A141481](https://oeis.org/A141481/b141481.txt) shows the sequence well
//! past the limit requested by the puzzle. However, this is fast enough to solve to not
//! be worth turning this into a lookup table.
use crate::util::hash::*;
use crate::util::parse::*;
use crate::util::point::*;

pub fn parse(input: &str) -> u32 {
    input.unsigned()
}

pub fn part1(input: &u32) -> u32 {
    let target = *input;
    let mut a = 3;

    // Find the donut that contains the value.
    while a * a < target {
        a += 2;
    }
    let b = a - 1;
    let c = a - 2;

    // Distance to donut plus distance to center of edge.
    (b / 2) + (c / 2).abs_diff((target - c * c - 1) % b)
}

pub fn part2(input: &u32) -> u32 {
    let target = *input;
    let mut values = FastMap::build([(ORIGIN, 1)]);
    let mut position = ORIGIN;
    let mut direction = RIGHT;
    let mut steps = 1;

    loop {
        for _ in 0..2 {
            for _ in 0..steps {
                position += direction;

                let next: u32 =
                    DIAGONAL.iter().map(|&d| values.get(&(position + d)).unwrap_or(&0)).sum();

                if next > target {
                    return next;
                }
                values.insert(position, next);
            }
            direction = direction.counter_clockwise();
        }
        steps += 1;
    }
}
