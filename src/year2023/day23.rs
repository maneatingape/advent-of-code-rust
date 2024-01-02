use crate::util::grid::*;
use crate::util::hash::*;
use crate::util::point::*;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

pub struct Input {
    start: usize,
    end: usize,
    extra: u32,
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
    let mut directed: [u64; 36] = [0; 36];
    let mut undirected: [u64; 36] = [0; 36];
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

    // Compress
    let start = undirected[0].trailing_zeros() as usize;
    let end = undirected[1].trailing_zeros() as usize;
    let extra = 2 + weight[0][start] + weight[1][end];

    // Heuristic
    let mut mask = 0;

    for (i, edges) in undirected.iter().enumerate() {
        if edges.count_ones() < 4 {
            mask |= 1 << i;
        }
    }

    for (i, edges) in undirected.iter_mut().enumerate() {
        if edges.count_ones() < 4 {
            *edges = (*edges & !mask) | directed[i];
        }
    }

    Input { start, end, extra, directed, undirected, weight }
}

pub fn part1(input: &Input) -> u32 {
    let mut cost = [0; 36];

    let mut todo = VecDeque::new();
    todo.push_back(input.start);

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

    cost[input.end] + input.extra
}

pub fn part2(input: &Input) -> u32 {
    let shared = AtomicU32::new(0);
    let threads = thread::available_parallelism().unwrap().get();

    // Seed each worker thread with a starting state
    let mut seeds = VecDeque::new();
    seeds.push_back((input.start, 1 << input.start, 0));

    while seeds.len() < threads {
        let Some((from, seen, cost)) = seeds.pop_front() else {
            break;
        };

        if from == input.end {
            shared.fetch_max(cost, Ordering::Relaxed);
            continue;
        }

        let mut nodes = input.undirected[from] & !seen;

        while nodes > 0 {
            let to = nodes.trailing_zeros() as usize;
            let mask = 1 << to;
            nodes ^= mask;

            seeds.push_back((to, seen | mask, cost + input.weight[from][to]));
        }
    }

    // Use as many cores as possible to parallelize the remaining search.
    thread::scope(|scope| {
        for start in &seeds {
            scope.spawn(|| worker(input, &shared, start));
        }
    });

    shared.load(Ordering::Relaxed) + input.extra
}

fn worker(input: &Input, shared: &AtomicU32, start: &(usize, u64, u32)) {
    let (from, seen, cost) = *start;
    let result = dfs(input, from, seen);
    shared.fetch_max(result + cost, Ordering::Relaxed);
}

fn dfs(input: &Input, from: usize, seen: u64) -> u32 {
    if from == input.end {
        return 0;
    }

    let mut nodes = input.undirected[from] & !seen;
    let mut result = 0;

    while nodes > 0 {
        let to = nodes.trailing_zeros() as usize;
        let mask = 1 << to;
        nodes ^= mask;

        result = result.max(input.weight[from][to] + dfs(input, to, seen | mask));
    }

    result
}
