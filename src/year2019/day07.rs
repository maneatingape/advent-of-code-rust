//! # Amplification Circuit
//!
//! Brute force solution for both parts using the utility [`permutations`] method to test each of
//! the possible 5! or 120 permutations of the phase settings.
use std::array::from_fn;

use crate::util::intcode::*;
use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed().collect()
}

pub fn part1(input: &[i64]) -> i64 {
    let mut result = 0;
    let mut computer = Computer::new(input);

    let mut sequence = |phases: [i64; 5]| {
        // Send exactly 2 inputs and receive exactly 1 output per amplifier.
        let signal = phases.iter().fold(0, |signal, &phase| {
            computer.reset();
            computer.input(phase);
            computer.input(signal);
            let State::Output(next) = computer.run() else { unreachable!() };
            next
        });

        result = result.max(signal);
    };

    permutations([0, 1, 2, 3, 4], 0, &mut sequence);
    result
}

pub fn part2(input: &[i64]) -> i64 {
    let mut result = 0;
    let mut computers: [_; 5] = from_fn(|_| Computer::new(input));

    let mut feedback = |phases: [i64; 5]| {
        // Reset state and send each initial phase setting exactly once.
        for (computer, phase) in computers.iter_mut().zip(phases) {
            computer.reset();
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

    permutations([5, 6, 7, 8, 9], 0, &mut feedback);
    result
}

/// Generates all possible permutations of the array, passing them to a callback function.
fn permutations(mut array: [i64; 5], i: usize, f: &mut impl FnMut([i64; 5])) {
    if i == 5 {
        f(array);
    } else {
        for j in i..5 {
            array.swap(i, j);
            permutations(array, i + 1, f);
            array.swap(i, j);
        }
    }
}
