//! # Printing Department
//!
//! To speed things up, we build a queue of rolls to be removed. For part one, the length of the
//! initial list is the answer. For part two, as we process rolls from the list one at a time, we
//! update neighboring rolls. If any neighbor drops below the threshold, then we add it to the
//! list. This approach avoids a time-consuming scan of the entire grid to find new rolls to remove.
//!
//! Building the initial list of counts, then processing the list is written to minimize branches.
//! This allows the compiler to auto-vectorize the loops, resulting in a 3.5x speedup.
use crate::util::grid::*;

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    // Newlines form an empty edge around the grid to avoid boundary checks.
    let Grid { width, bytes, .. } = Grid::parse_with_border(input);
    let width = width as usize;

    // Diagonal neighbors relative to the center offset.
    let center = width + 1;
    let offsets = [0, 1, 2, width, width + 2, 2 * width, 2 * width + 1, 2 * width + 2];

    // Count neighbors one offset at a time over the whole grid, so that each pass auto-vectorizes.
    let mut counts = vec![0; bytes.len()];

    for offset in offsets {
        for (count, &b) in counts[center..].iter_mut().zip(&bytes[offset..]) {
            *count += u8::from(b == b'@');
        }
    }

    // Set empty cells to a value high enough that they will never drop below the threshold.
    for (count, &b) in counts.iter_mut().zip(&bytes) {
        *count = if b == b'@' { *count } else { u8::MAX };
    }

    // Add rolls that can be removed to the initial list. The list has room for every cell, so
    // pushes always write the next slot then only advance the tail when needed. This avoids
    // branches.
    let mut todo = vec![0; bytes.len()];
    let mut head = 0;
    let mut tail = 0;

    for (index, &count) in counts.iter().enumerate() {
        todo[tail] = index;
        tail += usize::from(count < 4);
    }

    let part_one = tail;

    // Update neighbors as rolls are removed. If they drop below the threshold, then add to the
    // list. Each cell is processed at most once.
    while head < tail {
        let base = todo[head] - center;
        head += 1;

        for next in offsets.map(|offset| base + offset) {
            let count = counts[next];
            todo[tail] = next;
            tail += usize::from(count == 4);
            counts[next] -= 1;
        }
    }

    (part_one, tail)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}
