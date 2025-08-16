//! # The N-Body Problem
//!
//! There are two insights needed to solve part two:
//!
//! * Each axis is independent
//! * Each axis is periodic somewhat like
//!   [simple harmonic motion](https://en.wikipedia.org/wiki/Simple_harmonic_motion).
//!   The velocity returns to zero twice per period.
//!
//! First find the period of each axis, then the answer is the
//! [least common multiple](https://en.wikipedia.org/wiki/Least_common_multiple) of all three
//! combined.
//!
//! The [`signum`] function comes in handy when updating the velocity.
//!
//! [`signum`]: i32::signum
use crate::util::math::*;
use crate::util::parse::*;
use std::array::from_fn;

type Axis = [i32; 8];
type Input = [Axis; 3];

/// Group each axis together
pub fn parse(input: &str) -> Input {
    let n: Vec<_> = input.iter_signed().collect();
    [
        [n[0], n[3], n[6], n[9], 0, 0, 0, 0],
        [n[1], n[4], n[7], n[10], 0, 0, 0, 0],
        [n[2], n[5], n[8], n[11], 0, 0, 0, 0],
    ]
}

pub fn part1(input: &Input) -> i32 {
    let [mut x, mut y, mut z] = *input;

    for _ in 0..1000 {
        x = step(x);
        y = step(y);
        z = step(z);
    }

    let [p0, p1, p2, p3, v0, v1, v2, v3] = from_fn(|i| x[i].abs() + y[i].abs() + z[i].abs());
    p0 * v0 + p1 * v1 + p2 * v2 + p3 * v3
}

pub fn part2(input: &Input) -> usize {
    let [mut x, mut y, mut z] = *input;
    let [mut a, mut b, mut c] = [0, 0, 0];
    let mut count = 0;

    while a * b * c == 0 {
        count += 1;

        if a == 0 {
            x = step(x);
            if stopped(x) {
                a = count;
            }
        }

        if b == 0 {
            y = step(y);
            if stopped(y) {
                b = count;
            }
        }

        if c == 0 {
            z = step(z);
            if stopped(z) {
                c = count;
            }
        }
    }

    // a, b and c are the half period, so multiply by 2 to get final result.
    2 * a.lcm(b.lcm(c))
}

fn step(axis: Axis) -> Axis {
    // "p" is position and "v" velocity
    let [p0, p1, p2, p3, v0, v1, v2, v3] = axis;

    let a = (p1 - p0).signum();
    let b = (p2 - p0).signum();
    let c = (p3 - p0).signum();
    let d = (p2 - p1).signum();
    let e = (p3 - p1).signum();
    let f = (p3 - p2).signum();

    let n0 = v0 + a + b + c;
    let n1 = v1 - a + d + e;
    let n2 = v2 - b - d + f;
    let n3 = v3 - c - e - f;

    [p0 + n0, p1 + n1, p2 + n2, p3 + n3, n0, n1, n2, n3]
}

fn stopped(axis: Axis) -> bool {
    axis[4] == 0 && axis[5] == 0 && axis[6] == 0 && axis[7] == 0
}
