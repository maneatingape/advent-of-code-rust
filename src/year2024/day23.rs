//! # LAN Party
//!
//! This is the [Clique problem](https://en.wikipedia.org/wiki/Clique_problem). For part one we
//! find triangles (cliques of size 3) for each node by checking if there's an edge between any
//! distinct pair of neighbouring nodes.
//!
//! Part two asks to find the maximum clique, for which we could use the
//! [Bron–Kerbosch algorithm](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm).
//! However the input has a specific structure that enables a simpler approach of finding the
//! largest *maximal* clique using a greedy algorithm. Nodes are arranged in clusters of 13 and
//! the maximum clique is size 14. This approach will not necessarily work for any general graph,
//! but will work for the inputs provided.
use crate::util::hash::*;

type Input = (FastMap<usize, Vec<usize>>, Vec<[bool; 676]>);

/// Convert each character pair `xy` to an index from 0..676 so that we can use much faster array
/// lookup instead of a `HashMap`.
pub fn parse(input: &str) -> Input {
    let mut nodes = FastMap::with_capacity(1_000);
    let mut edges = vec![[false; 676]; 676];

    let to_index = |b: &[u8]| 26 * to_usize(b[0]) + to_usize(b[1]);
    let empty = || Vec::with_capacity(16);

    for edge in input.as_bytes().chunks(6) {
        let from = to_index(&edge[..2]);
        let to = to_index(&edge[3..]);

        // Graph is undirected so add edges to both nodes.
        nodes.entry(from).or_insert_with(empty).push(to);
        nodes.entry(to).or_insert_with(empty).push(from);

        // https://en.wikipedia.org/wiki/Adjacency_matrix
        edges[from][to] = true;
        edges[to][from] = true;
    }

    (nodes, edges)
}

pub fn part1(input: &Input) -> usize {
    let (nodes, edges) = input;
    let mut seen = [false; 676];
    let mut triangles = 0;

    // Only consider nodes starting with `t`.
    for n1 in 494..520 {
        if let Some(neighbours) = nodes.get(&n1) {
            seen[n1] = true;

            for (i, &n2) in neighbours.iter().enumerate() {
                for &n3 in neighbours.iter().skip(i) {
                    // Skip nodes if we've already seen them.
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

    // Greedy algorithm to find *maximal* (not maximum) cliques.
    for (&n1, neighbours) in nodes {
        if !seen[n1] {
            clique.clear();
            clique.push(n1);

            // Add nodes if they're connected to every node already in the clique.
            for &n2 in neighbours {
                if clique.iter().all(|&c| edges[n2][c]) {
                    seen[n2] = true;
                    clique.push(n2);
                }
            }

            // For the specific graphs given in the input
            // finding the largest maximal clique will work.
            if clique.len() > largest.len() {
                largest.clone_from(&clique);
            }
        }
    }

    // Convert each index back into 2 character identifiers sorted alphabetically.
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
