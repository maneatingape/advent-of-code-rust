//! # Monster Messages
//!
//! Parsing the input has some nuances. Rust doesn't like recursive structs without indirection,
//! so for non-leaf rules we keep the rule number in order to lazily lookup the rule in a `vec`
//! later. This also handles parsing the rules in any order, as a rule may refer to another that
//! has not been parsed yet.
//!
//! My input created 2²¹ or 2097152 total valid matching sequences so trying to generate all
//! possibilities up front is much slower.
//!
//! ## Part One
//!
//! The `check` function implements a recursive matcher. If a rule is a prefix of the message
//! then the function returns `Some(index)` where `index` is the first character *after* the
//! matching pattern, in order to allow matching to continue with the next rule.
//! If there is no match then the function return `None`. For example:
//!
//! | Rule   | Message   | Result    |
//! | ------ | --------- | --------- |
//! | `aaaa` | `aaaab`   | `Some(4)` |
//! | `aa`   | `aaaab`   | `Some(2)` |
//! | `bb`   | `aaaab`   | `None`    |
//!
//! As rule 0 must match the *entire* message with no characters left over, we count only messages
//! with a result of `Some(len)` where `len` is the length of the complete message.
//!
//! ## Part Two
//!
//! First we do some detective work analyzing the new rules. Rule 8 is:
//! ```none
//!     8: 42 | 42 8
//! ```
//! This matches one or more repeated rule `42`s (in regex format this would be something like `42+`).
//!
//! Rule 11 is:
//! ```none
//!     11: 42 31 | 42 11 31
//! ```
//! This matches one or more nested pairs of rule 42 and 31, for example `42 31` or `42 42 31 31`.
//!
//! Assuming rule 0 is the same for all inputs:
//! ```none
//!     0: 8 11
//! ```
//! gives a pattern that matches:
//! 1. A sequence of two or more rule `42`
//! 2. Followed by a sequence of one or more rule `31`
//! 3. As long as the number of `42` matches are at least one greater than the number of `31` matches.
//!
//! For example `42 42 31` or `42 42 42 31` or `42 42 42 31 31` matches but *not* `42 42 31 31`.
//!
//! Since we don't need to handle the general input case (a common pattern in Advent of Code) we can
//! implement this rule directly in code.
use crate::util::parse::*;
use Rule::*;

#[derive(Clone, Copy)]
pub enum Rule {
    Letter(u8),
    Follow(usize),
    Choice(usize, usize),
    Sequence(usize, usize),
    Compound(usize, usize, usize, usize),
}

type Input<'a> = (Vec<Rule>, Vec<&'a [u8]>);

pub fn parse(input: &str) -> Input<'_> {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let mut tokens = Vec::new();
    let mut rules = vec![Letter(0); 640]; // 640 rules ought to be enough for anybody.

    for line in prefix.lines() {
        tokens.extend(line.iter_unsigned::<usize>());
        rules[tokens[0]] = match tokens[1..] {
            [] if line.contains('a') => Letter(b'a'),
            [] => Letter(b'b'),
            [a] => Follow(a),
            [a, b] if line.contains('|') => Choice(a, b),
            [a, b] => Sequence(a, b),
            [a, b, c, d] => Compound(a, b, c, d),
            _ => unreachable!(),
        };
        tokens.clear();
    }

    let messages = suffix.lines().map(str::as_bytes).collect();
    (rules, messages)
}

pub fn part1(input: &Input<'_>) -> usize {
    let (rules, messages) = input;
    messages.iter().filter(|message| check(rules, 0, message, 0) == Some(message.len())).count()
}

pub fn part2(input: &Input<'_>) -> usize {
    let (rules, messages) = input;
    let predicate = |message: &&&[u8]| {
        let mut index = 0;
        let mut first = 0;
        let mut second = 0;

        while let Some(next) = check(rules, 42, message, index) {
            index = next;
            first += 1;
        }

        if first >= 2 {
            while let Some(next) = check(rules, 31, message, index) {
                index = next;
                second += 1;
            }
        }

        index == message.len() && second >= 1 && (first > second)
    };
    messages.iter().filter(predicate).count()
}

fn check(rules: &[Rule], rule: usize, message: &[u8], index: usize) -> Option<usize> {
    // Convenience closures help shorten the expressions in the match block.
    // The compiler usually inlines short closures so these should have no effect on performance.
    let apply = |a| check(rules, a, message, index);
    let sequence = |a, b| apply(a).and_then(|next| check(rules, b, message, next));

    match rules[rule] {
        Letter(l) => (index < message.len() && message[index] == l).then_some(index + 1),
        Follow(a) => apply(a),
        Choice(a, b) => apply(a).or_else(|| apply(b)),
        Sequence(a, b) => sequence(a, b),
        Compound(a, b, c, d) => sequence(a, b).or_else(|| sequence(c, d)),
    }
}
