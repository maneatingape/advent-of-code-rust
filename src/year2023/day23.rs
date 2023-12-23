use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;
use std::collections::VecDeque;

pub struct Input {
    directed: [u64; 36],
    undirected: [u64; 36],
    weight: [[u32; 36]; 36],
}

pub fn parse(input: &str) -> Input {
    let mut grid = Grid::parse(input);
    let width = grid.width;
    let height = grid.height;

    // Modify edge of grid to remove the need for boundary checks.
    grid[Point::new(1, 0)] = b'#';
    grid[Point::new(width - 2, height - 1)] = b'#';

    // Move start and end away from edge.
    let start = Point::new(1, 1);
    let end = Point::new(width - 2, height - 2);

    // Points of interest are start, end and junctions.
    grid[start] = b'P';
    grid[end] = b'P';

    let mut poi = FastMap::new();
    poi.insert(start, 0);
    poi.insert(end, 1);

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let position = Point::new(x, y);

            if grid[position] != b'#' {
                let neighbors =
                    ORTHOGONAL.iter().map(|&o| position + o).filter(|&n| grid[n] != b'#').count();
                if neighbors > 2 {
                    grid[position] = b'P';
                    poi.insert(position, poi.len());
                }
            }
        }
    }

    // BFS to find distances between POIs.
    let mut todo = VecDeque::new();
    let mut directed = [0; 36];
    let mut undirected = [0; 36];
    let mut weight = [[0; 36]; 36];

    for (&start, &from) in &poi {
        todo.push_back((start, 0, true));
        grid[start] = b'#';

        while let Some((position, cost, forward)) = todo.pop_front() {
            for direction in ORTHOGONAL {
                let next = position + direction;

                match grid[next] {
                    b'#' => (),
                    b'P' => {
                        let to = poi[&next];

                        if forward {
                            directed[from] |= 1 << to;
                        } else {
                            directed[to] |= 1 << from;
                        }

                        undirected[from] |= 1 << to;
                        undirected[to] |= 1 << from;

                        weight[from][to] = cost + 1;
                        weight[to][from] = cost + 1;
                    }
                    b'.' => {
                        todo.push_back((next, cost + 1, forward));
                        grid[next] = b'#';
                    }
                    _ => {
                        let same = direction == Point::from(grid[next]);
                        todo.push_back((next, cost + 1, forward && same));
                        grid[next] = b'#';
                    }
                }
            }
        }
    }

    Input { directed, undirected, weight }
}

pub fn part1(input: &Input) -> u32 {
    let mut cost = [0; 36];

    let mut todo = VecDeque::new();
    todo.push_back(0);

    while let Some(from) = todo.pop_front() {
        let mut nodes = input.directed[from];

        while nodes > 0 {
            let to = nodes.trailing_zeros() as usize;
            let mask = 1 << to;
            nodes ^= mask;

            cost[to] = cost[to].max(cost[from] + input.weight[from][to]);
            todo.push_back(to);
        }
    }

    2 + cost[1]
}

pub fn part2(input: &Input) -> u32 {
    2 + dfs(input, 0, 1, 0)
}

fn dfs(input: &Input, from: usize, seen: u64, cost: u32) -> u32 {
    if from == 1 {
        return cost;
    }

    let mut nodes = input.undirected[from] & !seen;
    let mut result = 0;

    while nodes > 0 {
        let to = nodes.trailing_zeros() as usize;
        let mask = 1 << to;
        nodes ^= mask;

        result = result.max(dfs(input, to, seen | mask, cost + input.weight[from][to]));
    }

    result
}
