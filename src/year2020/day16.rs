//! # Ticket Translation
//!
//! Part one is optimized by first merging as many of the rules as possible. The trick to merge
//! ranges efficiently is to first sort them by start, then combine any that start before the end
//! of the previous range. For my input this cut down the checks for each number from 40 to 1.
//! The invalid rows are saved and passed to part two.
//!
//! Part two is a [constraint satisfaction problem](https://en.wikipedia.org/wiki/Constraint_satisfaction_problem).
//! First we transpose the ticket rows to columns, grouping each number in the same position in the
//! ticket. For each column we check every number, eliminating rules that don't fit, since
//! we know that potential rules must be valid for every field in the same position.
//!
//! To solve this, there must be at least one column with only one rule remaining. As this rule can
//! only belong to this column, we eliminate it from other columns. This causes a chain reaction
//! where a second column will reduce to only one rule, continuing until all columns have been
//! resolved.
use crate::util::iter::*;
use crate::util::parse::*;

type Result = (u32, u64);
type Ticket = Vec<u32>;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Rule {
    departure: bool,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl Rule {
    fn from(line: &str) -> Rule {
        let departure = line.starts_with("departure");
        let [a, b, c, d] = line.iter_unsigned().chunk::<4>().next().unwrap();
        Rule { departure, a, b, c, d }
    }

    fn check(&self, n: u32) -> bool {
        (self.a <= n && n <= self.b) || (self.c <= n && n <= self.d)
    }
}

pub fn parse(input: &str) -> Result {
    let [first, second, third] = input.splitn(3, "\n\n").chunk::<3>().next().unwrap();
    let rules: Vec<_> = first.lines().map(Rule::from).collect();
    let your_ticket: Ticket = second.iter_unsigned().collect();
    let mut nearby_tickets = vec![Vec::new(); rules.len()];

    for (i, n) in third.iter_unsigned().enumerate() {
        nearby_tickets[i % rules.len()].push(n);
    }

    let (error_rate, valid) = solve_part_one(&rules, &nearby_tickets);
    let product = solve_part_two(&rules, &your_ticket, &nearby_tickets, &valid);

    (error_rate, product)
}

pub fn part1(input: &Result) -> u32 {
    input.0
}

pub fn part2(input: &Result) -> u64 {
    input.1
}

fn solve_part_one(rules: &[Rule], tickets: &[Ticket]) -> (u32, Vec<bool>) {
    let mut ranges: Vec<_> = rules.iter().flat_map(|r| [r.a..r.b + 1, r.c..r.d + 1]).collect();
    ranges.sort_unstable_by_key(|r| r.start);
    ranges.dedup_by(|next, prev| {
        if next.start <= prev.end {
            prev.end = prev.end.max(next.end);
            true
        } else {
            false
        }
    });

    let mut total = 0;
    let mut valid = vec![true; tickets[0].len()];

    for column in tickets {
        for (i, n) in column.iter().enumerate() {
            let check = ranges.iter().any(|range| range.contains(n));
            if !check {
                total += n;
                valid[i] = false;
            }
        }
    }

    (total, valid)
}

fn solve_part_two(
    rules: &[Rule],
    your_ticket: &Ticket,
    nearby_tickets: &[Ticket],
    valid: &[bool],
) -> u64 {
    let mut rules_by_column = Vec::new();
    let mut product = 1;

    for ticket in nearby_tickets {
        let mut remaining = rules.to_vec();

        for (&valid, &n) in valid.iter().zip(ticket.iter()) {
            if valid {
                remaining.retain(|rule| rule.check(n));
            }
        }

        rules_by_column.push(remaining);
    }

    while rules_by_column.iter().any(|rules| rules.len() > 1) {
        for i in 0..rules_by_column.len() {
            if rules_by_column[i].len() == 1 {
                let found = rules_by_column[i].pop().unwrap();

                if found.departure {
                    product *= your_ticket[i] as u64;
                }

                for remaining in &mut rules_by_column {
                    remaining.retain(|&rule| rule != found);
                }
            }
        }
    }

    product
}
