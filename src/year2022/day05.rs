use crate::util::collection::*;
use crate::util::parse::*;

type Stack = Vec<Vec<char>>;
type Move = [usize; 3];
type Input = (Stack, Vec<Move>);

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let lines: Vec<&str> = prefix.lines().collect();
    let width = (lines[0].len() + 1) / 4;

    let mut stack: Stack = vec![Vec::new(); width];
    for row in lines.iter().rev() {
        for (i, c) in row.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stack[i].push(c);
            }
        }
    }

    let moves: Vec<Move> = suffix
        .iter_unsigned()
        .chunked::<3>()
        .map(|[amount, from, to]| [amount, from - 1, to - 1])
        .collect();

    (stack, moves)
}

pub fn part1(input: &Input) -> String {
    play(input, true)
}

pub fn part2(input: &Input) -> String {
    play(input, false)
}

fn play(input: &Input, reverse: bool) -> String {
    let (initial, moves) = input;
    let mut stack = initial.clone();
    let mut crates: Vec<char> = Vec::new();

    for &[amount, from, to] in moves {
        let start = stack[from].len() - amount;
        crates.extend(stack[from].drain(start..));
        if reverse {
            stack[to].extend(crates.iter().rev());
        } else {
            stack[to].extend(crates.iter());
        }
        crates.clear();
    }

    stack.iter().map(|v| v.last().unwrap()).collect()
}
