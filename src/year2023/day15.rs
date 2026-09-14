//! # Lens Library
//!
//! Calculates part one and two at the same time as a speed optimization. Assumes labels are always
//! 8 characters or fewer.
use std::array::from_fn;

use crate::util::parse::*;

type Input = (usize, usize);

struct Item {
    label: usize,
    lens: usize,
}

pub fn parse(input: &str) -> Input {
    let input = input.trim().as_bytes();

    let mut part_one = 0;
    let mut part_two = 0;
    let mut hash = 0;
    let mut label = 0;
    let mut boxes: [Vec<Item>; 256] = from_fn(|_| Vec::new());

    for (i, &b) in input.iter().enumerate() {
        match b {
            b',' => {
                part_one += hash;
                hash = 0;
                label = 0;
                continue;
            }
            b'-' => boxes[hash].retain(|item| item.label != label),
            b'=' => {
                let lens = input[i + 1].to_decimal();
                let slot = &mut boxes[hash];

                match slot.iter_mut().find(|item| item.label == label) {
                    Some(item) => item.lens = lens,
                    None => slot.push(Item { label, lens }),
                }
            }
            _ => (),
        }

        let u = usize::from(b);
        hash = ((hash + u) * 17) & 0xff;
        label = (label << 8) | u;
    }

    for (i, next) in boxes.iter().enumerate() {
        for (j, item) in next.iter().enumerate() {
            part_two += (i + 1) * (j + 1) * item.lens;
        }
    }

    // The final step has no trailing comma.
    (part_one + hash, part_two)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}
