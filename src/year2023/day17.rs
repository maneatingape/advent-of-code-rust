use crate::util::grid::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Grid<u32> {
    let Grid { width, height, bytes } = Grid::parse(input);
    let bytes = bytes.iter().map(|b| b.to_decimal() as u32).collect();
    Grid { width, height, bytes }
}

pub fn part1(grid: &Grid<u32>) -> u32 {
    astar::<1, 3>(grid)
}

pub fn part2(grid: &Grid<u32>) -> u32 {
    astar::<4, 10>(grid)
}

fn astar<const L: usize, const U: usize>(grid: &Grid<u32>) -> u32 {
    let width = grid.width as usize;
    let height = grid.height as usize;
    let heat = &grid.bytes;

    let mut index = 0;
    let mut todo = vec![Vec::with_capacity(100); 128];
    let mut seen = vec![[0_u32; 2]; heat.len()];

    todo[0].push((0, 0, 0));
    todo[0].push((0, 0, 1));

    loop {
        while let Some((x, y, direction)) = todo[index % 128].pop() {
            let index = width * y + x;
            let cost = seen[index][direction];
            let heuristic =
                |x: usize, y: usize, cost: u32| (cost as usize + width - x + height - y) % 128;

            if x == width - 1 && y == height - 1 {
                return cost;
            }

            if direction == 0 {
                // Left
                {
                    let mut index = index;
                    let mut cost = cost;

                    for i in 1..=U {
                        if i > x {
                            break;
                        }

                        index -= 1;
                        cost += heat[index];

                        if i >= L && (seen[index][1] == 0 || cost < seen[index][1]) {
                            todo[heuristic(x - i, y, cost)].push((x - i, y, 1));
                            seen[index][1] = cost;
                        }
                    }
                }

                // Right
                {
                    let mut index = index;
                    let mut cost = cost;

                    for i in 1..=U {
                        if x + i >= width {
                            break;
                        }

                        index += 1;
                        cost += heat[index];

                        if i >= L && (seen[index][1] == 0 || cost < seen[index][1]) {
                            todo[heuristic(x + i, y, cost)].push((x + i, y, 1));
                            seen[index][1] = cost;
                        }
                    }
                }
            } else {
                // Up
                {
                    let mut index = index;
                    let mut cost = cost;

                    for i in 1..=U {
                        if i > y {
                            break;
                        }

                        index -= width;
                        cost += heat[index];

                        if i >= L && (seen[index][0] == 0 || cost < seen[index][0]) {
                            todo[heuristic(x, y - i, cost)].push((x, y - i, 0));
                            seen[index][0] = cost;
                        }
                    }
                }

                // Down
                {
                    let mut index = index;
                    let mut cost = cost;

                    for i in 1..=U {
                        if y + i >= height {
                            break;
                        }

                        index += width;
                        cost += heat[index];

                        if i >= L && (seen[index][0] == 0 || cost < seen[index][0]) {
                            todo[heuristic(x, y + i, cost)].push((x, y + i, 0));
                            seen[index][0] = cost;
                        }
                    }
                }
            }
        }

        index += 1;
    }
}
