//! # Knights of the Dinner Table
//!
//! This problem is very similar to [`Day 9`] and we solve it in almost exactly the same way by
//! computing an adjacency matrix of happiness then permuting the order of the diners.
//!
//! For part one we reduce the permutations from 8! = 40,320 permutations to 7! = 5,040
//! permutations by arbitrarily choosing one of the diners as the start.
//!
//! We solve part two at the same time by noticing that by inserting yourself between two diners
//! you set the value of their mutual link to zero. Keeping tracking of the weakest link
//! then subtracting that from the value for part one gives the result for part two at almost
//! no additional cost.
//!
//! [`Day 9`]: crate::year2015::day09
use crate::util::hash::*;
use crate::util::parse::*;
use crate::util::slice::*;

type Input = (i32, i32);

pub fn parse(input: &str) -> Input {
    // Assign each diner an index on a first come first served basis.
    let lines: Vec<Vec<_>> = input.lines().map(|line| line.split([' ', '.']).collect()).collect();
    let mut indices = FastMap::new();

    for tokens in &lines {
        let size = indices.len();
        indices.entry(tokens[0]).or_insert(size);

        let size = indices.len();
        indices.entry(tokens[10]).or_insert(size);
    }

    // Calculate the happiness values. Note that the values are not reciprocal a => b != b => a.
    let stride = indices.len();
    let mut happiness = vec![0; stride * stride];

    for tokens in &lines {
        let start = indices[tokens[0]];
        let end = indices[tokens[10]];
        let sign = if tokens[2] == "gain" { 1 } else { -1 };
        let value: i32 = tokens[3].signed();

        // Add the values together to make the mutual link reciprocal
        happiness[stride * start + end] += sign * value;
        happiness[stride * end + start] += sign * value;
    }

    // Solve both parts simultaneously.
    let mut part_one = 0;
    let mut part_two = 0;
    let mut indices: Vec<_> = (1..stride).collect();

    indices.permutations(|slice| {
        let mut sum = 0;
        let mut weakest_link = i32::MAX;

        let mut link = |from, to| {
            let value = happiness[stride * from + to];
            sum += value;
            weakest_link = weakest_link.min(value);
        };

        link(0, slice[0]);
        link(0, slice[slice.len() - 1]);

        for i in 1..slice.len() {
            link(slice[i], slice[i - 1]);
        }

        part_one = part_one.max(sum);
        part_two = part_two.max(sum - weakest_link);
    });

    (part_one, part_two)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}
