//! # Restroom Redoubt
//!
//! For part one we jump straight to the final position by multiplying the velocity by 100.
//! The image appears in part two when the positions of all robots are unique.
//!
//! The x coordinates repeat every 101 seconds and the y coordinates repeat every 103 seconds.
//! Calculating each axis independently then looking it up is twice as fast
//! as calculating as needed.
use crate::util::grid::*;
use crate::util::iter::*;
use crate::util::parse::*;
use crate::util::point::*;

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

pub fn part2(robots: &[Robot]) -> usize {
    let mut xs = vec![vec![0; robots.len()]; 101];
    let mut ys = vec![vec![0; robots.len()]; 103];
    let mut grid = Grid::new(101, 103, 0);

    for (time, row) in xs.iter_mut().enumerate() {
        for (i, [x, _, dx, _]) in robots.iter().enumerate() {
            row[i] = (x + dx * time as i32).rem_euclid(101);
        }
    }

    for (time, row) in ys.iter_mut().enumerate() {
        for (i, [_, y, _, dy]) in robots.iter().enumerate() {
            row[i] = (y + dy * time as i32).rem_euclid(103);
        }
    }

    'outer: for time in 1..10403 {
        for (&x, &y) in xs[time % 101].iter().zip(ys[time % 103].iter()) {
            let point = Point::new(x, y);
            if grid[point] == time {
                continue 'outer;
            }
            grid[point] = time;
        }

        return time;
    }

    unreachable!()
}
