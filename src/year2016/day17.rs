//! # Two Steps Forward
//!
//! Brute force search over every possible path. As we need all paths (not just the shortest)
//! a DFS is faster than a BFS as we can reuse the same buffer to store the path so far
//! only adding or removing the last step.
use crate::util::md5::*;

pub struct State {
    path: Vec<u8>,
    prefix: usize,
    size: usize,
    min: String,
    max: usize,
}

pub fn parse(input: &str) -> State {
    let bytes = input.trim().as_bytes();
    let prefix = bytes.len();
    let size = bytes.len();
    let min = String::new();
    let max = 0;

    let mut path = vec![0; 1024];
    path[..size].copy_from_slice(bytes);

    let mut state = State { path, prefix, size, min, max };
    explore(&mut state, 0, 0);
    state
}

pub fn part1(input: &State) -> String {
    input.min.to_string()
}

pub fn part2(input: &State) -> usize {
    input.max
}

fn explore(state: &mut State, x: u32, y: u32) {
    // If we've reached the end then don't go any further.
    if x == 3 && y == 3 {
        let adjusted = state.size - state.prefix;

        if state.min.is_empty() || adjusted < state.min.len() {
            let steps = state.path[state.prefix..state.size].to_vec();
            state.min = String::from_utf8(steps).unwrap();
        }
        state.max = state.max.max(adjusted);

        return;
    }

    // Round size up to next multiple of 64 bytes for md5 algorithm.
    let current = state.size;
    let padded = buffer_size(current);
    let (result, ..) = hash(&mut state.path[..padded], current);

    // Remove MD5 padding.
    state.path[padded - 8] = 0;
    state.path[padded - 7] = 0;
    state.path[padded - 6] = 0;
    state.path[padded - 5] = 0;
    state.path[padded - 4] = 0;
    state.path[padded - 3] = 0;
    state.path[padded - 2] = 0;
    state.path[padded - 1] = 0;

    state.size += 1;

    if y > 0 && ((result >> 28) & 0xf) > 0xa {
        state.path[current] = b'U';
        explore(state, x, y - 1);
    }
    if y < 3 && ((result >> 24) & 0xf) > 0xa {
        state.path[current] = b'D';
        explore(state, x, y + 1);
    }
    if x > 0 && ((result >> 20) & 0xf) > 0xa {
        state.path[current] = b'L';
        explore(state, x - 1, y);
    }
    if x < 3 && ((result >> 16) & 0xf) > 0xa {
        state.path[current] = b'R';
        explore(state, x + 1, y);
    }

    state.size = current;
    state.path[current] = 0;
}
