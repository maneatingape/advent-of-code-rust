//! # One-Time Pad
//!
//! Brute force slog through all possible keys, parallelized as much as possible. An optimization
//! for part two is a quick method to convert `u32` to 8 ASCII digits.
use crate::util::md5::*;
use crate::util::thread::*;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

/// Atomics can be safely shared between threads.
struct Shared<'a> {
    input: &'a str,
    part_two: bool,
    done: AtomicBool,
    counter: AtomicI32,
    mutex: Mutex<Exclusive>,
}

/// Regular data structures need to be protected by a mutex.
struct Exclusive {
    threes: BTreeMap<i32, u32>,
    fives: BTreeMap<i32, u32>,
    found: BTreeSet<i32>,
}

pub fn parse(input: &str) -> &str {
    input.trim()
}

/// Hash each key once.
pub fn part1(input: &str) -> i32 {
    generate_pad(input, false)
}

/// Hash each key an additional 2016 times.
pub fn part2(input: &str) -> i32 {
    generate_pad(input, true)
}

/// Find the first 64 keys that sastify the rules.
fn generate_pad(input: &str, part_two: bool) -> i32 {
    let exclusive =
        Exclusive { threes: BTreeMap::new(), fives: BTreeMap::new(), found: BTreeSet::new() };
    let shared = Shared {
        input,
        part_two,
        done: AtomicBool::new(false),
        counter: AtomicI32::new(0),
        mutex: Mutex::new(exclusive),
    };

    // Use as many cores as possible to parallelize the search.
    spawn(|| worker(&shared));

    let exclusive = shared.mutex.into_inner().unwrap();
    *exclusive.found.iter().nth(63).unwrap()
}

#[cfg(not(feature = "simd"))]
fn worker(shared: &Shared<'_>) {
    while !shared.done.load(Ordering::Relaxed) {
        // Get the next key to check.
        let n = shared.counter.fetch_add(1, Ordering::Relaxed);

        // Calculate the hash.
        let (mut buffer, size) = format_string(shared.input, n);
        let mut result = hash(&mut buffer, size);

        if shared.part_two {
            for _ in 0..2016 {
                buffer[0..8].copy_from_slice(&to_ascii(result.0));
                buffer[8..16].copy_from_slice(&to_ascii(result.1));
                buffer[16..24].copy_from_slice(&to_ascii(result.2));
                buffer[24..32].copy_from_slice(&to_ascii(result.3));
                result = hash(&mut buffer, 32);
            }
        }

        check(shared, n, result);
    }
}

/// Use SIMD to compute hashes in parallel in blocks of 32.
#[cfg(feature = "simd")]
#[expect(clippy::needless_range_loop)]
fn worker(shared: &Shared<'_>) {
    let mut result = ([0; 32], [0; 32], [0; 32], [0; 32]);
    let mut buffers = [[0; 64]; 32];

    while !shared.done.load(Ordering::Relaxed) {
        // Get the next key to check.
        let start = shared.counter.fetch_add(32, Ordering::Relaxed);

        // Calculate the hash.
        for i in 0..32 {
            let (mut buffer, size) = format_string(shared.input, start + i as i32);
            let (a, b, c, d) = hash(&mut buffer, size);

            result.0[i] = a;
            result.1[i] = b;
            result.2[i] = c;
            result.3[i] = d;
        }

        if shared.part_two {
            for _ in 0..2016 {
                for i in 0..32 {
                    buffers[i][0..8].copy_from_slice(&to_ascii(result.0[i]));
                    buffers[i][8..16].copy_from_slice(&to_ascii(result.1[i]));
                    buffers[i][16..24].copy_from_slice(&to_ascii(result.2[i]));
                    buffers[i][24..32].copy_from_slice(&to_ascii(result.3[i]));
                }
                result = simd::hash::<32>(&mut buffers, 32);
            }
        }

        for i in 0..32 {
            let hash = (result.0[i], result.1[i], result.2[i], result.3[i]);
            check(shared, start + i as i32, hash);
        }
    }
}

/// Check for sequences of 3 or 5 consecutive matching digits.
fn check(shared: &Shared<'_>, n: i32, hash: (u32, u32, u32, u32)) {
    let (a, b, c, d) = hash;

    let mut prev = u32::MAX;
    let mut same = 1;
    let mut three = 0;
    let mut five = 0;

    for mut word in [d, c, b, a] {
        for _ in 0..8 {
            let next = word & 0xf;

            if next == prev {
                same += 1;
            } else {
                same = 1;
            }

            if same == 3 {
                three = 1 << next;
            }
            if same == 5 {
                five |= 1 << next;
            }

            word >>= 4;
            prev = next;
        }
    }

    if three != 0 || five != 0 {
        let mut exclusive = shared.mutex.lock().unwrap();
        let mut candidates = Vec::new();

        // Compare against all 5 digit sequences.
        if three != 0 {
            exclusive.threes.insert(n, three);

            for (_, mask) in exclusive.fives.range(n + 1..n + 1001) {
                if three & mask != 0 {
                    candidates.push(n);
                }
            }
        }

        // Compare against all 3 digit sequences.
        if five != 0 {
            exclusive.fives.insert(n, five);

            for (&index, &mask) in exclusive.threes.range(n - 1000..n) {
                if five & mask != 0 {
                    candidates.push(index);
                }
            }
        }

        // Add any matching keys found, finishing once we have at least 64 keys.
        exclusive.found.extend(candidates);

        if exclusive.found.len() >= 64 {
            shared.done.store(true, Ordering::Relaxed);
        }
    }
}

/// Write the salt and integer index as ASCII characters.
fn format_string(prefix: &str, n: i32) -> ([u8; 64], usize) {
    let string = format!("{prefix}{n}");
    let size = string.len();

    let mut buffer = [0; 64];
    buffer[0..size].copy_from_slice(string.as_bytes());

    (buffer, size)
}

/// Quickly convert a `u32` to an array of 8 ASCII values.
fn to_ascii(n: u32) -> [u8; 8] {
    // Spread each nibble into its own byte, for example `1234abcd` becomes `010203040a0b0c0d`.
    let mut n = n as u64;
    n = ((n << 16) & 0x0000ffff00000000) | (n & 0x000000000000ffff);
    n = ((n << 8) & 0x00ff000000ff0000) | (n & 0x000000ff000000ff);
    n = ((n << 4) & 0x0f000f000f000f00) | (n & 0x000f000f000f000f);

    // If a digit is 0 to 9 then we need to add `0x30` to convert to an ASCII digit.
    // For digits from 10 to 15 we need to further add `0x27` to convert to lowercase ASCII.
    // Steps:
    // * Add 6 to each digit
    // * If digit is 10 or higher then the highest bit in each nibble will be set
    // * Shift this bit to create a mask
    // * Multiply mask by 0x27 to get ASCII conversion offset
    // For example mask of `010203040a0b0c0d` is `0000000001010101`.

    let mask = ((n + 0x0606060606060606) >> 4) & 0x0101010101010101;
    n = n + 0x3030303030303030 + 0x27 * mask;
    n.to_be_bytes()
}
