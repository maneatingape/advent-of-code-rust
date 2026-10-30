//! # Restroom Redoubt
//!
//! For part one we jump straight to the final position by multiplying the velocity by 100.
//! The image appears in part two when the positions of all robots are unique.
//!
//! The x coordinates repeat every 101 seconds and the y coordinates repeat every 103 seconds.
//! For part two, we can treat the coordinates independently, searching for a time when the
//! [variance](https://en.wikipedia.org/wiki/Variance) of the robots' coordinates is at a minimum.
//!
//! Using the [Chinese Remainder Theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem)
//! we combine the two times into a single time mod 10,403 that is the answer.
use std::cmp::Ordering::*;

use crate::util::iter::*;
use crate::util::parse::*;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

type Axis = Vec<(i32, i32)>;
type Input = (Axis, Axis);

pub fn parse(input: &str) -> Input {
    input
        .iter_signed::<i32>()
        .chunk::<4>()
        .map(|[x, y, dx, dy]| ((x, dx.rem_euclid(WIDTH)), (y, dy.rem_euclid(HEIGHT))))
        .collect()
}

pub fn part1(input: &Input) -> i32 {
    let (xs, ys) = input;
    let mut quadrants = [0; 4];

    for (&(x, dx), &(y, dy)) in xs.iter().zip(ys) {
        let x = (x + 100 * dx) % WIDTH;
        let y = (y + 100 * dy) % HEIGHT;

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

pub fn part2(input: &Input) -> i32 {
    let (xs, ys) = input;

    // Search for times modulo WIDTH when the tree could possibly exist using x coordinates only,
    // and times modulo HEIGHT when the tree could possibly exist using y coordinates only.
    let t = minimum_variance::<WIDTH>(xs);
    let u = minimum_variance::<HEIGHT>(ys);

    // Combine indices using the Chinese Remainder Theorem to get index mod 10,403.
    (5_253 * t + 5_151 * u) % (WIDTH * HEIGHT)
}

/// Calculate the [population variance](https://en.wikipedia.org/wiki/Variance#Population_variance)
/// for each time in the period, returning the time when at the minimum variance.
///
/// Const generics allow the compiler to optimize the modulus calculation, making it 3 times faster.
fn minimum_variance<const PERIOD: i32>(axis: &Axis) -> i32 {
    let n = axis.len() as i32;
    (0..PERIOD)
        .min_by_key(|time| {
            let mut sum = 0;
            let mut sum2 = 0;

            for &(position, velocity) in axis {
                let robot = (position + velocity * time) % PERIOD;
                sum += robot;
                sum2 += robot * robot;
            }

            // Variance scaled by n² in order to keep values integer.
            n * sum2 - sum * sum
        })
        .unwrap()
}
