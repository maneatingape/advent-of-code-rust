//! # I Heard You Like Registers
//!
//! Computes both parts in a single pass. Each register name is between one and three letters,
//! so we use a base 27 index (counting `a` as 1 and a blank space as 0) into a `vec` which is
//! faster than using a hashmap.
use crate::util::iter::*;
use crate::util::parse::*;

type Input = (i32, i32);

pub fn parse(input: &str) -> Input {
    let mut registers = vec![0; 27 * 27 * 27];
    let mut part_two = 0;

    for [a, b, c, _, e, f, g] in input.split_ascii_whitespace().chunk::<7>() {
        let first = registers[to_index(e)];
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
            let third = &mut registers[to_index(a)];
            let fourth: i32 = c.signed();

            match b {
                "inc" => *third += fourth,
                "dec" => *third -= fourth,
                _ => unreachable!(),
            }

            part_two = part_two.max(*third);
        }
    }

    let part_one = registers.into_iter().max().unwrap();
    (part_one, part_two)
}

pub fn part1(input: &Input) -> i32 {
    input.0
}

pub fn part2(input: &Input) -> i32 {
    input.1
}

fn to_index(s: &str) -> usize {
    s.bytes().fold(0, |acc, b| 27 * acc + usize::from(b - b'a' + 1))
}
