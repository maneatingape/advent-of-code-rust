//! # How About a Nice Game of Chess?
//!
//! Essentially a repeat of [`Year 2015 Day 4`]. We brute force MD5 hashes as quickly as
//! possible in parallel in blocks of 1000 at a time.
//!
//! [`Year 2015 Day 4`]: crate::year2015::day04
use crate::util::md5::*;
use crate::util::thread::*;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

struct Shared {
    prefix: String,
    done: AtomicBool,
    counter: AtomicU32,
    mutex: Mutex<Exclusive>,
}

struct Exclusive {
    found: Vec<(u32, u32)>,
    mask: u16,
}

pub fn parse(input: &str) -> Vec<u32> {
    let shared = Shared {
        prefix: input.trim().to_owned(),
        done: AtomicBool::new(false),
        counter: AtomicU32::new(1000),
        mutex: Mutex::new(Exclusive { found: vec![], mask: 0 }),
    };

    // Handle the first 999 numbers specially as the number of digits varies.
    for n in 1..1000 {
        let (mut buffer, size) = format_string(&shared.prefix, n);
        check_hash(&mut buffer, size, n, &shared);
    }

    // Use as many cores as possible to parallelize the remaining search.
    spawn(|| {
        #[cfg(not(feature = "simd"))]
        worker(&shared);
        #[cfg(feature = "simd")]
        simd::worker(&shared);
    });

    let mut found = shared.mutex.into_inner().unwrap().found;
    found.sort_unstable();
    found.iter().map(|&(_, n)| n).collect()
}

pub fn part1(input: &[u32]) -> String {
    let password = input.iter().take(8).fold(0, |acc, n| (acc << 4) | (n >> 8));
    format!("{password:08x}")
}

pub fn part2(input: &[u32]) -> String {
    let mut password = 0;
    let mut mask = 0xffffffff;

    for n in input {
        let sixth = n >> 8;
        if sixth < 8 {
            let shift = 4 * (7 - sixth);
            let seventh = (n >> 4) & 0xf;
            password |= (seventh << shift) & mask;
            mask &= !(0xf << shift);
        }
    }

    format!("{password:08x}")
}

fn format_string(prefix: &str, n: u32) -> ([u8; 64], usize) {
    let string = format!("{prefix}{n}");
    let size = string.len();

    let mut buffer = [0; 64];
    buffer[0..size].copy_from_slice(string.as_bytes());

    (buffer, size)
}

fn check_hash(buffer: &mut [u8], size: usize, n: u32, shared: &Shared) {
    let (result, ..) = hash(buffer, size);

    if result & 0xfffff000 == 0 {
        let mut exclusive = shared.mutex.lock().unwrap();

        exclusive.found.push((n, result));
        exclusive.mask |= 1 << (result >> 8);

        if exclusive.mask & 0xff == 0xff {
            shared.done.store(true, Ordering::Relaxed);
        }
    }
}

#[cfg(not(feature = "simd"))]
fn worker(shared: &Shared) {
    while !shared.done.load(Ordering::Relaxed) {
        let offset = shared.counter.fetch_add(1000, Ordering::Relaxed);
        let (mut buffer, size) = format_string(&shared.prefix, offset);

        for n in 0..1000 {
            // Format macro is very slow, so update digits directly
            buffer[size - 3] = b'0' + (n / 100) as u8;
            buffer[size - 2] = b'0' + ((n / 10) % 10) as u8;
            buffer[size - 1] = b'0' + (n % 10) as u8;

            check_hash(&mut buffer, size, offset + n, shared);
        }
    }
}

#[cfg(feature = "simd")]
mod simd {
    use super::*;
    use crate::util::md5::simd::hash;
    use std::simd::{LaneCount, SupportedLaneCount};

    #[expect(clippy::needless_range_loop)]
    fn check_hash_simd<const N: usize>(
        buffers: &mut [[u8; 64]],
        size: usize,
        start: u32,
        offset: u32,
        shared: &Shared,
    ) where
        LaneCount<N>: SupportedLaneCount,
    {
        // Format macro is very slow, so update digits directly
        for i in 0..N {
            let n = offset + i as u32;
            buffers[i][size - 3] = b'0' + (n / 100) as u8;
            buffers[i][size - 2] = b'0' + ((n / 10) % 10) as u8;
            buffers[i][size - 1] = b'0' + (n % 10) as u8;
        }

        let (result, ..) = hash::<N>(buffers, size);

        for i in 0..N {
            if result[i] & 0xfffff000 == 0 {
                let mut exclusive = shared.mutex.lock().unwrap();

                exclusive.found.push((start + offset + i as u32, result[i]));
                exclusive.mask |= 1 << (result[i] >> 8);

                if exclusive.mask & 0xff == 0xff {
                    shared.done.store(true, Ordering::Relaxed);
                }
            }
        }
    }

    pub(super) fn worker(shared: &Shared) {
        while !shared.done.load(Ordering::Relaxed) {
            let start = shared.counter.fetch_add(1000, Ordering::Relaxed);
            let (prefix, size) = format_string(&shared.prefix, start);
            let mut buffers = [prefix; 32];

            for offset in (0..992).step_by(32) {
                check_hash_simd::<32>(&mut buffers, size, start, offset, shared);
            }

            check_hash_simd::<8>(&mut buffers, size, start, 992, shared);
        }
    }
}
