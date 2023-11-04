use crate::util::hash::*;
use crate::util::parse::*;
use std::cmp::Ordering;
use std::collections::VecDeque;

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
}

struct Valve<'a> {
    name: &'a str,
    flow: u32,
    edges: Vec<&'a str>,
}

impl Valve<'_> {
    fn parse(line: &str) -> Valve<'_> {
        let mut tokens: Vec<_> = line
            .split(|c: char| !c.is_ascii_uppercase() && !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .collect();
        let name = tokens[1];
        let flow = tokens[2].unsigned();
        tokens.drain(..3);
        Valve { name, flow, edges: tokens }
    }

    fn cmp(&self, other: &Valve<'_>) -> Ordering {
        other.flow.cmp(&self.flow).then(self.name.cmp(other.name))
    }
}

pub fn parse(input: &str) -> Input {
    let mut valves: Vec<_> = input.lines().map(Valve::parse).collect();
    valves.sort_unstable_by(Valve::cmp);

    let size = valves.iter().filter(|v| v.flow > 0).count() + 1;
    let mut distance = vec![u32::MAX; size * size];
    let indices: FastMap<_, _> = valves.iter().enumerate().map(|(i, v)| (v.name, i)).collect();

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
    let flow: Vec<_> = valves.iter().take(size).map(|v| v.flow).collect();
    distance.iter_mut().for_each(|d| *d += 1);

    Input { size, todo, flow, distance }
}

pub fn part1(input: &Input) -> u32 {
    let mut score = 0;
    let mut queue = Vec::new();
    queue.push(State { todo: input.todo, from: input.size - 1, time: 30, pressure: 0 });

    while let Some(State { todo, from, time, pressure }) = queue.pop() {
        score = score.max(pressure);
        let mut valves = todo;

        while valves > 0 {
            let to = valves.trailing_zeros() as usize;
            let mask = 1 << to;
            valves ^= mask;

            let needed = input.distance[from * input.size + to];
            if needed < time {
                let remaining = time - needed;
                let flow = input.flow[to];

                let heuristic = {
                    let mut pressure = pressure + remaining * flow;
                    let mut valves = todo ^ mask;
                    let mut remaining = remaining;

                    while valves > 0 && remaining > 3 {
                        remaining -= 3;
                        let to = valves.trailing_zeros() as usize;
                        let flow = input.flow[to];
                        pressure += remaining * flow;
                        valves ^= 1 << to;
                    }

                    pressure
                };

                if heuristic > score {
                    let next = State {
                        todo: todo ^ mask,
                        from: to,
                        time: remaining,
                        pressure: pressure + remaining * flow,
                    };
                    queue.push(next);
                }
            }
        }
    }

    score
}

pub fn part2(input: &Input) -> u32 {
    let mut score = vec![0; input.todo + 1];
    let mut queue = VecDeque::with_capacity(input.todo);
    queue.push_back(State { todo: input.todo, from: input.size - 1, time: 26, pressure: 0 });

    let stride = score.len();
    let mut cache = vec![0; stride * (input.size - 1)];

    while let Some(State { todo, from, time, pressure }) = queue.pop_front() {
        let done = todo ^ input.todo;
        score[done] = score[done].max(pressure);

        let mut valves = todo;
        while valves > 0 {
            let to = valves.trailing_zeros() as usize;
            let mask = 1 << to;
            valves ^= mask;

            let needed = input.distance[from * input.size + to];
            if needed < time {
                let next = todo ^ mask;
                let remaining = time - needed;
                let flow = input.flow[to];
                let pressure = pressure + remaining * flow;

                let index = stride * to + next;
                if cache[index] == 0 || cache[index] < pressure {
                    cache[index] = pressure;
                    queue.push_back(State { todo: next, from: to, time: remaining, pressure });
                }
            }
        }
    }

    let mut result = 0;
    let mut visited: Vec<_> = score.iter().map(|&s| s > 0).collect();
    subsets(input.todo, &mut score, &mut visited);

    for (i, you) in score.iter().enumerate() {
        let elephant = score[input.todo ^ i];
        result = result.max(you + elephant);
    }

    result
}

fn subsets(todo: usize, score: &mut [u32], visited: &mut [bool]) -> u32 {
    let mut valves = todo;
    let mut max = 0;

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
