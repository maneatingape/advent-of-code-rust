//! # The Ideal Stocking Stuffer
//!
//! This solution relies on brute forcing combinations as quickly as possible using our own internal
//! implementation of the [`MD5`] hashing algorithm.
//!
//! Using the [`write!`] macro to join the secret key to the number is quite slow. To speed things
//! up we reuse the same `u8` buffer, incrementing digits one at a time. If a carry occurs we
//! propagate from right to left. Hitting the start of the secret key means that we have
//! transitioned to a new power of ten, for example from 9 to 10 or 99 to 100, so we increase the
//! size of the buffer by one.
//!
//! Interestingly the total time to solve this problem is *extremely* sensitive to the secret key
//! provided as input. For example my key required ~10⁷ iterations to find the answer to part two.
//! However for unit testing, I was able to randomly find a value that takes only 455 iterations,
//! about 22,000 times faster!
//!
//! [`MD5`]: crate::util::md5
//! [`write!`]: std::write
use crate::util::md5::hash;

pub fn parse(input: &str) -> &[u8] {
    input.trim().as_bytes()
}

pub fn part1(input: &[u8]) -> u32 {
    find(input, 0xfffff000)
}

pub fn part2(input: &[u8]) -> u32 {
    find(input, 0xffffff00)
}

fn find(input: &[u8], mask: u32) -> u32 {
    let mut count = 0;
    let mut size = input.len();
    let mut buffer = [b'0'; 32];

    buffer[..size].copy_from_slice(input);

    loop {
        count += 1;

        let mut index = size;
        loop {
            if buffer[index] < b'9' {
                buffer[index] += 1;
                break;
            } else if index == input.len() {
                buffer[index] = b'1';
                size += 1;
                break;
            } else {
                buffer[index] = b'0';
                index -= 1;
            }
        }

        if hash(&buffer[..(size + 1)]).0 & mask == 0 {
            return count;
        }
    }
}
