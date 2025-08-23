//! # Giant Squid
//!
//! We use a trick to immediately calculate the winning turn and score for each board.
//!
//! First we create a bidirectional map between each number and turn that it's drawn. Since the
//! numbers are at most 2 digits we can use a fixed size array instead of a `HashMap` for speed.
//!
//! Then for each column and row within a board, map each number to a turn and take the maximum
//! value. This is the turn that the row or column will win. Then take the *minimum* of
//! these maximum values. This is the turn that the entire board will win.
//!
//! Filtering the board numbers by turn and a reverse lookup from turn to number gives the
//! score for each board. Sort each result by turn and the answers for part one and two are the
//! first and last values respectively.
use crate::util::parse::*;
use std::array::from_fn;

const BOARD_SIZE: usize = 25;
const ROWS_AND_COLS: [(usize, usize); 10] =
    [(0, 1), (5, 1), (10, 1), (15, 1), (20, 1), (0, 5), (1, 5), (2, 5), (3, 5), (4, 5)];

pub struct Board {
    turn: usize,
    score: usize,
}

pub fn parse(input: &str) -> Vec<Board> {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let boards: Vec<_> = suffix.iter_unsigned().collect();

    let mut number_to_turn = [0; 100];
    let mut turn_to_number = [0; 100];

    for (i, n) in prefix.iter_unsigned().enumerate() {
        number_to_turn[n] = i;
        turn_to_number[i] = n;
    }

    boards
        .chunks_exact(BOARD_SIZE)
        .map(|board| {
            let turns: [usize; BOARD_SIZE] = from_fn(|i| number_to_turn[board[i]]);
            let max = |&(skip, step)| *turns.iter().skip(skip).step_by(step).take(5).max().unwrap();

            let winning_turn = ROWS_AND_COLS.iter().map(max).min().unwrap();
            let unmarked: usize = board.iter().filter(|&&n| number_to_turn[n] > winning_turn).sum();
            let just_called = turn_to_number[winning_turn];

            Board { turn: winning_turn, score: unmarked * just_called }
        })
        .collect()
}

pub fn part1(input: &[Board]) -> usize {
    input.iter().min_by_key(|b| b.turn).unwrap().score
}

pub fn part2(input: &[Board]) -> usize {
    input.iter().max_by_key(|b| b.turn).unwrap().score
}
