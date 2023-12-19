//! # Aplenty
//!
//! Each rule is converted into a half open interval, including the start but excluding the end.
//! For example:
//!
//! * `x > 10` => `10..4001`
//! * `m < 20` => `1..20`
//! * `A` => `1..4001`
//!
//! For part one if a category is contained in a range, we send the part to the next rule,
//! stopping when `A` or `R` is reached.
//!
//! For part two we perform range splitting similar to [`Day 5`] that converts the category into
//! 1, 2 or 3 new ranges, then sends those ranges to the respective rule.
//!
//! [`Day 5`]: crate::year2023::day05
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;

pub struct Rule<'a> {
    start: u32,
    end: u32,
    category: usize,
    next: &'a str,
}

pub struct Input<'a> {
    workflows: FastMap<&'a str, Vec<Rule<'a>>>,
    parts: &'a str,
}

/// Parse each rule from the first half of the input.
/// Leaves the second half of the input as a `&str` as it's faster to iterate over each chunk of
/// four numbers than to first collect into a `vec`.
pub fn parse(input: &str) -> Input<'_> {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let mut workflows = FastMap::with_capacity(1000);

    for line in prefix.lines() {
        let mut rules = Vec::with_capacity(5);
        let mut iter = line.split(['{', ':', ',', '}']);
        let key = iter.next().unwrap();

        for [first, second] in iter.chunk::<2>() {
            let rule = if second.is_empty() {
                // The last rule will match everything so pick category 0 arbitrarily.
                Rule { start: 1, end: 4001, category: 0, next: first }
            } else {
                // Map each category to an index for convenience so that we can store a part
                // in a fixed size array.
                let category = match first.as_bytes()[0] {
                    b'x' => 0,
                    b'm' => 1,
                    b'a' => 2,
                    b's' => 3,
                    _ => unreachable!(),
                };

                let value: u32 = (&first[2..]).unsigned();
                let next = second;

                // Convert each rule into a half open range.
                match first.as_bytes()[1] {
                    b'<' => Rule { start: 1, end: value, category, next },
                    b'>' => Rule { start: value + 1, end: 4001, category, next },
                    _ => unreachable!(),
                }
            };

            rules.push(rule);
        }

        workflows.insert(key, rules);
    }

    Input { workflows, parts: suffix }
}

pub fn part1(input: &Input<'_>) -> u32 {
    let Input { workflows, parts } = input;
    let mut result = 0;

    // We only care about the numbers and can ignore all delimeters and whitespace.
    for part in parts.iter_unsigned::<u32>().chunk::<4>() {
        let mut key = "in";

        while key.len() > 1 {
            // Find the first matching rule.
            for &Rule { start, end, category, next } in &workflows[key] {
                if start <= part[category] && part[category] < end {
                    key = next;
                    break;
                }
            }
        }

        if key == "A" {
            result += part.iter().sum::<u32>();
        }
    }

    result
}

pub fn part2(input: &Input<'_>) -> u64 {
    let Input { workflows, .. } = input;
    let mut result = 0;
    let mut todo = vec![("in", 0, [(1, 4001); 4])];

    while let Some((key, index, mut part)) = todo.pop() {
        if key.len() == 1 {
            if key == "A" {
                result += part.iter().map(|(s, e)| (e - s) as u64).product::<u64>();
            }
            continue;
        }

        let Rule { start: s2, end: e2, category, next } = workflows[key][index];
        let (s1, e1) = part[category];

        // x1 and x2 are the possible overlap.
        let x1 = s1.max(s2);
        let x2 = e1.min(e2);

        if x1 >= x2 {
            // No overlap. Check the next rating.
            todo.push((key, index + 1, part));
        } else {
            // Range that overlaps with the rating.
            part[category] = (x1, x2);
            todo.push((next, 0, part));

            // Range before rating.
            if s1 < x1 {
                part[category] = (s1, x1);
                todo.push((key, index + 1, part));
            }

            // Range after rating.
            if x2 < e1 {
                part[category] = (x2, e1);
                todo.push((key, index + 1, part));
            }
        }
    }

    result
}
