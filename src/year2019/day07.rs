//! # Amplification Circuit
//!
//! Brute force solution for both parts using the utility [`permutations`] method to test each of
//! the possible 5! or 120 permutations of the phase settings.
//!
//! Even though each `IntCode` computer runs in parallel in a separate thread the [`Sender`] and
//! [`Receiver`] objects synchronize messages between threads, blocking each thread when
//! input is needed.
//!
//! [`permutations`]: crate::util::slice
//! [`Sender`]: std::sync::mpsc::Sender
//! [`Receiver`]: std::sync::mpsc::Receiver
use super::day05::IntCode;
use crate::util::parse::*;
use crate::util::slice::*;

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed::<i64>().collect()
}

pub fn part1(input: &[i64]) -> i64 {
    let mut result = 0;

    let sequence = |slice: &[i64]| {
        let mut signal = 0;

        // Send exactly 2 inputs and receive exactly 1 output per amplifier.
        for &phase in slice {
            let (tx, rx) = IntCode::spawn(input);
            let _ = tx.send(phase);
            let _ = tx.send(signal);
            signal = rx.recv().unwrap();
        }

        result = result.max(signal);
    };

    [0, 1, 2, 3, 4].permutations(sequence);
    result
}

pub fn part2(input: &[i64]) -> i64 {
    let mut result = 0;

    let feedback = |slice: &[i64]| {
        let (senders, receivers): (Vec<_>, Vec<_>) = (0..5).map(|_| IntCode::spawn(input)).unzip();

        // Send each initial phase setting exactly once.
        for (tx, &phase) in senders.iter().zip(slice.iter()) {
            let _ = tx.send(phase);
        }

        // Chain amplifier inputs and ouputs in a loop until all threads finish.
        let mut signal = 0;

        'outer: loop {
            for (tx, rx) in senders.iter().zip(receivers.iter()) {
                let _ = tx.send(signal);
                let Ok(next) = rx.recv() else {
                    break 'outer;
                };
                signal = next;
            }
        }

        result = result.max(signal);
    };

    [5, 6, 7, 8, 9].permutations(feedback);
    result
}
