use crate::util::grid::*;
use crate::util::parse::*;
use crate::util::point::*;

const UP: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const RIGHT: usize = 3;

pub fn parse(input: &str) -> Grid<i32> {
    let Grid { width, height, bytes } = Grid::parse(input);
    let bytes = bytes.iter().map(|b| b.to_decimal() as i32).collect();
    Grid { width, height, bytes }
}

pub fn part1(grid: &Grid<i32>) -> i32 {
    astar(grid, 0, 3)
}

pub fn part2(grid: &Grid<i32>) -> i32 {
    astar(grid, 3, 10)
}

fn astar(grid: &Grid<i32>, lower: i32, upper: i32) -> i32 {
    let size = grid.width - 1;
    let end = Point::new(size, size);

    let mut index = 0;
    let mut todo = vec![Vec::new(); 256];
    let mut seen: Grid<[i32; 4]> = grid.default_copy();

    todo[0].push((ORIGIN, RIGHT));
    todo[0].push((ORIGIN, DOWN));

    loop {
        while let Some((position, direction)) = todo[index % 256].pop() {
            let cost = seen[position][direction];

            let mut push = |direction: usize| {
                let step = ORTHOGONAL[direction];
                let mut next = position;
                let mut next_cost = cost;

                for _ in 0..lower {
                    next += step;
                    if !grid.contains(next) {
                        return;
                    }
                    next_cost += grid[next];
                }
                for _ in lower..upper {
                    next += step;
                    if !grid.contains(next) {
                        return;
                    }
                    next_cost += grid[next];

                    if seen[next][direction] == 0 || next_cost < seen[next][direction] {
                        let heuristic = (next_cost + next.manhattan(end)) as usize;
                        todo[heuristic % 256].push((next, direction));
                        seen[next][direction] = next_cost;
                    }
                }
            };

            if position == end {
                return cost;
            }

            if direction < 2 {
                push(LEFT);
                push(RIGHT);
            } else {
                push(UP);
                push(DOWN);
            }
        }

        index += 1;
    }
}
