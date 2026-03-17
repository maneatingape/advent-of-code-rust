//! # Dueling Generators
//!
//! Multithreaded approach using worker threads to generate batches of numbers for judging.
//! Part one can be checked in parallel, but part two must be sent to a single thread as the
//! indices must be checked in order.
//!
//! The sequence of numbers are [modular exponentiation](https://en.wikipedia.org/wiki/Modular_exponentiation)
//! so we can jump to any location in the sequence, without needing to know the previous numbers.
//!
//! The generator is in the hot path, so anything we can do to make it run faster is worthwhile;
//! start by observing that our divisor 0x7fffffff is of the form `2ᵏ - 1`, which lends itself
//! well to computing a remainder with less work than a hardware division (the analysis here works
//! for any number adjacent to a power of two, not just Mersenne primes). At a high level,
//! computing `X % Y` is the same as repeatedly subtracting `Y` from a starting point of `X` until
//! reaching a value less than `Y`. How many times does that subtraction occur? That's easy,
//! `X / Y`. But when dividing by `Y` is expensive (a hardware division by an odd number takes
//! multiple clock cycles), what if we divide by `Y + 1` instead (dividing by 2ᵏ is just
//! performing a bit mask). Conceptually, the remainder after each subtraction of `Y + 1`
//! grows by an error of one until we reach a remainder of `X % (Y + 1)` - but we know the total
//! error: it was the number of times we subtracted the denominator, or `X / (Y + 1)`; and
//! that value is also available, with just a bit shift. Adding the 31-bit adjusted remainder
//! with the 31-bit error can overflow to 32 bits; then a final comparison against `MOD` gets
//! the correct answer in `fast_mod` faster than hardware division. See also
//! [this post](https://www.reddit.com/r/adventofcode/comments/7jxkiw/comment/drazokj/).
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::math::*;
use crate::util::parse::*;
use crate::util::thread::*;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

const MOD: usize = 0x7fffffff;
const PART_ONE: usize = 40_000_000;
const PART_TWO: usize = 5_000_000;
const BLOCK: usize = 50_000;

type Input = (usize, usize);

/// State shared between all threads.
pub struct Shared {
    first: usize,
    second: usize,
    iter: AtomicIter,
}

/// Generated numbers from `start` to `start + BLOCK`.
struct Block {
    start: usize,
    ones: usize,
    fours: Vec<u16>,
    eights: Vec<u16>,
}

pub fn parse(input: &str) -> Input {
    let [first, second] = input.iter_unsigned().chunk::<2>().next().unwrap();
    let shared = Shared { first, second, iter: AtomicIter::new(0, BLOCK as u32) };
    let (tx, rx) = channel();

    thread::scope(|scope| {
        // Use all cores except one to generate blocks of numbers for judging.
        for _ in 0..threads() - 1 {
            scope.spawn(|| sender(&shared, &tx));
        }
        // Judge batches serially.
        receiver(&shared, &rx)
    })
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}

fn sender(shared: &Shared, tx: &Sender<Block>) {
    while let Some(start) = shared.iter.next() {
        // Start at any point in the sequence using modular exponentiation.
        let start = start as usize;
        let mut first = shared.first * 16807.mod_pow(start, MOD);
        let mut second = shared.second * 48271.mod_pow(start, MOD);

        // Estimate capacity at one quarter or one eighth.
        let mut ones = 0;
        let mut fours = Vec::with_capacity(BLOCK / 4);
        let mut eights = Vec::with_capacity(BLOCK / 8);

        // Check part one pairs immediately while queueing part two pairs.
        for _ in 0..BLOCK {
            first = fast_mod(first * 16807);
            second = fast_mod(second * 48271);

            let left = first as u16;
            let right = second as u16;

            if left == right {
                ones += 1;
            }
            if left.is_multiple_of(4) {
                fours.push(left);
            }
            if right.is_multiple_of(8) {
                eights.push(right);
            }
        }

        let _unused = tx.send(Block { start, ones, fours, eights });
    }
}

fn receiver(shared: &Shared, rx: &Receiver<Block>) -> Input {
    let mut required = 0;
    let mut out_of_order = FastMap::new();

    let mut fours = Vec::with_capacity(PART_TWO + BLOCK);
    let mut eights = Vec::with_capacity(PART_TWO + BLOCK);
    let mut start = 0;

    let mut part_one = 0;
    let mut part_two = 0;

    while required < PART_ONE || fours.len() < PART_TWO || eights.len() < PART_TWO {
        // Blocks could be received in any order, as there's no guarantee threads will finish
        // processing at the same time. The `start` field of the block defines the order they
        // must be added to the vec.
        while let Ok(block) = rx.try_recv() {
            out_of_order.insert(block.start, block);
        }

        while let Some(block) = out_of_order.remove(&required) {
            required += BLOCK;

            if required <= PART_ONE {
                part_one += block.ones;
            }

            if fours.len() < PART_TWO {
                fours.extend_from_slice(&block.fours);
            }

            if eights.len() < PART_TWO {
                eights.extend_from_slice(&block.eights);
            }

            let end = PART_TWO.min(fours.len()).min(eights.len());
            part_two +=
                fours[start..end].iter().zip(&eights[start..end]).filter(|(a, b)| a == b).count();
            start = end;
        }
    }

    // Signal worker threads to finish.
    shared.iter.stop();

    (part_one, part_two)
}

/// Fast computation of n % 0x7fffffff.
#[inline]
fn fast_mod(n: usize) -> usize {
    let low = n & MOD;
    let high = n >> 31;
    let sum = low + high;
    if sum < MOD { sum } else { sum - MOD }
}
