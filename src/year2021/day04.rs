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
//! score for each board. Sort each result by turn and the answers for part 1 and part1 are the
//! first and last values respectively.
use crate::util::parse::*;
use std::array::from_fn;

pub struct Input {
    turn: usize,
    score: usize,
}

pub fn parse(input: &str) -> Vec<Input> {
    let mut to_turn = [0; 100];
    let mut from_turn = [0; 100];

    let mut chunks = input.split("\n\n");

    for (i, n) in chunks.next().unwrap().iter_unsigned().enumerate() {
        to_turn[n] = i;
        from_turn[i] = n;
    }

    let score = |chunk: &str| {
        let mut iter = chunk.iter_unsigned();
        let board: [usize; 25] = from_fn(|_| iter.next().unwrap());
        let turns: [usize; 25] = from_fn(|i| to_turn[board[i]]);

        let row_and_cols = [
            turns[0..5].iter().max().unwrap(),
            turns[5..10].iter().max().unwrap(),
            turns[10..15].iter().max().unwrap(),
            turns[15..20].iter().max().unwrap(),
            turns[20..25].iter().max().unwrap(),
            turns.iter().step_by(5).max().unwrap(),
            turns.iter().skip(1).step_by(5).max().unwrap(),
            turns.iter().skip(2).step_by(5).max().unwrap(),
            turns.iter().skip(3).step_by(5).max().unwrap(),
            turns.iter().skip(4).step_by(5).max().unwrap(),
        ];
        let winning_turn = **row_and_cols.iter().min().unwrap();
        let unmarked: usize = board.iter().filter(|&&n| to_turn[n] > winning_turn).sum();
        let just_called = from_turn[winning_turn];

        Input { turn: winning_turn, score: unmarked * just_called }
    };

    let mut scores: Vec<_> = chunks.map(score).collect();
    scores.sort_unstable_by_key(|s| s.turn);
    scores
}

pub fn part1(input: &[Input]) -> usize {
    input.first().unwrap().score
}

pub fn part2(input: &[Input]) -> usize {
    input.last().unwrap().score
}
