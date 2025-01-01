//! # Dueling Generators
//!
//! Multithreaded approach using worker threads to generate batches of numbers for judging.
//! Part one can be checked in parallel, but part two must be sent to a single thread as the
//! indices must be checked in order.
//!
//! The sequence of numbers are [modular exponentiation](https://en.wikipedia.org/wiki/Modular_exponentiation)
//! so we can jump to any location in the sequence, without needing to know the previous numbers.
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::math::*;
use crate::util::parse::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

const PART_ONE: usize = 40_000_000;
const PART_TWO: usize = 5_000_000;
const BLOCK: usize = 50_000;

type Input = (u32, u32);

/// State shared between all threads.
pub struct Shared {
    first: usize,
    second: usize,
    start: AtomicUsize,
    done: AtomicBool,
}

/// Generated numbers from `start` to `start + BLOCK`.
struct Block {
    start: usize,
    ones: u32,
    fours: Vec<u16>,
    eights: Vec<u16>,
}

pub fn parse(input: &str) -> Input {
    let [first, second] = input.iter_unsigned().chunk::<2>().next().unwrap();
    let shared = Shared { first, second, start: AtomicUsize::new(0), done: AtomicBool::new(false) };
    let (tx, rx) = channel();

    thread::scope(|scope| {
        // Use all cores except one to generate blocks of numbers for judging.
        for _ in 0..thread::available_parallelism().unwrap().get() - 1 {
            scope.spawn(|| sender(&shared, &tx));
        }
        // Judge batches serially.
        receiver(&shared, &rx)
    })
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}

fn sender(shared: &Shared, tx: &Sender<Block>) {
    while !shared.done.load(Ordering::Relaxed) {
        // Start at any point in the sequence using modular exponentiation.
        let start = shared.start.fetch_add(BLOCK, Ordering::Relaxed);
        let mut first = shared.first * 16807.mod_pow(start, 0x7fffffff);
        let mut second = shared.second * 48271.mod_pow(start, 0x7fffffff);

        // Estimate capacity at one quarter or one eight, plus a little extra for variance.
        let mut ones = 0;
        let mut fours = Vec::with_capacity((BLOCK * 30) / 100);
        let mut eights = Vec::with_capacity((BLOCK * 15) / 100);

        // Check part one pairs immediately while queueing part two pairs.
        for _ in 0..BLOCK {
            first = (first * 16807) % 0x7fffffff;
            second = (second * 48271) % 0x7fffffff;

            let left = first as u16;
            let right = second as u16;

            if left == right {
                ones += 1;
            }
            if left % 4 == 0 {
                fours.push(left);
            }
            if right % 8 == 0 {
                eights.push(right);
            }
        }

        let _unused = tx.send(Block { start, ones, fours, eights });
    }
}

fn receiver(shared: &Shared, rx: &Receiver<Block>) -> (u32, u32) {
    let mut remaining = PART_TWO;
    let mut part_two = 0;

    let mut required = 0;
    let mut out_of_order = FastMap::new();
    let mut blocks = Vec::new();

    let mut fours_block = 0;
    let mut fours_index = 0;

    let mut eights_block = 0;
    let mut eights_index = 0;

    while remaining > 0 {
        // Blocks could be received in any order, as there's no guarantee threads will finish
        // processing at the same time. The `start` field of the block defines the order they
        // must be added to the vec.
        while fours_block >= blocks.len() || eights_block >= blocks.len() {
            let block = rx.recv().unwrap();
            out_of_order.insert(block.start, block);

            while let Some(next) = out_of_order.remove(&required) {
                blocks.push(next);
                required += BLOCK;
            }
        }

        // Iterate over the minimum block size or numbers left to check.
        let fours = &blocks[fours_block].fours;
        let eights = &blocks[eights_block].eights;
        let iterations = remaining.min(fours.len() - fours_index).min(eights.len() - eights_index);

        remaining -= iterations;

        for _ in 0..iterations {
            if fours[fours_index] == eights[eights_index] {
                part_two += 1;
            }
            fours_index += 1;
            eights_index += 1;
        }

        // If we've checked all the numbers in a block, advance to the next one.
        // This may require waiting for a worker thread to create it first.
        if fours_index == fours.len() {
            fours_block += 1;
            fours_index = 0;
        }
        if eights_index == eights.len() {
            eights_block += 1;
            eights_index = 0;
        }
    }

    // Just in case, make sure we have enough blocks for part one.
    while required < PART_ONE {
        let block = rx.recv().unwrap();
        out_of_order.insert(block.start, block);

        while let Some(next) = out_of_order.remove(&required) {
            blocks.push(next);
            required += BLOCK;
        }
    }

    // Signal worker thread to finish.
    shared.done.store(true, Ordering::Relaxed);

    // Return results.
    let part_one = blocks.iter().take(PART_ONE / BLOCK).map(|p| p.ones).sum();
    (part_one, part_two)
}
