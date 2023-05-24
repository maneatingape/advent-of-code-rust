use crate::util::parse::*;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

pub struct Input {
    size: usize,
    todo: usize,
    flow: Vec<u32>,
    distance: Vec<u32>,
}

struct State {
    todo: usize,
    from: usize,
    time: u32,
    pressure: u32,
    unopened: u32,
}

pub struct Valve<'a> {
    name: &'a str,
    flow: u32,
    edges: Vec<&'a str>,
}

impl Valve<'_> {
    fn parse(line: &str) -> Valve {
        let mut tokens: Vec<&str> = line
            .split(|c: char| !c.is_ascii_uppercase() && !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .collect();
        let name = tokens[1];
        let flow = from(tokens[2]);
        tokens.drain(..3);
        Valve { name, flow, edges: tokens }
    }

    fn cmp(&self, other: &Valve) -> Ordering {
        let first = other.flow.cmp(&self.flow);
        if first != Ordering::Equal {
            first
        } else {
            self.name.cmp(other.name)
        }
    }
}

pub fn parse(input: &str) -> Input {
    let mut valves: Vec<Valve> = input.lines().map(Valve::parse).collect();
    valves.sort_unstable_by(|a, b| a.cmp(b));

    let size = valves.iter().filter(|v| v.flow > 0).count() + 1;
    let mut distance = vec![u32::MAX; size * size];
    let indices: HashMap<&str, usize> =
        valves.iter().enumerate().map(|(i, v)| (v.name, i)).collect();

    // Eliminate zero valves
    for (from, valve) in valves.iter().enumerate().take(size) {
        distance[from * size + from] = 0;
        for edge in &valve.edges {
            let mut prev = valve.name;
            let mut cur = edge;
            let mut to = indices[cur];
            let mut total = 1;

            while to >= size {
                let next = valves[to].edges.iter().find(|&&e| e != prev).unwrap();
                prev = cur;
                cur = next;
                to = indices[cur];
                total += 1;
            }

            distance[from * size + to] = total;
        }
    }

    // Floyd-Warshall
    for k in 0..size {
        for i in 0..size {
            for j in 0..size {
                let candidate = distance[i * size + k].saturating_add(distance[k * size + j]);
                if candidate < distance[i * size + j] {
                    distance[i * size + j] = candidate;
                }
            }
        }
    }

    let todo = (1 << (size - 1)) - 1;
    let flow: Vec<u32> = valves.iter().take(size).map(|v| v.flow).collect();
    distance.iter_mut().for_each(|d| *d += 1);

    Input { size, todo, flow, distance }
}

pub fn part1(input: &Input) -> u32 {
    let mut score = 0;
    let mut queue = VecDeque::with_capacity(input.todo);
    queue.push_back(State {
        todo: input.todo,
        from: input.size - 1,
        time: 30,
        pressure: 0,
        unopened: input.flow.iter().sum(),
    });

    while let Some(State { todo, from, time, pressure, unopened }) = queue.pop_front() {
        score = score.max(pressure);
        let mut valves = todo;

        while valves != 0 {
            let to = valves.trailing_zeros() as usize;
            let mask = 1 << to;
            valves ^= mask;
            let needed = input.distance[from * input.size + to];
            if needed < time {
                let remaining = time - needed;
                if pressure + remaining * unopened > score {
                    let flow = input.flow[to];
                    let next = State {
                        todo: todo ^ mask,
                        from: to,
                        time: remaining,
                        pressure: pressure + remaining * flow,
                        unopened: unopened - flow,
                    };
                    queue.push_back(next);
                }
            }
        }
    }

    score
}

pub fn part2(input: &Input) -> u32 {
    let mut score = vec![0; input.todo + 1];
    let mut queue = VecDeque::with_capacity(input.todo);
    queue.push_back(State {
        todo: input.todo,
        from: input.size - 1,
        time: 26,
        pressure: 0,
        unopened: 0,
    });

    while let Some(State { todo, from, time, pressure, .. }) = queue.pop_front() {
        let done = todo ^ input.todo;
        score[done] = score[done].max(pressure);
        let mut valves = todo;

        while valves != 0 {
            let to = valves.trailing_zeros() as usize;
            let mask = 1 << to;
            valves ^= mask;
            let needed = input.distance[from * input.size + to];
            if needed < time {
                let remaining = time - needed;
                let flow = input.flow[to];
                let next = State {
                    todo: todo ^ mask,
                    from: to,
                    time: remaining,
                    pressure: pressure + remaining * flow,
                    unopened: 0,
                };
                queue.push_back(next);
            }
        }
    }

    let mut result = 0;
    let mut visited: Vec<bool> = score.iter().map(|&s| s > 0).collect();
    subsets(input.todo, &mut score, &mut visited);

    for (i, you) in score.iter().enumerate() {
        let elephant = score[input.todo ^ i];
        result = result.max(you + elephant);
    }

    result
}

fn subsets(todo: usize, score: &mut [u32], visited: &mut [bool]) -> u32 {
    let mut valves = todo;
    let mut max = score[todo];

    while valves != 0 {
        let mask = 1 << valves.trailing_zeros();
        let next = todo ^ mask;
        let result = if visited[next] { score[next] } else { subsets(next, score, visited) };
        valves ^= mask;
        max = max.max(result);
    }

    score[todo] = max;
    visited[todo] = true;
    max
}
