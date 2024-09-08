//! # Two Steps Forward
//!
//! Brute force search over every possible path. Work is parallelized over multiple threads.
//! Keeping each thread busy and spreading the work as evenly as possible is quite tricky. Some
//! paths can dead-end quickly while others can take the majority of exploration time.
//!
//! To solve this we implement a very simple version of
//! [work stealing](https://en.wikipedia.org/wiki/Work_stealing). Threads process paths locally,
//! stopping every now and then to return paths to a global queue. This allows other threads that
//! have run out of work to pickup new paths to process.
//!
//! The approach from "Waiting: Parking and Condition Variables" in the excellent book
//! [Rust Atomics and Locks](https://marabos.nl/atomics/) prevent idle threads from busy
//! looping on the mutex.
use crate::util::md5::*;
use crate::util::thread::*;
use std::sync::{Condvar, Mutex};

type Input = (String, usize);
type Item = (u8, u8, usize, Vec<u8>);

struct State {
    todo: Vec<Item>,
    min: String,
    max: usize,
}

struct Exclusive {
    global: State,
    inflight: usize,
}

struct Shared {
    prefix: usize,
    mutex: Mutex<Exclusive>,
    not_empty: Condvar,
}

pub fn parse(input: &str) -> Input {
    // Initial starting position is the top left corner.
    let input = input.trim().as_bytes();
    let prefix = input.len();
    let start = (0, 0, prefix, extend(input, prefix, 0));

    // State shared between threads.
    let global = State { todo: vec![start], min: String::new(), max: 0 };
    let exclusive = Exclusive { global, inflight: 0 };
    let shared = Shared { prefix, mutex: Mutex::new(exclusive), not_empty: Condvar::new() };

    // Search paths in parallel.
    spawn(|| worker(&shared));

    let global = shared.mutex.into_inner().unwrap().global;
    (global.min, global.max)
}

pub fn part1(input: &Input) -> &str {
    &input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

/// Process local work items, stopping every now and then to redistribute items back to global pool.
/// This prevents threads idling or hotspotting.
fn worker(shared: &Shared) {
    let mut local = State { todo: Vec::new(), min: String::new(), max: 0 };

    loop {
        let mut exclusive = shared.mutex.lock().unwrap();
        let item = loop {
            // Pickup available work.
            if let Some(item) = exclusive.global.todo.pop() {
                exclusive.inflight += 1;
                break item;
            }
            // If no work available and no other thread is doing anything, then we're done.
            if exclusive.inflight == 0 {
                return;
            }
            // Put thread to sleep until another thread notifies us that work is available.
            // This avoids busy looping on the mutex.
            exclusive = shared.not_empty.wait(exclusive).unwrap();
        };

        // Drop mutex to release lock and allow other threads access.
        drop(exclusive);

        // Process local work items.
        local.todo.push(item);
        explore(shared, &mut local);

        // Redistribute local work items back to the global queue. Update min and max paths.
        let mut exclusive = shared.mutex.lock().unwrap();
        let global = &mut exclusive.global;

        global.todo.append(&mut local.todo);
        if global.min.is_empty() || local.min.len() < global.min.len() {
            global.min = local.min.clone();
        }
        global.max = global.max.max(local.max);

        // Mark ourselves as idle then notify all other threads that there is new work available.
        exclusive.inflight -= 1;
        shared.not_empty.notify_all();
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
                let middle = path[shared.prefix..size].to_vec();
                local.min = String::from_utf8(middle).unwrap();
            }
            local.max = local.max.max(adjusted);
        } else {
            // Explore other paths.
            let (result, ..) = hash(&mut path, size);

            if y > 0 && ((result >> 28) & 0xf) > 0xa {
                local.todo.push((x, y - 1, size + 1, extend(&path, size, b'U')));
            }
            if y < 3 && ((result >> 24) & 0xf) > 0xa {
                local.todo.push((x, y + 1, size + 1, extend(&path, size, b'D')));
            }
            if x > 0 && ((result >> 20) & 0xf) > 0xa {
                local.todo.push((x - 1, y, size + 1, extend(&path, size, b'L')));
            }
            if x < 3 && ((result >> 16) & 0xf) > 0xa {
                local.todo.push((x + 1, y, size + 1, extend(&path, size, b'R')));
            }
        }
    }
}

/// Convenience function to generate new path.
fn extend(src: &[u8], size: usize, b: u8) -> Vec<u8> {
    // Leave room for MD5 padding.
    let padded = buffer_size(size + 1);
    let mut next = vec![0; padded];
    // Copy existing path and next step.
    next[0..size].copy_from_slice(&src[0..size]);
    next[size] = b;
    next
}
