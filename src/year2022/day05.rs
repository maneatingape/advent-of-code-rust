use crate::util::collection::*;
use crate::util::parse::to_array3;

type Input = (Stack, Vec<Move>);
type Stack = Vec<Vec<char>>;
type Move = [usize; 3];

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
        let [amount, from, to]: Move = to_array3(line);
        [amount, from - 1, to - 1]
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

    for [amount, from, to] in moves {
        let start = stack[*from].len() - amount;
        let crates: Vec<char> = stack[*from].drain(start..).collect();
        if reverse {
            stack[*to].extend(crates.iter().rev());
        } else {
            stack[*to].extend(crates.iter());
        }
    }

    stack.iter().map(|v| v.last().unwrap()).collect()
}
