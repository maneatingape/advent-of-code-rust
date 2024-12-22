//! # LAN Party
use crate::util::hash::*;

type Input = (FastMap<usize, Vec<usize>>, Vec<[bool; 676]>);

pub fn parse(input: &str) -> Input {
    let mut nodes = FastMap::with_capacity(1_000);
    let mut edges = vec![[false; 676]; 676];

    let to_index = |b: &[u8]| 26 * to_usize(b[0]) + to_usize(b[1]);
    let empty = || Vec::with_capacity(16);

    for edge in input.as_bytes().chunks(6) {
        let from = to_index(&edge[..2]);
        let to = to_index(&edge[3..]);

        nodes.entry(from).or_insert_with(empty).push(to);
        nodes.entry(to).or_insert_with(empty).push(from);

        edges[from][to] = true;
        edges[to][from] = true;
    }

    (nodes, edges)
}

pub fn part1(input: &Input) -> usize {
    let (nodes, edges) = input;
    let mut seen = [false; 676];
    let mut triangles = 0;

    for n1 in 494..520 {
        if let Some(neighbours) = nodes.get(&n1) {
            seen[n1] = true;

            for (i, &n2) in neighbours.iter().enumerate() {
                for &n3 in neighbours.iter().skip(i) {
                    if !seen[n2] && !seen[n3] && edges[n2][n3] {
                        triangles += 1;
                    }
                }
            }
        }
    }

    triangles
}

pub fn part2(input: &Input) -> String {
    let (nodes, edges) = input;
    let mut seen = [false; 676];
    let mut clique = Vec::new();
    let mut largest = Vec::new();

    for (&n1, neighbours) in nodes {
        if !seen[n1] {
            clique.clear();
            clique.push(n1);

            for &n2 in neighbours {
                if clique.iter().all(|&c| edges[n2][c]) {
                    seen[n2] = true;
                    clique.push(n2);
                }
            }

            if clique.len() > largest.len() {
                largest.clone_from(&clique);
            }
        }
    }

    let mut result = String::new();
    largest.sort_unstable();

    for n in largest {
        result.push(to_char(n / 26));
        result.push(to_char(n % 26));
        result.push(',');
    }

    result.pop();
    result
}

fn to_usize(b: u8) -> usize {
    (b - b'a') as usize
}

fn to_char(u: usize) -> char {
    ((u as u8) + b'a') as char
}
