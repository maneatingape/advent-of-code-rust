use crate::util::collection::*;
use crate::util::parse::to_tuple_3;

type Input = (Stack, Vec<Move>);
type Stack = Vec<Vec<char>>;
type Move = (usize, usize, usize);

pub fn parse(input: &str) -> Input {
    let lines: Vec<&str> = input.lines().collect();
    let width = (lines[0].len() + 1) / 4;
    let height = lines.iter().position(|s| s.is_empty()).unwrap();

    let mut stack: Stack = Vec::tabulate(width, |_| Vec::new());
    for row in lines.iter().take(height - 1).rev() {
        for (i, c) in row.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stack[i].push(c);
            }
        }
    }

    fn helper(line: &&str) -> Move {
        let (amount, from, to): Move = to_tuple_3(line);
        (amount, from - 1, to - 1)
    }
    let moves: Vec<Move> = lines.iter().skip(height + 1).map(helper).collect();

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

    for (amount, from, to) in moves {
        let start = stack[*from].len() - amount;
        crates.extend(stack[*from].drain(start..));
        if reverse {
            stack[*to].extend(crates.iter().rev());
        } else {
            stack[*to].extend(crates.iter());
        }
        crates.clear();
    }

    stack.iter().map(|v| v.last().unwrap()).collect()
}
