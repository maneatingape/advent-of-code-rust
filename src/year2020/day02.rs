use crate::util::iter::*;
use crate::util::parse::*;

pub struct Rule<'a> {
    start: usize,
    end: usize,
    letter: u8,
    password: &'a [u8],
}

impl Rule<'_> {
    fn from([a, b, c, d]: [&str; 4]) -> Rule {
        let start = from(a);
        let end = from(b);
        let letter = c.as_bytes()[0];
        let password = d.as_bytes();
        Rule { start, end, letter, password }
    }
}

pub fn parse(input: &str) -> Vec<Rule> {
    input
        .split(['-', ':', ' ', '\n'])
        .filter(|s| !s.is_empty())
        .chunk::<4>()
        .map(Rule::from)
        .collect()
}

pub fn part1(input: &[Rule]) -> usize {
    input
        .iter()
        .filter(|rule| {
            let count = rule.password.iter().filter(|&&l| l == rule.letter).count();
            rule.start <= count && count <= rule.end
        })
        .count()
}

pub fn part2(input: &[Rule]) -> usize {
    input
        .iter()
        .filter(|rule| {
            let first = rule.password[rule.start - 1] == rule.letter;
            let second = rule.password[rule.end - 1] == rule.letter;
            first ^ second
        })
        .count()
}
