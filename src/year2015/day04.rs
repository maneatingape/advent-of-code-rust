//! # The Ideal Stocking Stuffer
//!
//! This solution relies on brute forcing combinations as quickly as possible using an internal
//! implementation of the [`MD5`] hashing algorithm.
//!
//! Each number's hash is independent of the others, so we speed things up by using threading
//! to search in parallel in blocks of 1000 numbers at a time.
//!
//! Using the [`format!`] macro to join the secret key to the number is quite slow. To go faster
//! we reuse the same `u8` buffer, incrementing digits one at a time.
//! The numbers from 1 to 999 are handled specially.
//!
//! Interestingly the total time to solve this problem is *extremely* sensitive to the secret key
//! provided as input. For example my key required ~10⁷ iterations to find the answer to part two.
//! However for unit testing, I was able to randomly find a value that takes only 455 iterations,
//! about 22,000 times faster!
//!
//! [`MD5`]: crate::util::md5
//! [`format!`]: std::format
use crate::util::md5::hash;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread;

type Input = (u32, u32);

enum Found {
    First(u32),
    Second(u32),
}

pub fn parse(input: &str) -> Input {
    let prefix = input.trim().to_string();
    let done = Arc::new(AtomicBool::new(false));
    let counter = Arc::new(AtomicU32::new(1000));
    let (tx, rx) = channel::<Found>();

    // Handle the first 999 numbers specially as the number of digits varies.
    for n in 1..1000 {
        let string = format!("{prefix}{n}");
        check_hash(string.as_bytes(), n, &tx);
    }

    // Use as many cores as possible to parallelize the search.
    for _ in 0..thread::available_parallelism().unwrap().get() {
        let prefix = prefix.clone();
        let done = done.clone();
        let counter = counter.clone();
        let tx = tx.clone();
        thread::spawn(move || worker(&prefix, &done, &counter, &tx));
    }

    // Explicitly drop the reference to the sender object so that when all search threads finish,
    // there will be no remaining references. When this happens `rx.recv` will return
    // `Error` and exit the loop below. This ensures we wait to receive results from all threads,
    // to handle the edge case where two values could be close together and found out of order.
    drop(tx);

    // We could potentially find multiple values, keep only the first occurence of each one.
    let mut first = u32::MAX;
    let mut second = u32::MAX;

    while let Ok(message) = rx.recv() {
        match message {
            Found::First(value) => {
                first = first.min(value);
            }
            Found::Second(value) => {
                second = second.min(value);
                done.store(true, Relaxed);
            }
        }
    }

    (first, second)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}

fn check_hash(buffer: &[u8], n: u32, tx: &Sender<Found>) {
    let (result, ..) = hash(buffer);

    if result & 0xffffff00 == 0 {
        let _ = tx.send(Found::Second(n));
    } else if result & 0xfffff000 == 0 {
        let _ = tx.send(Found::First(n));
    }
}

fn worker(prefix: &str, done: &Arc<AtomicBool>, counter: &Arc<AtomicU32>, tx: &Sender<Found>) {
    while !done.load(Relaxed) {
        let start = counter.fetch_add(1000, Relaxed);
        let string = format!("{prefix}{start}");
        let size = string.len() - 3;
        let mut buffer = string.as_bytes().to_vec();

        for n in 0..1000 {
            // Format macro is very slow, so update digits directly
            buffer[size] = b'0' + (n / 100) as u8;
            buffer[size + 1] = b'0' + ((n / 10) % 10) as u8;
            buffer[size + 2] = b'0' + (n % 10) as u8;
            check_hash(&buffer, start + n, tx);
        }
    }
}
