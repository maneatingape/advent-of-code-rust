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

pub struct Input {
    turn: usize,
    score: usize,
}

pub fn parse(input: &str) -> Vec<Input> {
    let mut to_turn = [0; 100];
    let mut from_turn = [0; 100];

    let chunks: Vec<Vec<usize>> =
        input.split("\n\n").map(|s| s.iter_unsigned().collect()).collect();

    for (i, &n) in chunks[0].iter().enumerate() {
        to_turn[n] = i;
        from_turn[i] = n;
    }

    let score = |board: &Vec<usize>| {
        let turns: Vec<_> = board.iter().map(|&n| to_turn[n]).collect();
        let row_and_cols = [
            turns[0..5].iter().max().unwrap(),
            turns[5..10].iter().max().unwrap(),
            turns[10..15].iter().max().unwrap(),
            turns[15..20].iter().max().unwrap(),
            turns[20..25].iter().max().unwrap(),
            turns.iter().skip(0).step_by(5).max().unwrap(),
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

    let mut result: Vec<_> = chunks.iter().skip(1).map(score).collect();
    result.sort_unstable_by(|a, b| a.turn.cmp(&b.turn));
    result
}

pub fn part1(input: &[Input]) -> usize {
    input[0].score
}

pub fn part2(input: &[Input]) -> usize {
    input[input.len() - 1].score
}
