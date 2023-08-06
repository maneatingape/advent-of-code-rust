//! # Extended Polymerization
//!
//! The key insight to this problem is the same as [`Day 6`]. We track the *total* number of
//! each pair as the positions don't affect the final result.
//!
//! Fixed sized arrays are used for speed as we know that the elements are limited to 26 values
//! and the possible pairs to 26 * 26 values.
//!
//! [`Day 6`]: crate::year2021::day06
use crate::util::iter::*;

type Elements = [u64; 26];
type Pairs = [u64; 26 * 26];
type Rules = Vec<Rule>;

pub struct Rule {
    from: usize,
    to_left: usize,
    to_right: usize,
    element: usize,
}

impl Rule {
    fn parse([a, b, c]: [u8; 3]) -> Rule {
        let from = pair(a, b);
        let to_left = pair(a, c);
        let to_right = pair(c, b);
        let element = element(c);
        Rule { from, to_left, to_right, element }
    }
}

pub struct Input {
    elements: Elements,
    pairs: Pairs,
    rules: Rules,
}

/// Count the initial pairs and elements and parse each instruction into a [`Rule`] struct.
pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let prefix = prefix.trim().as_bytes();

    let mut elements = [0; 26];
    prefix.iter().for_each(|&b| elements[element(b)] += 1);

    let mut pairs = [0; 26 * 26];
    prefix.windows(2).for_each(|w| pairs[pair(w[0], w[1])] += 1);

    let rules: Vec<_> =
        suffix.bytes().filter(u8::is_ascii_uppercase).chunk::<3>().map(Rule::parse).collect();

    Input { elements, pairs, rules }
}

/// Apply 10 steps.
pub fn part1(input: &Input) -> u64 {
    steps(input, 10)
}

/// Apply 40 steps.
pub fn part2(input: &Input) -> u64 {
    steps(input, 40)
}

/// Simulate an arbitrary number of steps.
///
/// A rule `AC` -> `ABC` implies that for each pair `AC` we create an equal number of pairs
/// `AB` and `BC`, then increment the amount of element `B`.
fn steps(input: &Input, rounds: usize) -> u64 {
    let mut elements = input.elements;
    let mut pairs = input.pairs;
    let rules = &input.rules;

    for _ in 0..rounds {
        let mut next: Pairs = [0; 26 * 26];

        for rule in rules {
            let n = pairs[rule.from];
            next[rule.to_left] += n;
            next[rule.to_right] += n;
            elements[rule.element] += n;
        }

        pairs = next;
    }

    let max = elements.iter().max().unwrap();
    let min = elements.iter().filter(|&&n| n > 0).min().unwrap();
    max - min
}

/// Convert a single uppercase ASCII character to an index between 0 and 25
fn element(byte: u8) -> usize {
    (byte - b'A') as usize
}

/// Convert two uppercase ASCII characters to an index between 0 and 675.
fn pair(first: u8, second: u8) -> usize {
    26 * element(first) + element(second)
}
