use crate::util::grid::*;
use crate::util::point::*;

#[derive(Clone, Copy)]
struct State {
    point: Point,
    risk: u16,
}

struct PriorityQueue {
    todo: Vec<Vec<State>>,
    head: usize,
}

impl PriorityQueue {
    fn new() -> PriorityQueue {
        PriorityQueue {
            todo: vec![vec![]; 10_000],
            head: 0,
        }
    }

    fn pop(&mut self) -> State {
        while self.todo[self.head].is_empty() {
            self.head += 1;
        }
        self.todo[self.head].pop().unwrap()
    }

    fn push(&mut self, state: State) {
        self.head = self.head.min(state.risk as usize);
        self.todo[state.risk as usize].push(state);
    }
}

pub fn parse(input: &str) -> Grid<u8> {
    let mut grid = Grid::parse(input);
    grid.bytes.iter_mut().for_each(|b| *b -= 48);
    grid
}

pub fn part1(input: &Grid<u8>) -> u16 {
    dijkstra(input)
}

pub fn part2(input: &Grid<u8>) -> u16 {
    let mut expanded = Grid {
        width: 5 * input.width,
        height: 5 * input.height,
        bytes: vec![0u8; 25 * input.bytes.len()],
    };

    for x1 in 0..input.width {
        for y1 in 0..input.height {
            let point = Point { x: x1, y: y1 };
            let base = input[point] as i32;

            for x2 in 0..5 {
                for y2 in 0..5 {
                    let point = Point {
                        x: x2 * input.width + x1,
                        y: y2 * input.height + y1,
                    };
                    expanded[point] = (1 + (base - 1 + x2 + y2) % 9) as u8;
                }
            }
        }
    }

    dijkstra(&expanded)
}

fn dijkstra(grid: &Grid<u8>) -> u16 {
    let start = State {
        point: ORIGIN,
        risk: 0,
    };
    let end = Point {
        x: grid.width - 1,
        y: grid.height - 1,
    };

    let mut todo = PriorityQueue::new();
    let mut cost = grid.default_copy::<Option<u16>>();
    todo.push(start);

    loop {
        let State { point, risk } = todo.pop();

        if point == end {
            return risk;
        }

        for next in ORTHOGONAL
            .iter()
            .map(|&n| point + n)
            .filter(|&n| grid.contains(n))
        {
            let next_cost = risk + grid[next] as u16;
            if cost[next].is_none() || next_cost < cost[next].unwrap() {
                let next_state = State {
                    point: next,
                    risk: next_cost,
                };
                todo.push(next_state);
                cost[next] = Some(next_cost);
            }
        }
    }
}
