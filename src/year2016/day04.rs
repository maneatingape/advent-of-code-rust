//! # Security Through Obscurity
//!
//! We can quickly eliminate decoys without needing an expensive sort by checking that the
//! frequency of checksum letters is non-increasing, letters of equal frequency are in alphabetical
//! order and that there are no intervening letters in between any two frequencies.
//!
//! In part two as the [Caesar cipher](https://en.wikipedia.org/wiki/Caesar_cipher) does
//! not change the length of words, we can also eliminate most candidates with a simple length
//! check and only decrypt a much smaller number of strings.
use crate::util::parse::*;

pub struct Room<'a> {
    name: &'a str,
    sector_id: u32,
}

pub fn parse(input: &str) -> Vec<Room<'_>> {
    let mut valid = Vec::new();

    for line in input.lines() {
        // The sector id and checksum are fixed size leaving whatever is left over as the name.
        let size = line.len();
        let name = &line[..size - 11];
        let checksum = &line.as_bytes()[size - 6..size - 1];

        // Count the frequency of each digit, the frequency of each frequency `fof` and the
        // highest total frequency.
        let mut freq = [0; 26];
        let mut fof = [0; 26];
        let mut highest = 0;

        for b in name.bytes() {
            if b != b'-' {
                let index = to_index(b);
                let current = freq[index];
                let next = freq[index] + 1;

                freq[index] = next;
                fof[current] -= 1;
                fof[next] += 1;

                highest = highest.max(next);
            }
        }

        // Filter real rooms vs decoys.
        if freq[to_index(checksum[0])] == highest && rules(checksum, &freq, &fof) {
            let sector_id = (&line[size - 10..size - 7]).unsigned();
            valid.push(Room { name, sector_id });
        }
    }

    valid
}

pub fn part1(input: &[Room<'_>]) -> u32 {
    input.iter().map(|room| room.sector_id).sum()
}

pub fn part2(input: &[Room<'_>]) -> u32 {
    for &Room { name, sector_id } in input {
        let bytes = name.as_bytes();

        // Quickly eliminate most rooms as the length of words doesn't change.
        if bytes.len() == 24 && bytes[9] == b'-' && bytes[16] == b'-' {
            let mut buffer = String::with_capacity(24);

            // Decrypt potential candidates.
            for b in name.bytes() {
                if b == b'-' {
                    buffer.push(' ');
                } else {
                    let rotate = (sector_id % 26) as u8;
                    let decrypted = (b - b'a' + rotate) % 26 + b'a';
                    buffer.push(decrypted as char);
                }
            }

            if buffer == "northpole object storage" {
                return sector_id;
            }
        }
    }

    unreachable!()
}

/// Check each pair making sure that the frequency is non-increasing and that there are
/// no letters in between (`fof` should be zero for all intervening frequencies).
/// If the frequency is equal then also make sure letters are in alphabetical order.
fn rules(checksum: &[u8], freq: &[usize], fof: &[i32]) -> bool {
    checksum.windows(2).all(|w| {
        let end = freq[to_index(w[0])];
        let start = freq[to_index(w[1])];
        !(start > end || (start == end && w[1] <= w[0]) || (start + 1..end).any(|i| fof[i] != 0))
    })
}

fn to_index(b: u8) -> usize {
    (b - b'a') as usize
}
