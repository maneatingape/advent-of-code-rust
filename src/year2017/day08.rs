//! # I Heard You Like Registers
//!
//! Computes both parts in a single pass.
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;

type Input = (i32, i32);

pub fn parse(input: &str) -> Input {
    let mut registers = FastMap::new();
    let mut part_two = 0;

    for [a, b, c, _, e, f, g] in input.split_ascii_whitespace().chunk::<7>() {
        let first = *registers.entry(e).or_insert(0);
        let second: i32 = g.signed();

        let predicate = match f {
            "==" => first == second,
            "!=" => first != second,
            ">=" => first >= second,
            "<=" => first <= second,
            ">" => first > second,
            "<" => first < second,
            _ => unreachable!(),
        };

        if predicate {
            let third = registers.entry(a).or_insert(0);
            let fourth: i32 = c.signed();

            match b {
                "inc" => *third += fourth,
                "dec" => *third -= fourth,
                _ => unreachable!(),
            }

            part_two = part_two.max(*third);
        }
    }

    let part_one = *registers.values().max().unwrap();
    (part_one, part_two)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}
