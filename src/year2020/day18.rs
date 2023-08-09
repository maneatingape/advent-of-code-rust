//! # Operation Order
//!
//! We use the [shunting yard algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm)
//! to convert each expressions from infix to [postfix](https://en.wikipedia.org/wiki/Reverse_Polish_notation).
//!
//! Postfix expressions are straightforward to evaluate using a stack (and there's even
//! an [entire language](https://en.wikipedia.org/wiki/Forth_(programming_language))
//! designed around them).
#[derive(PartialEq, Eq)]
enum Token {
    Number(u64),
    Add,
    Mul,
    Open,
}

pub fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

pub fn part1(input: &[&[u8]]) -> u64 {
    eval(input, true)
}
pub fn part2(input: &[&[u8]]) -> u64 {
    eval(input, false)
}

fn eval(input: &[&[u8]], part_one: bool) -> u64 {
    let mut output = Vec::new();
    let mut operator = Vec::new();

    let mut push = |token| match token {
        Token::Number(n) => output.push(n),
        Token::Add => {
            let a = output.pop().unwrap();
            let b = output.pop().unwrap();
            output.push(a + b);
        }
        Token::Mul => {
            let a = output.pop().unwrap();
            let b = output.pop().unwrap();
            output.push(a * b);
        }
        Token::Open => unreachable!(),
    };

    for &line in input {
        for &c in line {
            match c {
                b' ' => (),
                b'(' => operator.push(Token::Open),
                b')' => loop {
                    match operator.pop().unwrap() {
                        Token::Open => break,
                        next => push(next),
                    }
                },
                b'+' => {
                    loop {
                        match operator.last() {
                            Some(Token::Add) => push(operator.pop().unwrap()),
                            Some(Token::Mul) if part_one => push(operator.pop().unwrap()),
                            _ => break,
                        }
                    }
                    operator.push(Token::Add);
                }
                b'*' => {
                    while let Some(Token::Add | Token::Mul) = operator.last() {
                        push(operator.pop().unwrap());
                    }
                    operator.push(Token::Mul);
                }
                n if n.is_ascii_digit() => {
                    let n = (n - b'0') as u64;
                    push(Token::Number(n));
                }
                _ => unreachable!(),
            }
        }

        // Drains remaining operators to the output.
        while let Some(token) = operator.pop() {
            push(token);
        }
    }

    output.iter().sum()
}
