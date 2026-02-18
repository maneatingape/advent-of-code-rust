//! # Monkey Math
//!
//! The Monkeys form a [binary tree](https://en.wikipedia.org/wiki/Binary_tree). We first
//! compute the result by recursively following the structure all the way to the leaves.
//! We also find the `humn` node and all its parents the same way, marking them as "unknown".
//!
//! For part two we know that the value on the left and the right of the root must be equal.
//! Following the tree down the path previously marked "unknown" we recursively solve
//! equations until we reach the `humn` node.
//!
//! For example say the root's children are `a` and `b`
//!
//! ```none
//!     yell[a] = 6
//!     unknown[a] = false
//!     yell[b] = 5
//!     unknown[b] = true
//! ```
//!
//! So this implies `b` is a parent of `humn` and must equal `6` to pass (the current value is
//! irrelevant). We then recursively look at the children of `b`:
//!
//! ```none
//!     yell[c] = 4
//!     unknown[a] = true
//!     operation = "+"
//!     yell[d] = 4
//!     unknown[b] = false
//! ```
//!
//! We know that `c + d` must equal 6 so this implies `c = 2`. We then recursively look at the
//! children of `c`
//!
//! ```none
//!     yell[humn] = 123
//!     unknown[a] = true
//! ```
//!
//! Once we finally reach the `humn` node the value that we currently have `2` is the answer.
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

    // Assign each monkey an index on a first come first served basis.
    let indices: FastMap<_, _> =
        lines.iter().enumerate().map(|(index, line)| (&line[0..4], index)).collect();

    let monkeys: Vec<_> = lines.iter().map(|line| Monkey::parse(&line[6..], &indices)).collect();

    // We only need the specific indices of the root and human.
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

/// Recursively compute the total following the tree structure all the way to the leaves.
fn compute(input: &mut Input, index: usize) -> i64 {
    let result = match input.monkeys[index] {
        Monkey::Number(n) => n,
        Monkey::Result(left, operation, right) => {
            let l = compute(input, left);
            let r = compute(input, right);
            match operation {
                Operation::Add => l + r,
                Operation::Sub => l - r,
                Operation::Mul => l * r,
                Operation::Div => l / r,
            }
        }
    };
    // Cache the computed value for use in part two.
    input.yell[index] = result;
    result
}

/// Recursively find the humn node then mark it and all its parents all the way to the
/// root as "unknown".
fn find(input: &mut Input, humn: usize, index: usize) -> bool {
    let result = match input.monkeys[index] {
        Monkey::Number(_) => humn == index,
        Monkey::Result(left, _, right) => find(input, humn, left) || find(input, humn, right),
    };
    input.unknown[index] = result;
    result
}

/// Recursively finds the value of the expression on the "unknown" side so that it equals the
/// known side.
fn inverse(input: &Input, index: usize, value: i64) -> i64 {
    let Input { root, yell, unknown, monkeys } = input;

    match monkeys[index] {
        // The only leaf node we'll actually ever reach is the "humn" node so the value at this
        // point is the answer.
        Monkey::Number(_) => value,
        // If we're the root then the left and right side must be equal.
        Monkey::Result(left, _, right) if index == *root => {
            if unknown[left] {
                inverse(input, left, yell[right])
            } else {
                inverse(input, right, yell[left])
            }
        }
        // Addition and multiplication are commutative, but subtraction and division are not,
        // so we have to handle unknowns on the right and left differently.
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
