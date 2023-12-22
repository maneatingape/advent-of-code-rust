use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

type Input = [u64; 4];

pub fn parse(input: &str) -> Input {
    let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
    let mut bytes = Vec::with_capacity(655 * 655);

    for row in 0..655 {
        for _ in 0..5 {
            bytes.extend_from_slice(raw[row % 131]);
        }
    }

    let start = Point::new(327, 327);

    let mut grid = Grid { width: 655, height: 655, bytes };
    grid[start] = b'#';

    let mut total = vec![0; 328];
    total[0] = 1;

    let mut todo = VecDeque::new();
    todo.push_back((start, 0));

    while let Some((position, cost)) = todo.pop_front() {
        for o in ORTHOGONAL {
            let next = position + o;

            if grid[next] != b'#' {
                grid[next] = b'#';

                let cost = cost + 1;
                total[cost] += 1;

                if cost < 327 {
                    todo.push_back((next, cost));
                }
            }
        }
    }

    let sum = |limit: usize| total.iter().take(limit + 1).skip(limit % 2).step_by(2).sum();
    [65, 196, 327, 64].map(sum)
}

pub fn part1(input: &Input) -> u64 {
    input[3]
}

pub fn part2(f: &Input) -> u64 {
    // f(x) = ax² + bx + c = plots reachable in 65 + 131 * x steps
    let a = (f[0] + f[2] - 2 * f[1]) / 2;
    let b = (4 * f[1] - 3 * f[0] - f[2]) / 2;
    let c = f[0];

    // n = number of whole grid widths = (26501365 - 65) / 131
    let n = 202300;
    a * n * n + b * n + c
}
