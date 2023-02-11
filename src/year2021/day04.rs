use crate::util::parse::*;

pub struct Input {
    turn: usize,
    score: usize,
}

pub fn parse(input: &str) -> Vec<Input> {
    let mut to_turn = [0; 100];
    let mut from_turn = [0; 100];

    let chunks: Vec<Vec<usize>> = input
        .split("\n\n")
        .map(|s| s.iter_unsigned().collect())
        .collect();

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
        Input {
            turn: winning_turn,
            score: unmarked * just_called,
        }
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
