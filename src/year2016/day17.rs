//! # Two Steps Forward
//!
//! Brute force search over every possible path. Work is parallelized over multiple threads.
//! Keeping each thread busy and spreading the work as evenly as possible is quite tricky. Some
//! paths can dead-end quickly while others can take the majority of exploration time.
//!
//! To solve this we implement a very simple version of work sharing. Threads process paths locally
//! stopping every now and then to return paths to a global queue. This allows other threads that
//! have run out of work to pick up new paths to process.
//!
//! The approach from "Waiting: Parking and Condition Variables" in the excellent book
//! [Rust Atomics and Locks](https://marabos.nl/atomics/) prevent idle threads from busy
//! looping on the mutex.
use crate::util::md5::*;
use crate::util::thread::*;
use std::sync::{Condvar, Mutex};

type Input = (Vec<u8>, usize);
type Item = (u8, u8, usize, Vec<u8>);

struct State {
    todo: Vec<Item>,
    min: Vec<u8>,
    max: usize,
    inflight: usize,
}

struct Shared {
    prefix: usize,
    mutex: Mutex<State>,
    not_empty: Condvar,
}

pub fn parse(input: &str) -> Input {
    // Initial starting position is the top left corner.
    let input = input.trim().as_bytes();
    let prefix = input.len();
    let start = (0, 0, prefix, extend(input, prefix, 0));

    // State shared between threads.
    let state = State { todo: vec![start], min: vec![], max: 0, inflight: threads() };
    let shared = Shared { prefix, mutex: Mutex::new(state), not_empty: Condvar::new() };

    // Search paths in parallel.
    spawn(|| worker(&shared));

    let state = shared.mutex.into_inner().unwrap();
    (state.min, state.max)
}

pub fn part1(input: &Input) -> &str {
    str::from_utf8(&input.0).unwrap()
}

pub fn part2(input: &Input) -> usize {
    input.1
}

/// Process local work items, stopping every now and then to redistribute items back to global pool.
/// This prevents threads idling or hotspotting.
fn worker(shared: &Shared) {
    let mut local = State { todo: vec![], min: vec![], max: 0, inflight: 0 };

    loop {
        // Process local work items.
        explore(shared, &mut local);

        // Acquire mutex.
        let mut state = shared.mutex.lock().unwrap();

        // Update min and max paths.
        if !local.min.is_empty() && (state.min.is_empty() || local.min.len() < state.min.len()) {
            state.min.clone_from(&local.min);
        }
        state.max = state.max.max(local.max);

        if local.todo.is_empty() {
            // Mark ourselves as idle then notify all other threads in case we're done.
            state.inflight -= 1;
            shared.not_empty.notify_all();

            loop {
                // Pickup available work.
                if let Some(item) = state.todo.pop() {
                    state.inflight += 1;
                    local.todo.push(item);
                    break;
                }
                // If no work available and no other thread is doing anything, then we're done.
                if state.inflight == 0 {
                    return;
                }
                // Put thread to sleep until another thread notifies us that work is available.
                // This avoids busy looping on the mutex.
                state = shared.not_empty.wait(state).unwrap();
            }
        } else {
            // Redistribute excess local work items back to the global queue then notify all other
            // threads that there is new work available.
            state.todo.extend(local.todo.drain(1..));
            shared.not_empty.notify_all();
        }
    }
}

/// Explore at most 100 paths, stopping sooner if we run out.
/// 100 is chosen empirically as the amount that results in the least total time taken.
///
/// Too low and threads waste time locking the mutex, reading and writing global state.
/// Too high and some threads are starved with no paths, while other threads do all the work.
fn explore(shared: &Shared, local: &mut State) {
    for _ in 0..100 {
        let Some((x, y, size, mut path)) = local.todo.pop() else { break };

        if x == 3 && y == 3 {
            // Stop if we've reached the bottom right room.
            let adjusted = size - shared.prefix;
            if local.min.is_empty() || adjusted < local.min.len() {
                // Remove salt and padding.
                local.min = path[shared.prefix..size].to_vec();
            }
            local.max = local.max.max(adjusted);
        } else {
            // Explore other paths.
            let [result, ..] = hash(&mut path, size);

            if y > 0 && is_open(result, 28) {
                local.todo.push((x, y - 1, size + 1, extend(&path, size, b'U')));
            }
            if y < 3 && is_open(result, 24) {
                local.todo.push((x, y + 1, size + 1, extend(&path, size, b'D')));
            }
            if x > 0 && is_open(result, 20) {
                local.todo.push((x - 1, y, size + 1, extend(&path, size, b'L')));
            }
            if x < 3 && is_open(result, 16) {
                local.todo.push((x + 1, y, size + 1, extend(&path, size, b'R')));
            }
        }
    }
}

/// Check if a door is open based on MD5 hex digit (b-f means open).
#[inline]
fn is_open(hash: u32, shift: u32) -> bool {
    ((hash >> shift) & 0xf) > 0xa
}

/// Convenience function to generate new path.
fn extend(src: &[u8], size: usize, b: u8) -> Vec<u8> {
    // Leave room for MD5 padding.
    let mut next = vec![0; buffer_size(size + 1)];
    // Copy existing path and next step.
    next[..size].copy_from_slice(&src[..size]);
    next[size] = b;
    next
}
