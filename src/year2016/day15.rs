//! # Timing is Everything
//!
//! Part one is the [Chinese Remainder Theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem).
//! The integers n₁, n₂, ... nₖ map to the disc sizes which happen to be prime. This satisfies the
//! requirement that the integers are [pairwise coprime](https://en.wikipedia.org/wiki/Coprime_integers#Coprimality_in_sets).
//!
//! For simplicity we use the "search by sieving" method. We start at zero with a step the size of
//! the first integer. Then we search for each subsequent integer located at the correct offset of
//! minutes and multiply the step by the new integer. This preserves the relative offset at each step
//! in the next search.
use crate::util::iter::*;
use crate::util::parse::*;

type Pair = [usize; 2];

pub fn parse(input: &str) -> Pair {
    let disks: Vec<Pair> = input.iter_unsigned().skip(1).step_by(2).chunk::<2>().collect();
    solve(&disks)
}

pub fn part1(input: &Pair) -> usize {
    input[0]
}

pub fn part2(input: &Pair) -> usize {
    input[1]
}

fn solve(discs: &[Pair]) -> Pair {
    let mut time = 0;
    let mut step = 1;

    for (offset, &[size, position]) in discs.iter().enumerate() {
        while !(time + offset + 1 + position).is_multiple_of(size) {
            time += step;
        }
        step *= size;
    }
    let p1 = time;
    // Part1 finished, solve for the additional disk with fixed size and position
    let (offset, size, position) = (discs.len(), 11, 0);
    while !(time + offset + 1 + position).is_multiple_of(size) {
        time += step;
    }
    // Part2 result
    let p2 = time;
    [p1, p2]
}
