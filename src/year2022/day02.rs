pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> usize {
    play(input, &["", "B X", "C Y", "A Z", "A X", "B Y", "C Z", "C X", "A Y", "B Z"])
}

pub fn part2(input: &[&str]) -> usize {
    play(input, &["", "B X", "C X", "A X", "A Y", "B Y", "C Y", "C Z", "A Z", "B Z"])
}

fn play(input: &[&str], order: &[&str]) -> usize {
    input
        .iter()
        .map(|a| order.iter().position(|b| a == b).unwrap())
        .sum()
}
