//! # Amplification Circuit
//!
//! Brute force solution for both parts using the utility [`permutations`] method to test each of
//! the possible 5! or 120 permutations of the phase settings.
use super::intcode::*;
use crate::util::parse::*;
use std::array::from_fn;

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed::<i64>().collect()
}

pub fn part1(input: &[i64]) -> i64 {
    let mut result = 0;
    let mut computer = Computer::new(input);

    let sequence = |slice: &[i64]| {
        let mut signal = 0;

        // Send exactly 2 inputs and receive exactly 1 output per amplifier.
        for &phase in slice {
            computer.reset();
            computer.input(phase);
            computer.input(signal);
            let State::Output(next) = computer.run() else { unreachable!() };
            signal = next;
        }

        result = result.max(signal);
    };

    permutations(&mut [0, 1, 2, 3, 4], sequence);
    result
}

pub fn part2(input: &[i64]) -> i64 {
    let mut result = 0;
    let mut computers: [Computer; 5] = from_fn(|_| Computer::new(input));

    let feedback = |slice: &[i64]| {
        // Reset state.
        computers.iter_mut().for_each(Computer::reset);

        // Send each initial phase setting exactly once.
        for (computer, &phase) in computers.iter_mut().zip(slice) {
            computer.input(phase);
        }

        // Chain amplifier inputs and outputs in a loop until all threads finish.
        let mut signal = 0;

        'outer: loop {
            for computer in &mut computers {
                computer.input(signal);
                let State::Output(next) = computer.run() else { break 'outer };
                signal = next;
            }
        }

        result = result.max(signal);
    };

    permutations(&mut [5, 6, 7, 8, 9], feedback);
    result
}

/// Generates all possible permutations of a mutable slice, passing them one at a time to a
/// callback function.
/// Uses [Heap's algorithm](https://en.wikipedia.org/wiki/Heap%27s_algorithm) for efficiency,
/// modifying the slice in place.
fn permutations(slice: &mut [i64], mut callback: impl FnMut(&[i64])) {
    callback(slice);

    let n = slice.len();
    let mut c = vec![0; n];
    let mut i = 1;

    while i < n {
        if c[i] < i {
            let swap_index = if i.is_multiple_of(2) { 0 } else { c[i] };
            slice.swap(swap_index, i);
            callback(slice);
            c[i] += 1;
            i = 1;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
}
