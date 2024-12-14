//! # Restroom Redoubt
//!
//! For part one we jump straight to the final position by multiplying the velocity by 100.
//! The image appears in part two when the positions of all robots are unique.
//!
//! The x coordinates repeat every 101 seconds and the y coordinates repeat every 103 seconds.
//! Calculating each axis independently then looking it up is twice as fast
//! as calculating as needed.
use crate::util::iter::*;
use crate::util::math::*;
use crate::util::parse::*;

type Robot = [i32; 4];

pub fn parse(input: &str) -> Vec<Robot> {
    input.iter_signed().chunk::<4>().collect()
}

pub fn part1(input: &[Robot]) -> i32 {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for [x, y, dx, dy] in input {
        let x = (x + 100 * dx).rem_euclid(101);
        let y = (y + 100 * dy).rem_euclid(103);

        if x < 50 {
            if y < 51 {
                q1 += 1;
            }
            if y > 51 {
                q2 += 1;
            }
        }
        if x > 50 {
            if y < 51 {
                q3 += 1;
            }
            if y > 51 {
                q4 += 1;
            }
        }
    }

    q1 * q2 * q3 * q4
}

pub fn part2(input: &[Robot]) -> usize {
    let robots: Vec<_> = input
        .iter()
        .map(|&[x, y, h, v]| [x, y, h.rem_euclid(101), v.rem_euclid(103)])
        .collect();

    let coefficient1 = 103 * 103.mod_inv(101).unwrap();
    let coefficient2 = 101 * 101.mod_inv(103).unwrap();
    let horizontal: Vec<_> = (0..101).map(|n| n.mod_inv(101)).collect();
    let vertical: Vec<_> = (0..103).map(|n| n.mod_inv(103)).collect();

    let mut unique = vec![true; 10403];

    for (i, &[x1, y1, h1, v1]) in robots.iter().enumerate().skip(1) {
        for &[x2, y2, h2, v2] in robots.iter().take(i) {
            if x1 == x2 && h1 == h2 {
                if let Some(b) = vertical[to_index(v2 - v1, 103)] {
                    let u = to_index((y1 - y2) * b, 103);

                    for n in (0..10403).step_by(103) {
                        unique[n + u] = false;
                    }
                }
            } else if y1 == y2 && v1 == v2 {
                if let Some(a) = horizontal[to_index(h2 - h1, 101)] {
                    let t = to_index((x1 - x2) * a, 101);

                    for n in (0..10403).step_by(101) {
                        unique[n + t] = false;
                    }
                }
            } else if let Some(a) = horizontal[to_index(h2 - h1, 101)] {
                if let Some(b) = vertical[to_index(v2 - v1, 103)] {
                    let t = (x1 - x2) * a;
                    let u = (y1 - y2) * b;
                    let crt = to_index(t * coefficient1 + u * coefficient2, 10403);
                    unique[crt] = false;
                }
            }
        }
    }

    unique.iter().position(|&u| u).unwrap()
}

#[inline]
fn to_index(a: i32, m: i32) -> usize {
    a.rem_euclid(m) as usize
}
