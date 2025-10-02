//! # Password Philosophy
//!
//! Parsing the rules upfront allows both part 1 and part 2 to be solved in a straightforward manner.
//!
//! There's no need to first convert the input into lines since we know that each rule has 4 parts.
//! Instead we use the [`split`] method with a slice of delimiters to break the input into
//! an `Iterator` of tokens, then use our utility [`chunk`] method to group the tokens into an
//! array of size 4.
//!
//! [`split`]: slice::split
//! [`chunk`]: crate::util::iter
use crate::util::iter::*;
use crate::util::parse::*;

pub struct Rule<'a> {
    start: usize,
    end: usize,
    letter: u8,
    password: &'a [u8],
}

impl Rule<'_> {
    fn from([a, b, c, d]: [&str; 4]) -> Rule<'_> {
        let start = a.unsigned();
        let end = b.unsigned();
        let letter = c.as_bytes()[0];
        let password = d.as_bytes();
        Rule { start, end, letter, password }
    }
}

pub fn parse(input: &str) -> Vec<Rule<'_>> {
    input
        .split(['-', ':', ' ', '\n'])
        .filter(|s| !s.is_empty())
        .chunk::<4>()
        .map(Rule::from)
        .collect()
}

pub fn part1(input: &[Rule<'_>]) -> usize {
    input
        .iter()
        .filter(|rule| {
            let count = rule.password.iter().filter(|&&l| l == rule.letter).count();
            rule.start <= count && count <= rule.end
        })
        .count()
}

pub fn part2(input: &[Rule<'_>]) -> usize {
    input
        .iter()
        .filter(|rule| {
            let first = rule.password[rule.start - 1] == rule.letter;
            let second = rule.password[rule.end - 1] == rule.letter;
            first ^ second
        })
        .count()
}
