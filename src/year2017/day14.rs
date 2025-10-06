//! # Disk Defragmentation
//!
//! This problem is a blend of the hashing from [`Day 10`] and the connected clique finding
//! from [`Day 12`] and reuses the same flood fill approach to count groups.
//!
//! [`Day 10`]: crate::year2017::day10
//! [`Day 12`]: crate::year2017::day12
use crate::util::thread::*;

/// Parallelize the hashing as each row is independent.
pub fn parse(input: &str) -> Vec<u8> {
    let prefix = &input.trim();
    let rows: Vec<_> = (0..128).collect();
    let result = spawn_parallel_iterator(&rows, |iter| worker(prefix, iter));

    let mut sorted: Vec<_> = result.into_iter().flatten().collect();
    sorted.sort_unstable_by_key(|&(index, _)| index);
    sorted.into_iter().flat_map(|(_, row)| row).collect()
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
fn worker(prefix: &str, iter: ParIter<'_, usize>) -> Vec<(usize, Vec<u8>)> {
    iter.map(|&index| (index, fill_row(prefix, index))).collect()
}

/// Compute the knot hash for a row and expand into an array of bytes.
fn fill_row(prefix: &str, index: usize) -> Vec<u8> {
    let s = format!("{prefix}-{index}");
    let mut lengths: Vec<_> = s.bytes().map(|b| b as usize).collect();
    lengths.extend([17, 31, 73, 47, 23]);

    let knot = knot_hash(&lengths);
    let mut result = vec![0; 128];

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
