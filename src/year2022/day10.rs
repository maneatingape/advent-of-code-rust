use crate::util::parse::*;

pub fn parse(input: &str) -> Vec<i32> {
    let mut x = 1;
    let mut xs: Vec<i32> = vec![1];

    for token in input.split_ascii_whitespace() {
        match token {
            "noop" => (),
            "addx" => (),
            delta => x += from::<i32>(delta),
        }
        xs.push(x);
    }

    xs
}

pub fn part1(input: &[i32]) -> i32 {
    input
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, x)| ((i + 1) as i32) * x)
        .sum()
}

pub fn part2(input: &[i32]) -> String {
    let mut result = input
        .chunks_exact(40)
        .map(|row| row.iter().enumerate().map(|(i, c)| if ((i as i32) - c).abs() <= 1 { '#' } else { '.' }).collect())
        .collect::<Vec<String>>()
        .join("\n");
    result.insert(0, '\n');
    result
}
