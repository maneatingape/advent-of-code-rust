use crate::util::chunk::*;
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
