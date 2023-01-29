use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<i64> {
    input.to_signed_iter().collect()
}

pub fn part1(input: &[i64]) -> i64 {
    decrypt(input, 1, 1)
}

pub fn part2(input: &[i64]) -> i64 {
    decrypt(input, 811589153, 10)
}

fn decrypt(input: &[i64], key: i64, rounds: usize) -> i64 {
    let mut mixed: Vec<(usize, i64)> = input.iter().map(|x| x * key).enumerate().collect();

    for _ in 0..rounds {
        for index in 0..input.len() {
            let from = mixed.iter().position(|(i, _)| *i == index).unwrap();
            let pair @ (_, number) = mixed.remove(from);
            let to = (from as i64 + number).rem_euclid(mixed.len() as i64) as usize;
            mixed.insert(to, pair);
        }
    }

    let start = mixed.iter().position(|(_, x)| *x == 0).unwrap();
    (1..4).map(|offset| mixed[(start + 1000 * offset) % mixed.len()].1).sum()
}