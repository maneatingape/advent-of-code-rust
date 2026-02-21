//! # Scrambled Letters and Hash
//!
//! The forward transformations are straightforward. The trickiest reverse transformation is the
//! rotation based on the index of the letter. First we build a lookup table of how many places to
//! rotate right based on the letter index. This is +1 for positions 0-3 and +2 for positions 4-7.
//!
//! Then we invert this by mapping the transformed index to the rotation. For example position 3 is
//! rotated right by 4 places, ending up at position 7, so the inverse lookup table to rotate left
//! stores 4 at index 7.
use crate::util::parse::*;

const ROTATE_LETTER_RIGHT: [usize; 8] = [1, 2, 3, 4, 6, 7, 0, 1];
const ROTATE_LETTER_LEFT: [usize; 8] = [1, 1, 6, 2, 7, 3, 0, 4];

#[derive(Clone, Copy)]
pub enum Op {
    SwapPosition(usize, usize),
    SwapLetter(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetterLeft(u8),
    RotateLetterRight(u8),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Op {
    fn from(line: &str) -> Op {
        let tokens: Vec<_> = line.split_ascii_whitespace().collect();
        let digit = |i: usize| tokens[i].unsigned();
        let letter = |i: usize| tokens[i].as_bytes()[0];

        match tokens[0] {
            "reverse" => Op::Reverse(digit(2), digit(4)),
            "move" => Op::Move(digit(2), digit(5)),
            _ => match tokens[1] {
                "position" => Op::SwapPosition(digit(2), digit(5)),
                "letter" => Op::SwapLetter(letter(2), letter(5)),
                "left" => Op::RotateLeft(digit(2)),
                "right" => Op::RotateRight(digit(2)),
                "based" => Op::RotateLetterRight(letter(6)),
                _ => unreachable!(),
            },
        }
    }

    fn transform(self, password: &mut Vec<u8>) {
        let position = |a: u8| password.iter().position(|&b| a == b).unwrap();

        match self {
            Op::SwapPosition(first, second) => password.swap(first, second),
            Op::SwapLetter(first, second) => {
                let first = position(first);
                let second = position(second);
                password.swap(first, second);
            }
            Op::RotateLeft(first) => password.rotate_left(first),
            Op::RotateRight(first) => password.rotate_right(first),
            Op::RotateLetterLeft(first) => {
                let first = position(first);
                let second = ROTATE_LETTER_LEFT[first] % password.len();
                password.rotate_left(second);
            }
            Op::RotateLetterRight(first) => {
                let first = position(first);
                let second = ROTATE_LETTER_RIGHT[first] % password.len();
                password.rotate_right(second);
            }
            Op::Reverse(first, second) => password[first..=second].reverse(),
            Op::Move(first, second) => {
                let letter = password.remove(first);
                password.insert(second, letter);
            }
        }
    }

    fn inverse(self) -> Op {
        match self {
            Op::RotateLeft(first) => Op::RotateRight(first),
            Op::RotateRight(first) => Op::RotateLeft(first),
            Op::RotateLetterLeft(first) => Op::RotateLetterRight(first),
            Op::RotateLetterRight(first) => Op::RotateLetterLeft(first),
            Op::Move(first, second) => Op::Move(second, first),
            // Other operations are their own inverse.
            other => other,
        }
    }
}

pub fn parse(input: &str) -> Vec<Op> {
    input.lines().map(Op::from).collect()
}

pub fn part1(input: &[Op]) -> String {
    scramble(input, b"abcdefgh")
}

pub fn part2(input: &[Op]) -> String {
    unscramble(input, b"fbgdceah")
}

pub fn scramble(input: &[Op], slice: &[u8]) -> String {
    let mut password = slice.to_vec();

    for op in input {
        op.transform(&mut password);
    }

    String::from_utf8(password).unwrap()
}

pub fn unscramble(input: &[Op], slice: &[u8]) -> String {
    let mut password = slice.to_vec();

    for op in input.iter().rev() {
        op.inverse().transform(&mut password);
    }

    String::from_utf8(password).unwrap()
}
