use crate::util::parse::*;
use std::cmp::Ordering;
use std::collections::HashMap;

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
        if first != Ordering::Equal { first } else { self.name.cmp(other.name) }
    }
}

pub struct Input {
    size: usize,
    todo: u32,
    flow: Vec<u32>,
    distance: Vec<u32>,
}

pub fn parse(input: &str) -> Input {
    let mut valves: Vec<Valve> = input.lines().map(Valve::parse).collect();
    valves.sort_unstable_by(|a, b| a.cmp(b));

    let size = valves.iter().filter(|v| v.flow > 0).count() + 1;
    let mut distance = vec![u32::MAX; size * size];
    let indices: HashMap<&str, usize> = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name, i))
        .collect();

    // Eliminate zero valves
    for (from, valve) in valves.iter().enumerate().take(size) {
        distance[from * size + from] = 0;
        for edge in valve.edges.iter() {
            let mut prev = valve.name;
            let mut cur = edge;
            let mut to = *indices.get(cur).unwrap();
            let mut total = 1;

            while to >= size {
                let next = valves[to].edges.iter().find(|&&e| e != prev).unwrap();
                prev = cur;
                cur = next;
                to = *indices.get(cur).unwrap();
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

    let todo: u32 = (1 << (size - 1)) - 1;
    let flow: Vec<u32> = valves.iter().take(size).map(|v| v.flow).collect();
    distance.iter_mut().for_each(|d| *d += 1);

    Input { size, todo, flow, distance }
}

pub fn part1(input: &Input) -> u32 {
    let mut score = vec![0; 1 << (input.size - 1)];
    explore(input, input.todo, 0, input.size - 1, 30, 0, &mut score);
    *score.iter().max().unwrap()
}

pub fn part2(input: &Input) -> u32 {
    let mut score = vec![0; 1 << (input.size - 1)];
    explore(input, input.todo, 0, input.size - 1, 26, 0, &mut score);

    let mut visited = vec![false; 1 << (input.size - 1)];
    subsets(input.todo, input.size - 1, &mut score, &mut visited);

    let mut result = 0;
    for i in 0..=input.todo {
        let you = i as usize;
        let elephant = (input.todo ^ i) as usize;
        result = result.max(score[you] + score[elephant]);
    }
    result
}

fn explore(input: &Input, todo: u32, done: u32, from: usize, time: u32, pressure: u32, score: &mut Vec<u32>) {
    score[done as usize] = score[done as usize].max(pressure);

    for to in 0..(input.size - 1) {
        let mask = 1 << to;
        if todo & mask != 0 {
            let needed = input.distance[from * input.size + to];
            if needed < time {
                let remaining = time - needed;
                let extra = remaining * input.flow[to];
                explore(input, todo ^ mask, done ^ mask, to, remaining, pressure + extra, score);
            }
        }
    }
}

fn subsets(todo: u32, size: usize, score: &mut [u32], visited: &mut [bool]) -> u32 {
    let index = todo as usize;
    if visited[index] {
        score[index]
    } else {
        let mut max = score[index];

        for to in 0..size {
            let mask = 1 << to;
            if todo & mask != 0 {
                max = max.max(subsets(todo ^ mask, size, score, visited));
            }
        }

        score[index] = max;
        visited[index] = true;
        max
    }
}
