//! # Scrambled Letters and Hash
//!
//! The forward transformations are straightforward. The trickiest reverse transformation is the
//! rotation based on the index of the letter. First we build a lookup table of how many places to
//! rotate right based on the letter index. This is +1 for positions 0-3 and +2 for positions 4-7.
//!
//! Then we invert this by mapping the transformed index to the rotation. For example, position 3 is
//! rotated right by 4 places, ending up at position 7, so the inverse lookup table to rotate left
//! stores 4 at index 7.
use crate::util::parse::*;

const ROTATE_LETTER_RIGHT: [usize; 8] = [1, 2, 3, 4, 6, 7, 0, 1];
const ROTATE_LETTER_LEFT: [usize; 8] = [1, 1, 6, 2, 7, 3, 0, 4];

#[derive(Clone, Copy)]
pub enum Op {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetterLeft(char),
    RotateLetterRight(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Op {
    fn from(line: &str) -> Self {
        let tokens: Vec<_> = line.split_ascii_whitespace().collect();
        let digit = |i: usize| tokens[i].unsigned();
        let letter = |i: usize| tokens[i].chars().next().unwrap();

        match tokens[0] {
            "reverse" => Self::Reverse(digit(2), digit(4)),
            "move" => Self::Move(digit(2), digit(5)),
            _ => match tokens[1] {
                "position" => Self::SwapPosition(digit(2), digit(5)),
                "letter" => Self::SwapLetter(letter(2), letter(5)),
                "left" => Self::RotateLeft(digit(2)),
                "right" => Self::RotateRight(digit(2)),
                "based" => Self::RotateLetterRight(letter(6)),
                _ => unreachable!(),
            },
        }
    }

    fn transform(self, password: &mut Vec<char>) {
        let position = |a: char| password.iter().position(|&b| a == b).unwrap();

        match self {
            Self::SwapPosition(first, second) => password.swap(first, second),
            Self::SwapLetter(first, second) => {
                let first = position(first);
                let second = position(second);
                password.swap(first, second);
            }
            Self::RotateLeft(first) => password.rotate_left(first),
            Self::RotateRight(first) => password.rotate_right(first),
            Self::RotateLetterLeft(first) => {
                let first = position(first);
                let second = ROTATE_LETTER_LEFT[first] % password.len();
                password.rotate_left(second);
            }
            Self::RotateLetterRight(first) => {
                let first = position(first);
                let second = ROTATE_LETTER_RIGHT[first] % password.len();
                password.rotate_right(second);
            }
            Self::Reverse(first, second) => password[first..=second].reverse(),
            Self::Move(first, second) => {
                let letter = password.remove(first);
                password.insert(second, letter);
            }
        }
    }

    fn inverse(self) -> Self {
        match self {
            Self::RotateLeft(first) => Self::RotateRight(first),
            Self::RotateRight(first) => Self::RotateLeft(first),
            Self::RotateLetterLeft(first) => Self::RotateLetterRight(first),
            Self::RotateLetterRight(first) => Self::RotateLetterLeft(first),
            Self::Move(first, second) => Self::Move(second, first),
            // Other operations are their own inverse.
            other => other,
        }
    }
}

pub fn parse(input: &str) -> Vec<Op> {
    input.lines().map(Op::from).collect()
}

pub fn part1(input: &[Op]) -> String {
    scramble(input, "abcdefgh")
}

pub fn part2(input: &[Op]) -> String {
    unscramble(input, "fbgdceah")
}

pub fn scramble(input: &[Op], slice: &str) -> String {
    let mut password = slice.chars().collect();

    for op in input {
        op.transform(&mut password);
    }

    password.iter().collect()
}

pub fn unscramble(input: &[Op], slice: &str) -> String {
    let mut password = slice.chars().collect();

    for op in input.iter().rev() {
        op.inverse().transform(&mut password);
    }

    password.iter().collect()
}
