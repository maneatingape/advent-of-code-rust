#![allow(clippy::needless_range_loop)]

use crate::util::collection::*;

type Input = (usize, Vec<usize>);

pub fn parse(input: &str) -> Input {
    let bytes = input.as_bytes();
    let width = bytes.iter().position(|b| b.is_ascii_whitespace()).unwrap();
    let digits: Vec<usize> = bytes
        .iter()
        .filter(|b| b.is_ascii_digit())
        .map(|b| (b - 47) as usize)
        .collect();
    (width, digits)
}

pub fn part1(input: &Input) -> usize {
    let (width, digits) = input;
    let mut visible: Vec<bool> = Vec::fill(digits.len(), false);

    for i in 0..*width {
        let mut left_max = 0;
        let mut right_max = 0;
        let mut top_max = 0;
        let mut bottom_max = 0;

        for j in 0..*width {
            let left = (i * width) + j;
            if digits[left] > left_max {
                visible[left] = true;
                left_max = digits[left];
            }

            let right = (i * width) + (width - j - 1);
            if digits[right] > right_max {
                visible[right] = true;
                right_max = digits[right];
            }

            let top = (j * width) + i;
            if digits[top] > top_max {
                visible[top] = true;
                top_max = digits[top];
            }

            let bottom = (width - j - 1) * width + i;
            if digits[bottom] > bottom_max {
                visible[bottom] = true;
                bottom_max = digits[bottom];
            }
        }
    }

    visible.iter().filter(|&&b| b).count()
}

pub fn part2(input: &Input) -> usize {
    let (width, digits) = input;
    let mut scenic: Vec<usize> = Vec::fill(digits.len(), 1);

    for i in 1..(*width - 1) {
        let mut left_max = [0; 11];
        let mut right_max = [0; 11];
        let mut top_max = [0; 11];
        let mut bottom_max = [0; 11];

        for j in 1..(*width - 1) {
            let left = (i * width) + j;
            scenic[left] *= j - left_max[digits[left]];
            for k in 1..=digits[left] {
                left_max[k] = j;
            }

            let right = (i * width) + (width - j - 1);
            scenic[right] *= j - right_max[digits[right]];
            for k in 1..=digits[right] {
                right_max[k] = j;
            }

            let top = (j * width) + i;
            scenic[top] *= j - top_max[digits[top]];
            for k in 1..=digits[top] {
                top_max[k] = j;
            }

            let bottom = (width - j - 1) * width + i;
            scenic[bottom] *= j - bottom_max[digits[bottom]];
            for k in 1..=digits[bottom] {
                bottom_max[k] = j;
            }
        }
    }

    *scenic.iter().max().unwrap()
}
