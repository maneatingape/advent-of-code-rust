//! # Trick Shot
//!
//! Although this problem is easy to brute force, we can apply some reasoning and simplify both parts.
//!
//! ## Part One
//! Part one can be solved analytically. Movement upwards in the positive y direction is symmetrical.
//! For example launching a probe at a y-velocity of 5 initially,
//! would result in a  speed and y-position:
//!
//! ```text
//!     Time:       0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12
//!     Speed:      5,  4,  3,  2,  1,  0, -1, -2, -3, -4, -5, -6, -7
//!     Y-Position: 0,  5,  9, 12, 14, 15, 15, 14, 12,  9,  5,  0, -6
//! ```
//!
//! The maximum y velocity is reached when we *just* touch the target area on the way down at the
//! bottom y-coordinate. For the example above, if the bottom y coordinate was -6 then the maximum
//! initial upwards velocity is one less, our starting velocity of 5.
//!
//! The maximum height is `5 + 4 + 3 + 2 + 1`, which is the sum from 1 to n given by the formula for
//! triangular numbers [`(n * (n + 1) / 2`](https://en.wikipedia.org/wiki/Triangular_number#Formula).
//!
//! ## Part Two
//! A brute force solution would check every possible combination of `x` and `y` for a total
//! complexity of `O(xy)`. By thinking in terms of time `t` instead and applying a dynamic
//! programming solution we can instead solve in a complexity of `O(x + y)`by treating `x` and `y`
//! independently.
//!
//! We create 2 `vecs`. The first `new` counts how many x-velocity values enter the target area at
//! time `t` for the first time, only considering horizontal movement. The second `continuing`
//! counts how many are still in the target area at time `t`.
//!
//! For example using the sample `target area: x=20..30, y=-10..-5` gives a progression:
//!
//! ```text
//!     X-Velocity : 6
//!     Time:        0,  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
//!     New:         0,  0, 0, 0, 0, 1, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
//!     Continuing:  0,  0, 0, 0, 0, 0, 1, 1, 1, 1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1,  1
//!
//!     X-Velocity : 7
//!     Time:        0,  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
//!     New:         0,  0, 0, 0, 1, 1, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
//!     Continuing:  0,  0, 0, 0, 0, 1, 2, 2, 2, 2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2
//!
//!     X-Velocity : 8
//!     Time:        0,  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
//!     New:         0,  0, 0, 1, 1, 1, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
//!     Continuing:  0,  0, 0, 0, 1, 2, 2, 2, 2, 2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2
//!
//!     ...
//!
//!     X-Velocity : 30
//!     Time:        0,  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
//!     New:         0, 11, 5, 3, 1, 1, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
//!     Continuing:  0,  0, 0, 1, 2, 2, 2, 2, 2, 2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2,  2
//! ```
//!
//! Then for each y velocity value we find the time when it enters the target area. The first time
//! this happens we add *both* `new` and `continuing` to the total. For subsequent times while we're
//! still in the target area we add only the `new` values, as the `continuing` are trajectories
//! that we've already considered. For example for an initial y-velocity of 0:
//!
//! ```text
//!     Time:       0,   1,   2,     3,   4
//!     Speed:      0,  -1,  -2,    -3,  -4
//!     Y-Position: 0,  -1,  -3,    -6, -10
//!     Total:      0,   0,   0, 3 + 1,   5
//! ```
//!
//! Summing this for all y-velocity values gives the desired result.
use crate::util::iter::*;
use crate::util::parse::*;

type Input = [i32; 4];

pub fn parse(input: &str) -> Input {
    input.iter_signed().chunk::<4>().next().unwrap()
}

pub fn part1(input: &Input) -> i32 {
    let &[_, _, bottom, _] = input;
    let n = -(bottom + 1);
    n * (n + 1) / 2
}

pub fn part2(input: &Input) -> usize {
    let &[left, right, bottom, top] = input;

    let mut n = 1;
    while n * (n + 1) / 2 < left {
        n += 1;
    }

    let min_dx = n;
    let max_dx = right + 1;
    let min_dy = bottom;
    let max_dy = -bottom;

    let max_t = (1 - 2 * bottom) as usize;
    let mut new = vec![0; max_t];
    let mut continuing = vec![0; max_t];
    let mut total = 0;

    for dx in min_dx..max_dx {
        let mut x = 0;
        let mut dx = dx;
        let mut first = true;

        for t in 0..max_t {
            if x > right {
                break;
            }
            if x >= left {
                if first {
                    first = false;
                    new[t] += 1;
                } else {
                    continuing[t] += 1;
                }
            }
            x += dx;
            dx = (dx - 1).max(0);
        }
    }

    for dy in min_dy..max_dy {
        let mut y = 0;
        let mut dy = dy;
        let mut t = 0;
        let mut first = true;

        while y >= bottom {
            if y <= top {
                if first {
                    first = false;
                    total += continuing[t];
                }
                total += new[t];
            }
            y += dy;
            dy -= 1;
            t += 1;
        }
    }

    total
}
