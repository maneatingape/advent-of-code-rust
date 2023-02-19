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
    fn parse([&a, &b, &c]: [&u8; 3]) -> Rule {
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

pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let prefix = prefix.trim().as_bytes();

    let mut elements = [0; 26];
    prefix.iter().for_each(|&b| elements[element(b)] += 1);

    let mut pairs = [0; 26 * 26];
    prefix.windows(2).for_each(|w| pairs[pair(w[0], w[1])] += 1);

    let rules: Vec<_> = suffix
        .as_bytes()
        .iter()
        .filter(|b| b.is_ascii_uppercase())
        .chunk::<3>()
        .map(Rule::parse)
        .collect();

    Input { elements, pairs, rules }
}

pub fn part1(input: &Input) -> u64 {
    steps(input, 10)
}

pub fn part2(input: &Input) -> u64 {
    steps(input, 40)
}

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

fn element(byte: u8) -> usize {
    (byte - 65) as usize
}

fn pair(first: u8, second: u8) -> usize {
    26 * element(first) + element(second)
}
