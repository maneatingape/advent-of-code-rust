use crate::util::iter::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<[i128; 6]> {
    input.iter_signed().chunk::<6>().collect()
}

pub fn part1(input: &[[i128; 6]]) -> u32 {
    let mut result = 0;
    let lower = 200_000_000_000_000;
    let upper = 400_000_000_000_000;

    for i in 1..input.len() {
        for j in 0..i {
            let [x1, y1, _, vx1, vy1, _] = input[i];
            let [x2, y2, _, vx2, vy2, _] = input[j];

            let (a1, b1, c1) = line(x1, y1, vx1, vy1);
            let (a2, b2, c2) = line(x2, y2, vx2, vy2);

            let determinant = a1 * b2 - a2 * b1;
            if determinant == 0 {
                // Trajectories are parallel.
                continue;
            }

            let x3 = (b1 * c2 - b2 * c1) / determinant;
            let y3 = (a1 * c2 - a2 * c1) / determinant;

            if (lower <= x3 && x3 <= upper)
                && (lower <= y3 && y3 <= upper)
                && dot(x3 - x1, y3 - y1, vx1, vy1) > 0
                && dot(x3 - x2, y3 - y2, vx2, vy2) > 0
            {
                result += 1;
            }
        }
    }

    result
}

pub fn part2(_input: &[[i128; 6]]) -> &'static str {
    "n/a"
}

fn line(x1: i128, y1: i128, vx1: i128, vy1: i128) -> (i128, i128, i128) {
    let x2 = x1 + vx1;
    let y2 = y1 + vy1;

    let a = -vy1;
    let b = -vx1;
    let c = x1 * y2 - x2 * y1;

    (a, b, c)
}

fn dot(x1: i128, y1: i128, x2: i128, y2: i128) -> i128 {
    x1 * x2 + y1 * y2
}
