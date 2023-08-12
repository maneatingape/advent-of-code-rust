use crate::util::hash::*;
use crate::util::parse::*;

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

enum Monkey {
    Number(i64),
    Result(usize, Operation, usize),
}

impl Monkey {
    fn parse(str: &str, indices: &FastMap<&str, usize>) -> Monkey {
        if str.len() < 11 {
            Monkey::Number(str.signed())
        } else {
            let left = indices[&str[0..4]];
            let right = indices[&str[7..11]];
            let operation = match str.as_bytes()[5] {
                b'+' => Operation::Add,
                b'-' => Operation::Sub,
                b'*' => Operation::Mul,
                b'/' => Operation::Div,
                _ => unreachable!(),
            };
            Monkey::Result(left, operation, right)
        }
    }
}

pub struct Input {
    root: usize,
    monkeys: Vec<Monkey>,
    yell: Vec<i64>,
    unknown: Vec<bool>,
}

pub fn parse(input: &str) -> Input {
    let lines: Vec<_> = input.lines().collect();

    let indices: FastMap<_, _> =
        lines.iter().enumerate().map(|(index, line)| (&line[0..4], index)).collect();

    let monkeys: Vec<_> = lines.iter().map(|line| Monkey::parse(&line[6..], &indices)).collect();

    let root = indices["root"];
    let humn = indices["humn"];
    let mut input =
        Input { root, monkeys, yell: vec![0; lines.len()], unknown: vec![false; lines.len()] };

    compute(&mut input, root);
    find(&mut input, humn, root);
    input
}

pub fn part1(input: &Input) -> i64 {
    let Input { yell, root, .. } = input;
    yell[*root]
}

pub fn part2(input: &Input) -> i64 {
    let Input { root, .. } = input;
    inverse(input, *root, -1)
}

fn compute(input: &mut Input, index: usize) -> i64 {
    let result = match input.monkeys[index] {
        Monkey::Number(n) => n,
        Monkey::Result(left, operation, right) => match operation {
            Operation::Add => compute(input, left) + compute(input, right),
            Operation::Sub => compute(input, left) - compute(input, right),
            Operation::Mul => compute(input, left) * compute(input, right),
            Operation::Div => compute(input, left) / compute(input, right),
        },
    };
    input.yell[index] = result;
    result
}

fn find(input: &mut Input, humn: usize, index: usize) -> bool {
    let result = match input.monkeys[index] {
        Monkey::Number(_) => humn == index,
        Monkey::Result(left, _, right) => find(input, humn, left) || find(input, humn, right),
    };
    input.unknown[index] = result;
    result
}

fn inverse(input: &Input, index: usize, value: i64) -> i64 {
    let Input { root, yell, unknown, monkeys } = input;

    match monkeys[index] {
        Monkey::Number(_) => value,
        Monkey::Result(left, _, right) if index == *root => {
            if unknown[left] {
                inverse(input, left, yell[right])
            } else {
                inverse(input, right, yell[left])
            }
        }
        Monkey::Result(left, operation, right) => {
            if unknown[left] {
                match operation {
                    Operation::Add => inverse(input, left, value - yell[right]),
                    Operation::Sub => inverse(input, left, value + yell[right]),
                    Operation::Mul => inverse(input, left, value / yell[right]),
                    Operation::Div => inverse(input, left, value * yell[right]),
                }
            } else {
                match operation {
                    Operation::Add => inverse(input, right, value - yell[left]),
                    Operation::Sub => inverse(input, right, yell[left] - value),
                    Operation::Mul => inverse(input, right, value / yell[left]),
                    Operation::Div => inverse(input, right, yell[left] / value),
                }
            }
        }
    }
}
