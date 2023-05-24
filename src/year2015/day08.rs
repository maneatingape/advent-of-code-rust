const NEWLINE: u8 = 10;
const QUOTE: u8 = 34;
const SLASH: u8 = 92;
const ESCAPE: u8 = 120;

pub fn parse(input: &str) -> &[u8] {
    input.as_bytes()
}

pub fn part1(input: &[u8]) -> u32 {
    let (_, result) = input.iter().fold((false, 0), |(flag, count), &b| match (flag, b) {
        (true, ESCAPE) => (false, count + 3),
        (true, _) => (false, count + 1),
        (false, SLASH) => (true, count),
        (false, NEWLINE) => (false, count + 2),
        _ => (false, count),
    });
    result
}

pub fn part2(input: &[u8]) -> u32 {
    input
        .iter()
        .map(|&b| match b {
            QUOTE | SLASH => 1,
            NEWLINE => 2,
            _ => 0,
        })
        .sum()
}
