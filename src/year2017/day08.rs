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

    for [target, operation, amount, _, source, comparison, threshold] in
        input.split_ascii_whitespace().chunk::<7>()
    {
        let value = registers[to_index(source)];
        let threshold = threshold.signed();

        let predicate = match comparison {
            "==" => value == threshold,
            "!=" => value != threshold,
            ">=" => value >= threshold,
            "<=" => value <= threshold,
            ">" => value > threshold,
            "<" => value < threshold,
            _ => unreachable!(),
        };

        if predicate {
            let register = &mut registers[to_index(target)];
            let amount: i32 = amount.signed();

            match operation {
                "inc" => *register += amount,
                "dec" => *register -= amount,
                _ => unreachable!(),
            }

            part_two = part_two.max(*register);
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
