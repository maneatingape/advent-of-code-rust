//! # Restroom Redoubt
//!
//! For part one we jump straight to the final position by multiplying the velocity by 100.
//! The image appears in part two when the positions of all robots are unique.
//!
//! The x coordinates repeat every 101 seconds and the y coordinates repeat every 103 seconds.
//! First we check for times when the robot x coordinates could form the left and right columns
//! of the tree's bounding box. This gives a time `t` mod 101.
//!
//! Then we check the y coordinates looking for the top and bottom rows of the bounding box,
//! giving a time `u` mod 103.
//!
//! Using the [Chinese Remainder Theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem)
//! we combine the two times into a single time mod 10403 that is the answer.
use crate::util::iter::*;
use crate::util::parse::*;
use std::cmp::Ordering::*;

type Robot = [usize; 4];

pub fn parse(input: &str) -> Vec<Robot> {
    input
        .iter_signed::<i32>()
        .chunk::<4>()
        .map(|[x, y, dx, dy]| {
            [x as usize, y as usize, dx.rem_euclid(101) as usize, dy.rem_euclid(103) as usize]
        })
        .collect()
}

pub fn part1(input: &[Robot]) -> i32 {
    let mut quadrants = [0; 4];

    for [x, y, dx, dy] in input {
        let x = (x + 100 * dx) % 101;
        let y = (y + 100 * dy) % 103;

        match (x.cmp(&50), y.cmp(&51)) {
            (Less, Less) => quadrants[0] += 1,
            (Less, Greater) => quadrants[1] += 1,
            (Greater, Less) => quadrants[2] += 1,
            (Greater, Greater) => quadrants[3] += 1,
            _ => (),
        }
    }

    quadrants.iter().product()
}

pub fn part2(robots: &[Robot]) -> usize {
    // Search for times mod 101 when the tree could possibly exist using x coordinates only.
    // and times mod 103 when the tree could possibly exist using y coordinates only.
    let mut rows = Vec::new();
    let mut columns = Vec::new();

    for time in 0..103 {
        let mut xs = [0; 101];
        let mut ys = [0; 103];

        for [x, y, dx, dy] in robots {
            let x = (x + time * dx) % 101;
            xs[x] += 1;
            let y = (y + time * dy) % 103;
            ys[y] += 1;
        }

        // Tree bounding box is 31x33.
        if time < 101 && xs.iter().filter(|&&c| c >= 33).count() >= 2 {
            columns.push(time);
        }
        if ys.iter().filter(|&&c| c >= 31).count() >= 2 {
            rows.push(time);
        }
    }

    // If there's only one combination then return answer.
    if rows.len() == 1 && columns.len() == 1 {
        let t = columns[0];
        let u = rows[0];
        // Combine indices using the Chinese Remainder Theorem to get index mod 10403.
        return (5253 * t + 5151 * u) % 10403;
    }

    // Backup check looking for time when all robot positions are unique.
    let mut floor = vec![0; 10403];

    for &t in &columns {
        'outer: for &u in &rows {
            let time = (5253 * t + 5151 * u) % 10403;

            for &[x, y, dx, dy] in robots {
                let x = (x + time * dx) % 101;
                let y = (y + time * dy) % 103;

                let index = 101 * y + x;
                if floor[index] == time {
                    continue 'outer;
                }
                floor[index] = time;
            }

            return time;
        }
    }

    unreachable!()
}
