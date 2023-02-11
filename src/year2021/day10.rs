pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.as_bytes()).collect()
}

pub fn part1(input: &[&[u8]]) -> u64 {
    let mut stack: Vec<u8> = Vec::new();
    let mut score = 0;

    for line in input {
        score += syntax_score(line, &mut stack);
        stack.clear();
    }

    score
}

pub fn part2(input: &[&[u8]]) -> u64 {
    let mut stack: Vec<u8> = Vec::new();
    let mut scores = Vec::new();

    for line in input {
        if syntax_score(line, &mut stack) == 0 {
            scores.push(autocomplete_score(&stack));
        }
        stack.clear();
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn syntax_score(line: &[u8], stack: &mut Vec<u8>) -> u64 {
    for &b in line.iter() {
        match b {
            b'(' | b'[' | b'{' | b'<' => stack.push(b),
            b')' => {
                if stack.pop().unwrap() != b'(' {
                    return 3;
                }
            },
            b']' => {
                if stack.pop().unwrap() != b'[' {
                    return 57;
                }
            },
            b'}' => {
                if stack.pop().unwrap() != b'{' {
                    return 1197;
                }
            },
            b'>' => {
                if stack.pop().unwrap() != b'<' {
                    return 25137;
                }
            },
            _ => unreachable!(),
        }
    }

    0
}

fn autocomplete_score(stack: &[u8]) -> u64 {
    fn helper(b: u8) -> u64 {
        match b {
            b'(' => 1,
            b'[' => 2,
            b'{' => 3,
            b'<' => 4,
            _ => unreachable!(),
        }
    }
    stack.iter().rev().fold(0, |acc, &b| 5 * acc + helper(b))
}
