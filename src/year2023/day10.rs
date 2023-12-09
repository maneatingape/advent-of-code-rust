use crate::util::grid::*;
use crate::util::point::*;
use std::collections::VecDeque;

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);

    let (mut position, mut direction) = find_start(&grid);
    let mut inner = direction.clockwise();

    let mut todo = VecDeque::new();
    let mut seen: Grid<bool> = grid.default_copy();
    let mut steps = 1;

    loop {
        let candidate = position + inner;
        if seen.contains(candidate) && !seen[candidate] {
            todo.push_back(candidate);
        }

        seen[position] = true;

        (direction, inner) = match grid[position] {
            b'-' | b'|' => (direction, inner),
            b'L' => {
                if direction == DOWN {
                    (RIGHT, inner.counter_clockwise())
                } else {
                    (UP, inner.clockwise())
                }
            }
            b'J' => {
                if direction == DOWN {
                    (LEFT, inner.clockwise())
                } else {
                    (UP, inner.counter_clockwise())
                }
            }
            b'7' => {
                if direction == UP {
                    (LEFT, inner.counter_clockwise())
                } else {
                    (DOWN, inner.clockwise())
                }
            }
            b'F' => {
                if direction == UP {
                    (RIGHT, inner.clockwise())
                } else {
                    (DOWN, inner.counter_clockwise())
                }
            }
            _ => break,
        };

        // Cover outer corners
        let candidate = position + inner;
        if seen.contains(candidate) && !seen[candidate] {
            todo.push_back(candidate);
        }

        position += direction;
        steps += 1;
    }

    let mut outer = false;
    let mut area = 0;

    while let Some(next) = todo.pop_front() {
        if seen[next] {
            continue;
        }

        seen[next] = true;
        area += 1;

        for o in ORTHOGONAL {
            let canidate = next + o;
            if !seen.contains(canidate) {
                outer = true;
                continue;
            }
            if !seen[canidate] {
                todo.push_back(canidate);
            }
        }
    }

    let part_one = steps / 2;
    let part_two = if outer { grid.bytes.len() - steps - area } else { area };
    (part_one, part_two)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

fn find_start(grid: &Grid<u8>) -> (Point, Point) {
    let start = grid.find(b'S').unwrap();

    if matches!(grid[start + UP], b'|' | b'7' | b'F') {
        return (start + UP, UP);
    }
    if matches!(grid[start + DOWN], b'|' | b'L' | b'J') {
        return (start + DOWN, DOWN);
    }
    if matches!(grid[start + LEFT], b'-' | b'L' | b'F') {
        return (start + LEFT, LEFT);
    }
    if matches!(grid[start + RIGHT], b'-' | b'7' | b'J') {
        return (start + RIGHT, RIGHT);
    }

    unreachable!()
}
