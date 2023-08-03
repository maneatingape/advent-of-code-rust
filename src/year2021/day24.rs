use crate::util::parse::*;
use Round::*;

enum Round {
    Push(i32),
    Pop(i32),
}

pub struct Constraint {
    index: usize,
    value: i32,
}

impl Constraint {
    fn min(&self) -> i32 {
        (1 + self.value).max(1)
    }

    fn max(&self) -> i32 {
        (9 + self.value).min(9)
    }
}

pub fn parse(input: &str) -> Vec<Constraint> {
    let lines: Vec<_> = input.lines().collect();
    let rounds: Vec<_> = lines
        .chunks(18)
        .map(|chunk| {
            let helper = |i: usize| {
                let token = chunk[i].split_ascii_whitespace().last().unwrap();
                token.signed()
            };
            if helper(4) == 1 {
                Push(helper(15))
            } else {
                Pop(helper(5))
            }
        })
        .collect();

    let mut stack = Vec::new();
    let mut constraints = Vec::new();

    for (index, round) in rounds.into_iter().enumerate() {
        match round {
            Push(value) => stack.push(Constraint { index, value }),
            Pop(second_value) => {
                let mut first = stack.pop().unwrap();
                let delta = first.value + second_value;
                first.value = -delta;
                let second = Constraint { index, value: delta };
                constraints.push(first);
                constraints.push(second);
            }
        }
    }

    constraints.sort_unstable_by_key(|c| c.index);
    constraints
}

pub fn part1(input: &[Constraint]) -> String {
    input.iter().map(|c| c.max().to_string()).collect()
}

pub fn part2(input: &[Constraint]) -> String {
    input.iter().map(|c| c.min().to_string()).collect()
}
