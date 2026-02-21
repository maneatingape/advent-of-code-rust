//! # Syntax Scoring
//!
//! This day is a variation of the classic parentheses balancing problem. To solve we use a `vec`
//! as a stack, pushing opening delimiters onto the stack, then popping the top of the stack
//! whenever we encounter a closing delimiter. If there is a mismatch between opening and closing
//! delimiters then we return the specified error value immediately.
//!
//! For part two the completion score is the remaining items on the stack, reversed and converted from
//! corresponding closing delimiters. For example the completion string `])}>` would have a stack
//! that looks like `<{([`, where the right hand side is the top of the stack.
type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let mut stack = Vec::new();
    let mut scores = Vec::new();
    let mut part_one = 0;

    for line in input.lines() {
        let score = syntax_score(line, &mut stack);

        if score == 0 {
            scores.push(autocomplete_score(&stack));
        } else {
            part_one += score;
        }

        stack.clear();
    }

    scores.sort_unstable();
    let part_two = scores[scores.len() / 2];

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}

fn syntax_score(line: &str, stack: &mut Vec<u8>) -> u64 {
    for b in line.bytes() {
        match b {
            b'(' | b'[' | b'{' | b'<' => stack.push(b),
            b')' => {
                if stack.pop().unwrap() != b'(' {
                    return 3;
                }
            }
            b']' => {
                if stack.pop().unwrap() != b'[' {
                    return 57;
                }
            }
            b'}' => {
                if stack.pop().unwrap() != b'{' {
                    return 1197;
                }
            }
            b'>' => {
                if stack.pop().unwrap() != b'<' {
                    return 25137;
                }
            }
            _ => unreachable!(),
        }
    }

    0
}

fn autocomplete_score(stack: &[u8]) -> u64 {
    stack.iter().rev().fold(0, |acc, &b| {
        let score = match b {
            b'(' => 1,
            b'[' => 2,
            b'{' => 3,
            b'<' => 4,
            _ => unreachable!(),
        };
        5 * acc + score
    })
}
