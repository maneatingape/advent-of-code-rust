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
use crate::util::md5::*;
use crate::util::thread::*;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Shared {
    prefix: String,
    iter: AtomicIter,
    first: AtomicU32,
    second: AtomicU32,
}

pub fn parse(input: &str) -> Shared {
    let shared = Shared {
        prefix: input.trim().to_owned(),
        iter: AtomicIter::new(1000, 1000),
        first: AtomicU32::new(u32::MAX),
        second: AtomicU32::new(u32::MAX),
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

    shared
}

pub fn part1(input: &Shared) -> u32 {
    input.first.load(Ordering::Relaxed)
}

pub fn part2(input: &Shared) -> u32 {
    input.second.load(Ordering::Relaxed)
}

fn format_string(prefix: &str, n: u32) -> ([u8; 64], usize) {
    let string = format!("{prefix}{n}");
    let size = string.len();

    let mut buffer = [0; 64];
    buffer[..size].copy_from_slice(string.as_bytes());

    (buffer, size)
}

fn check_hash(buffer: &mut [u8], size: usize, n: u32, shared: &Shared) {
    let [result, ..] = hash(buffer, size);

    if result & 0xffffff00 == 0 {
        shared.second.fetch_min(n, Ordering::Relaxed);
        shared.iter.stop();
    } else if result & 0xfffff000 == 0 {
        shared.first.fetch_min(n, Ordering::Relaxed);
    }
}

#[cfg(not(feature = "simd"))]
fn worker(shared: &Shared) {
    while let Some(offset) = shared.iter.next() {
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
    use crate::util::bitset::*;
    use crate::util::md5::simd::hash_fixed;
    use std::simd::cmp::SimdPartialEq as _;
    use std::simd::*;

    #[expect(clippy::needless_range_loop)]
    fn check_hash_simd<const N: usize>(
        buffers: &mut [[u8; 64]; N],
        size: usize,
        start: u32,
        offset: u32,
        shared: &Shared,
    ) {
        // Format macro is very slow, so update digits directly
        for i in 0..N {
            let n = offset + i as u32;
            buffers[i][size - 3] = b'0' + (n / 100) as u8;
            buffers[i][size - 2] = b'0' + ((n / 10) % 10) as u8;
            buffers[i][size - 1] = b'0' + (n % 10) as u8;
        }

        let [result, ..] = hash_fixed(buffers, size);
        let bitmask = (result & Simd::splat(0xfffff000)).simd_eq(Simd::splat(0)).to_bitmask();

        for i in bitmask.biterator() {
            if result[i] & 0xffffff00 == 0 {
                shared.second.fetch_min(start + offset + i as u32, Ordering::Relaxed);
                shared.iter.stop();
            } else {
                shared.first.fetch_min(start + offset + i as u32, Ordering::Relaxed);
            }
        }
    }

    pub(super) fn worker(shared: &Shared) {
        while let Some(start) = shared.iter.next() {
            let (prefix, size) = format_string(&shared.prefix, start);
            let buffers = &mut [prefix; 32];

            for offset in (0..992).step_by(32) {
                check_hash_simd(buffers, size, start, offset, shared);
            }

            let buffers = &mut [prefix; 8];
            check_hash_simd(buffers, size, start, 992, shared);
        }
    }
}
