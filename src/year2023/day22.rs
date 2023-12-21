#![allow(clippy::needless_range_loop)]

use crate::util::iter::*;
use crate::util::parse::*;
use std::collections::VecDeque;

pub struct Input {
    up: Vec<Vec<usize>>,
    down: Vec<Vec<usize>>,
}

pub fn parse(input: &str) -> Input {
    let mut bricks: Vec<_> = input.iter_unsigned::<usize>().chunk::<6>().collect();
    let mut heights = [[0; 10]; 10];
    let mut indices = [[usize::MAX; 10]; 10];
    let mut up = vec![Vec::new(); bricks.len()];
    let mut down = vec![Vec::new(); bricks.len()];

    // Sort ascending by lowest z coordinate.
    bricks.sort_unstable_by_key(|b| b[2]);

    for (i, &[x1, y1, z1, x2, y2, z2]) in bricks.iter().enumerate() {
        let height = z2 - z1 + 1;
        let mut top = 0;
        let mut previous = usize::MAX;

        for x in x1..=x2 {
            for y in y1..=y2 {
                top = top.max(heights[x][y]);
            }
        }

        for x in x1..=x2 {
            for y in y1..=y2 {
                if heights[x][y] == top {
                    let index = indices[x][y];
                    if index != previous {
                        up[index].push(i);
                        down[i].push(index);
                        previous = index;
                    }
                }

                heights[x][y] = top + height;
                indices[x][y] = i;
            }
        }
    }

    Input { up, down }
}

pub fn part1(input: &Input) -> usize {
    let Input { down, .. } = input;
    let mut safe = vec![true; down.len()];

    for underneath in down {
        if underneath.len() == 1 {
            safe[underneath[0]] = false;
        }
    }

    safe.iter().filter(|&&b| b).count()
}

pub fn part2(input: &Input) -> usize {
    let Input { up, down } = input;
    let mut safe = vec![true; down.len()];

    for underneath in down {
        if underneath.len() == 1 {
            safe[underneath[0]] = false;
        }
    }

    let mut result = 0;
    let mut todo = VecDeque::new();
    let mut removed = vec![usize::MAX; down.len()];

    for (start, &safe) in safe.iter().enumerate() {
        if safe {
            continue;
        }

        todo.push_back(start);
        removed[start] = start;

        while let Some(current) = todo.pop_front() {
            for &next in &up[current] {
                if removed[next] != start && down[next].iter().all(|&i| removed[i] == start) {
                    result += 1;
                    removed[next] = start;
                    todo.push_back(next);
                }
            }
        }
    }

    result
}
