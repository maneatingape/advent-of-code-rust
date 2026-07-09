//! # Chiton
//!
//! Traversing a graph with different non-negative edge weights is a job for the classic
//! [A* algorithm](https://www.redblobgames.com/pathfinding/a-star/introduction.html),
//! explained really well in the linked blog post.
//!
//! The simplest possible heuristic in A* is to make no estimate at all, making the search
//! behave the same as Dijkstra's algorithm. With that heuristic, the search frontier visits
//! nearly every tile, since only a few tiles near the target might have a total risk higher
//! than the target itself. Slightly better is a heuristic of the Manhattan distance to the
//! target; although this still underestimates and prunes no more than 5% of the total search
//! space. Logically this makes sense: Manhattan distance only changes by 1 per tile, but with risks
//! between 1-9, the average risk is closer to 5, and our best path averages closer to 3 risk per
//! tile. But we can do much better with a heuristic that prunes about 75% of the search space,
//! by assigning a value within 2% of the actual cost for each node. In the long run, it is
//! faster to visit all 250,000 to set up a fairly close estimate which lets us prune the search
//! space to under 60,000 nodes, than it is to skip the heuristic but have a search space near
//! 250,000 nodes, since the effort of searching is less predictable than the effort to compute
//! the heuristic.
//!
//! The heuristic we use builds up an estimate for the minimum cost incurred from each node to
//! the destination. The target itself starts with its risk level. Then for every diagonal line of
//! tiles, starting next to the target and ending next to the origin, a given tile's estimate is
//! chosen to be its own risk level plus the minimum of the tile below, the tile to the right,
//! or the minimum Manhattan distance to reach any other tile on the same diagonal with a better
//! estimate. Building this up requires traveling each diagonal twice (the first pass captures
//! any better tiles below and left, the second pass captures any tiles above and right). Allowing
//! other tiles on the same diagonal to influence the current tile's estimate covers the case where
//! the optimal path moves up or left around an obstacle. Failure to consider the ability to
//! reach other tiles on the same diagonal via an unseen path that loops around a wall would result
//! in a heuristic that is not admissible in A*. At the same time, without actually verifying
//! whether same-diagonal tiles can actually be reached in the estimated Manhattan distance, the
//! heuristic is no longer perfectly consistent, which means in practice we can sometimes see a
//! neighbor point inserted into the work queue with a priority one less than the current tile.
//!
//! With our heuristic, the maximum possible increase in risk is 9, compounded with the maximum
//! jump in the estimate table of another 9. A circular array of 32 buckets (for bitwise math
//! windowing) can handle our empirical range of -1 to 18 in the set of active buckets, and we
//! avoid having to allocate memory as the search gradually shifts the active window of buckets.
//!
//! [`BinaryHeap`]: std::collections::BinaryHeap
use crate::util::parse::*;
use std::array::from_fn;

pub struct Square {
    size: usize,
    bytes: Vec<u8>,
}

pub fn parse(input: &str) -> Square {
    let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
    let size = raw.len();
    let bytes = raw.into_iter().flatten().map(|b| b.to_decimal()).collect();
    Square { size, bytes }
}

/// Search the regular size grid.
pub fn part1(input: &Square) -> usize {
    astar(input, build_estimates(input))
}

/// Create an expanded grid then search.
pub fn part2(input: &Square) -> usize {
    let Square { size, bytes } = input;

    let mut expanded = Square { size: 5 * size, bytes: vec![0; 25 * size * size] };

    for (i, &b) in bytes.iter().enumerate() {
        let x1 = i % size;
        let y1 = i / size;
        let base = b as usize;

        for x2 in 0..5 {
            for y2 in 0..5 {
                let index = (5 * size) * (y2 * size + y1) + (x2 * size + x1);
                expanded.bytes[index] = (1 + (base - 1 + x2 + y2) % 9) as u8;
            }
        }
    }

    astar(&expanded, build_estimates(&expanded))
}

// Create a table of heuristics for use in an A* search. This is admissible (never overestimates)
// but not consistent (the diagonal clamping can sometimes change a node's estimated score by more
// than its risk, necessitating the search queue to jump back a bucket).
fn build_estimates(square: &Square) -> Vec<u32> {
    let Square { size, bytes } = square;
    let edge = size - 1;
    let end = size * size - 1;

    let mut estimate = vec![0_u32; size * size];
    // Produce a coordinate in estimate, or 0 if x or y out of bounds. Since the origin does
    // not contribute to the overall risk level, we use it instead to hold an effective infinity
    // to make processing easier at the ends of diagonals.
    let coord = |col: usize, row: usize| -> usize {
        if col > edge || row > edge { 0 } else { row * size + col }
    };
    estimate[0] = u32::MAX; // Larger than any possible other estimate.

    // Give the target its own risk level.
    estimate[end] = bytes[end] as u32;

    // Visit the grid by diagonals, starting closest to the target.
    for diag in (1..edge * 2).rev() {
        let mut best_diag = u32::MAX - 18;
        let range: Vec<usize> = (diag.saturating_sub(edge)..edge.min(diag) + 1).collect();

        // For each tile crawling up and right, select the minimum between its lower neighbor,
        // its right neighbor, or the minimum Manhattan distance to any earlier node on the diagonal.
        for &col in &range {
            let row = diag - col;
            let value =
                estimate[coord(col + 1, row)].min(estimate[coord(col, row + 1)]).min(best_diag + 2)
                    + bytes[coord(col, row)] as u32;
            estimate[coord(col, row)] = value;
            best_diag = if best_diag + 2 < value { best_diag + 2 } else { value };
        }

        // For each tile crawling down and left, also check for the minimum Manhattan distance from
        // any better node earlier on the diagonal.
        best_diag = u32::MAX - 18;
        for &col in range.iter().rev() {
            let row = diag - col;
            let value =
                estimate[coord(col, row)].min(best_diag + 2 + bytes[coord(col, row)] as u32);
            estimate[coord(col, row)] = value;
            best_diag = if best_diag + 2 < value { best_diag + 2 } else { value };
        }
    }

    // The final estimate for the origin is about 2% shy of the actual risk level.
    estimate[0] = estimate[1].min(estimate[*size]);
    estimate
}

/// Implementation of [A* algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm)
/// without using the decrease-key functionality.
fn astar(square: &Square, mut grid_data: Vec<u32>) -> usize {
    let Square { size, bytes } = square;
    let edge = size - 1;
    let end = size * size - 1;

    // Initialize our specialized priority queue with 32 vecs. Chosen to be large enough
    // to cover the largest gap (actual risk increasing by 9 on the same step that the
    // heuristic jumps by 9), but also safe against the infrequent backwards jump by 1.
    let mut todo: [Vec<u32>; 32] = from_fn(|_| Vec::with_capacity(1_000));

    // On entry, grid_data contains estimates in the low 16 bits. As long as all estimates
    // are transformed uniformly by a constant, the sequence of nodes visited will be identical.
    // For best memory use, we prefer operating with the estimates in the high 16 bits,
    // and the cost to reach a node in the low 16 bits, with the initial cost estimate of
    // u16::MAX as a sentinel that the node has not been visited yet.
    for cell in &mut grid_data {
        *cell = (*cell << 16) - 1;
    }

    // Start location and risk are both zero.
    let mut i = (grid_data[0] >> 16) as usize;
    todo[i & 31].push(0);
    grid_data[0] = 0;

    loop {
        while todo[i & 31].is_empty() {
            i += 1;
        }

        if let Some(current) = todo[i & 31].pop() {
            let current = current as usize;
            let risk = (grid_data[current] & 0xffff) as usize;
            if current == end {
                return risk;
            }

            let mut check = |next: usize| {
                let next_cost = risk + bytes[next] as usize;
                if next_cost < grid_data[next] as usize & 0xffff {
                    let next_f = risk + (grid_data[next] >> 16) as usize;
                    // Cope if this resulted in the rare backward jump.
                    i = i.min(next_f);
                    todo[next_f & 31].push(next as u32);
                    grid_data[next] = (grid_data[next] & !0xffff) | next_cost as u32;
                }
            };
            let x = current % size;
            let y = current / size;

            if x > 0 {
                check(current - 1);
            }
            if x < edge {
                check(current + 1);
            }
            if y > 0 {
                check(current - size);
            }
            if y < edge {
                check(current + size);
            }
        }
    }
}
