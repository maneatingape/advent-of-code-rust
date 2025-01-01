//! # Disk Defragmentation
//!
//! This problem is a blend of the hashing from [`Day 10`] and the connected clique finding
//! from [`Day 12`] and reuses the same flood fill approach to count groups.
//!
//! [`Day 10`]: crate::year2017::day10
//! [`Day 12`]: crate::year2017::day12
use crate::util::thread::*;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Atomics can be safely shared between threads.
pub struct Shared {
    prefix: String,
    counter: AtomicUsize,
    mutex: Mutex<Exclusive>,
}

/// Regular data structures need to be protected by a mutex.
struct Exclusive {
    grid: Vec<u8>,
}

/// Parallelize the hashing as each row is independent.
pub fn parse(input: &str) -> Vec<u8> {
    let shared = Shared {
        prefix: input.trim().to_owned(),
        counter: AtomicUsize::new(0),
        mutex: Mutex::new(Exclusive { grid: vec![0; 0x4000] }),
    };

    // Use as many cores as possible to parallelize the hashing.
    spawn(|| worker(&shared));

    shared.mutex.into_inner().unwrap().grid
}

pub fn part1(input: &[u8]) -> u32 {
    input.iter().map(|&n| n as u32).sum()
}

pub fn part2(input: &[u8]) -> u32 {
    let mut grid: Vec<_> = input.iter().map(|&n| n == 1).collect();
    let mut groups = 0;

    for start in 0..0x4000 {
        // DFS from each new group.
        if grid[start] {
            groups += 1;
            dfs(&mut grid, start);
        }
    }

    groups
}

/// Each worker thread chooses the next available index then computes the hash and patches the
/// final vec with the result.
fn worker(shared: &Shared) {
    loop {
        let index = shared.counter.fetch_add(1, Ordering::Relaxed);
        if index >= 128 {
            break;
        }

        let row = fill_row(&shared.prefix, index);
        let start = index * 128;
        let end = start + 128;

        let mut exclusive = shared.mutex.lock().unwrap();
        exclusive.grid[start..end].copy_from_slice(&row);
    }
}

/// Compute the knot hash for a row and expand into an array of bytes.
fn fill_row(prefix: &str, index: usize) -> [u8; 128] {
    let s = format!("{prefix}-{index}");
    let mut lengths: Vec<_> = s.bytes().map(|b| b as usize).collect();
    lengths.extend([17, 31, 73, 47, 23]);

    let knot = knot_hash(&lengths);
    let mut result = [0; 128];

    for (i, chunk) in knot.chunks_exact(16).enumerate() {
        let reduced = chunk.iter().fold(0, |acc, n| acc ^ n);
        for j in 0..8 {
            result[8 * i + j] = (reduced >> (7 - j)) & 1;
        }
    }

    result
}

/// Slightly tweaked version of the code from Day 10 that always performs 64 rounds.
fn knot_hash(lengths: &[usize]) -> Vec<u8> {
    let mut knot: Vec<_> = (0..=255).collect();
    let mut position = 0;
    let mut skip = 0;

    for _ in 0..64 {
        for &length in lengths {
            let next = length + skip;
            knot[0..length].reverse();
            knot.rotate_left(next % 256);
            position += next;
            skip += 1;
        }
    }

    // Rotate the array the other direction so that the original starting position is restored.
    knot.rotate_right(position % 256);
    knot
}

/// Flood fill that explores the connected squares in the grid.
fn dfs(grid: &mut [bool], index: usize) {
    grid[index] = false;
    let x = index % 128;
    let y = index / 128;

    if x > 0 && grid[index - 1] {
        dfs(grid, index - 1);
    }
    if x < 127 && grid[index + 1] {
        dfs(grid, index + 1);
    }
    if y > 0 && grid[index - 128] {
        dfs(grid, index - 128);
    }
    if y < 127 && grid[index + 128] {
        dfs(grid, index + 128);
    }
}
