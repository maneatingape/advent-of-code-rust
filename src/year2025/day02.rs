//! # Gift Shop
use crate::util::iter::*;
use crate::util::parse::*;

type Range = [u32; 2];
type Pair = [u64; 2];

const FIRST: [Range; 5] = [[2, 1], [4, 2], [6, 3], [8, 4], [10, 5]];
const SECOND: [Range; 6] = [[3, 1], [5, 1], [6, 2], [7, 1], [9, 3], [10, 2]];
const THIRD: [Range; 2] = [[6, 1], [10, 1]];

pub fn parse(input: &str) -> Pair {
    let ranges: Vec<_> = input.iter_unsigned::<u64>().chunk::<2>().collect();
    let part_one = sum(&FIRST, &ranges);
    let part_two = part_one + sum(&SECOND, &ranges) - sum(&THIRD, &ranges);
    [part_one, part_two]
}

pub fn part1(input: &Pair) -> u64 {
    input[0]
}

pub fn part2(input: &Pair) -> u64 {
    input[1]
}

fn sum(ranges: &[Range], ids: &[Pair]) -> u64 {
    let mut result = 0;

    for &[digits, size] in ranges {
        let digits_power = 10_u64.pow(digits);
        let size_power = 10_u64.pow(size);

        let step = (digits_power - 1) / (size_power - 1);
        let start = step * (size_power / 10);
        let end = step * (size_power - 1);

        for &[from, to] in ids {
            let lower = from.next_multiple_of(step).max(start);
            let upper = to.min(end);

            if lower <= upper {
                let n = (upper - lower) / step;
                let triangular = n * (n + 1) / 2;
                result += lower * (n + 1) + step * triangular;
            }
        }
    }

    result
}
