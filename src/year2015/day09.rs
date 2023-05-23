use crate::util::iter::*;
use crate::util::hash::*;
use crate::util::parse::*;
use crate::util::slice::*;

type Result = (u32, u32);

pub fn parse(input: &str) -> Result {
    let tokens: Vec<_> = input
        .split_ascii_whitespace()
        .chunk::<5>()
        .collect();

    let mut indices = FastMapBuilder::empty();
    for [start, _, end, ..] in tokens.iter() {
        if !indices.contains_key(start) {
            indices.insert(start, indices.len());
        }
        if !indices.contains_key(end) {
            indices.insert(end, indices.len());
        }
    }

    let stride = indices.len();
    let mut distances = vec![0_u32; stride * stride];
    for [start, _, end, _, distance] in tokens.iter() {
        let start = indices[start];
        let end = indices[end];
        let distance = from(distance);
        distances[stride * start + end] = distance;
        distances[stride * end + start] = distance;
    }

    let mut indices: Vec<_> = (0..stride).collect();
    let mut min = u32::MAX;
    let mut max = u32::MIN;

    indices.as_mut_slice().permutations(|slice| {
        let result: u32 = slice
            .windows(2)
            .map(|w| distances[stride * w[0] + w[1]])
            .sum();
        min = min.min(result);
        max = max.max(result);
    });

    (min, max)
}

pub fn part1(input: &Result) -> u32 {
    input.0
}

pub fn part2(input: &Result) -> u32 {
    input.1
}
