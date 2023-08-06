//! # Chiton
//!
//! Traversing a graph with different non-negative edge weights is a job for the classic
//! [Djisktra's algorithm](https://www.redblobgames.com/pathfinding/a-star/introduction.html),
//! explained really well in the linked blog post.
//!
//! To speed things up we use a trick. Classic Djisktra uses a generic priority queue that
//! can be implemented in Rust using a [`BinaryHeap`]. However the total cost follows a strictly
//! increasing order in a constrained range of values, so we can use a much faster single purpose
//! data structure instead.
//!
//! The maximum possible increase in risk in 9, so we create an array of 10 `vec`s. The current
//! list of items to process is at `risk % 10` and each new item is added at `risk % 10 + new_cost`.
//! Once we have processed the current risk level we clear the vec to avoid having to reallocate
//! memory.
//!
//! [`BinaryHeap`]: std::collections::BinaryHeap
use std::array::from_fn;

pub struct Square {
    size: usize,
    bytes: Vec<u8>,
}

pub fn parse(input: &str) -> Square {
    let raw: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let size = raw.len();
    let mut bytes = Vec::with_capacity(size * size);

    raw.iter().for_each(|slice| bytes.extend_from_slice(slice));
    bytes.iter_mut().for_each(|b| *b -= b'0');

    Square { size, bytes }
}

/// Search the regular size grid.
pub fn part1(input: &Square) -> usize {
    dijkstra(input)
}

/// Create an expanded grid then search.
pub fn part2(input: &Square) -> usize {
    let Square { size, bytes } = input;

    let mut expanded = Square { size: 5 * size, bytes: vec![0; 25 * size * size] };

    for (i, b) in bytes.iter().enumerate() {
        let x1 = i % size;
        let y1 = i / size;
        let base = *b as usize;

        for x2 in 0..5 {
            for y2 in 0..5 {
                let index = (5 * size) * (y2 * size + y1) + (x2 * size + x1);
                expanded.bytes[index] = (1 + (base - 1 + x2 + y2) % 9) as u8;
            }
        }
    }

    dijkstra(&expanded)
}

/// Implementation of [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
/// without using the decrease-key functionality.
fn dijkstra(square: &Square) -> usize {
    let Square { size, bytes } = square;
    let edge = size - 1;
    let end = size * size - 1;

    // Initialise our specialized priority queue with 10 vecs.
    let mut todo: [Vec<u32>; 10] = from_fn(|_| Vec::with_capacity(1_000));
    let mut cost = vec![u16::MAX; size * size];
    let mut risk = 0;

    // Start location and risk are both zero.
    todo[0].push(0);
    cost[0] = 0;

    loop {
        let i = risk % 10;

        for j in 0..todo[i].len() {
            let current = todo[i][j] as usize;
            if current == end {
                return risk;
            }

            let mut check = |next: usize| {
                let next_cost = risk as u16 + bytes[next] as u16;
                if next_cost < cost[next] {
                    todo[(next_cost % 10) as usize].push(next as u32);
                    cost[next] = next_cost;
                }
            };
            let x = current % size;
            let y = current / size;

            if x > 0 {
                check(current - 1)
            }
            if x < edge {
                check(current + 1)
            }
            if y > 0 {
                check(current - size)
            }
            if y < edge {
                check(current + size)
            }
        }

        todo[i].clear();
        risk += 1;
    }
}
