use crate::util::grid::*;
use crate::util::heap::*;
use crate::util::parse::*;
use crate::util::point::*;

const UP: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const RIGHT: usize = 3;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(grid: &Grid<u8>) -> u32 {
    dijkstra(grid, 0, 3)
}

pub fn part2(grid: &Grid<u8>) -> u32 {
    dijkstra(grid, 3, 10)
}

fn dijkstra(grid: &Grid<u8>, lower: u32, upper: u32) -> u32 {
    let end = Point::new(grid.width - 1, grid.height - 1);
    let mut todo = MinHeap::with_capacity(100_000);
    let mut seen: [Grid<u32>; 4] =
        [grid.default_copy(), grid.default_copy(), grid.default_copy(), grid.default_copy()];

    for start in [(ORIGIN, RIGHT), (ORIGIN, DOWN)] {
        todo.push(0, start);
    }

    while let Some((cost, (position, direction))) = todo.pop() {
        let mut push = |direction: usize| {
            let step = ORTHOGONAL[direction];
            let mut next = position;
            let mut next_cost = cost;

            for i in 0..upper {
                next += step;
                if !grid.contains(next) {
                    return;
                }
                next_cost += grid[next].to_decimal() as u32;

                if i >= lower {
                    let state = (next, direction);
                    if seen[direction][next] == 0 || next_cost < seen[direction][next] {
                        todo.push(next_cost, state);
                        seen[direction][next] = next_cost;
                    }
                }
            }
        };

        if position == end {
            return cost;
        }

        match direction {
            UP | DOWN => {
                push(LEFT);
                push(RIGHT);
            }
            LEFT | RIGHT => {
                push(UP);
                push(DOWN);
            }
            _ => unreachable!(),
        }
    }

    unreachable!()
}
