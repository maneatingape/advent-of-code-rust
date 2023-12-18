use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;
use Rule::*;

#[derive(Clone, Copy, Debug)]
enum Rule<'a> {
    Send(&'a str),
    Less(usize, u32, &'a str),
    Greater(usize, u32, &'a str),
    Equal(usize, u32, &'a str),
}

pub struct Input<'a> {
    workflows: FastMap<&'a str, Vec<Rule<'a>>>,
    parts: &'a str,
}

pub fn parse(input: &str) -> Input<'_> {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let mut workflows = FastMap::with_capacity(1000);

    for line in prefix.lines() {
        let mut rules = Vec::with_capacity(5);
        let mut iter = line.split(['{', ':', ',', '}']);
        let key = iter.next().unwrap();

        for [first, second] in iter.chunk::<2>() {
            let rule = if second.is_empty() {
                Send(first)
            } else {
                let category = match first.as_bytes()[0] {
                    b'x' => 0,
                    b'm' => 1,
                    b'a' => 2,
                    b's' => 3,
                    _ => unreachable!(),
                };

                let value: u32 = (&first[2..]).unsigned();

                match first.as_bytes()[1] {
                    b'<' => Less(category, value, second),
                    b'>' => Greater(category, value, second),
                    b'=' => Equal(category, value, second),
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

    for part in parts.iter_unsigned::<u32>().chunk::<4>() {
        let mut key = "in";

        loop {
            if key == "A" {
                result += part.iter().sum::<u32>();
                break;
            }
            if key == "R" {
                break;
            }

            for &rule in &workflows[key] {
                match rule {
                    Send(next) => key = next,
                    Less(category, value, next) => {
                        if part[category] < value {
                            key = next;
                            break;
                        }
                    }
                    Greater(category, value, next) => {
                        if part[category] > value {
                            key = next;
                            break;
                        }
                    }
                    Equal(category, value, next) => {
                        if part[category] == value {
                            key = next;
                            break;
                        }
                    }
                }
            }
        }
    }

    result
}

pub fn part2(input: &Input<'_>) -> u64 {
    let Input { workflows, .. } = input;
    let mut result = 0;
    let mut todo = vec![("in", 0, [(1, 4000); 4])];

    while let Some((key, index, mut part)) = todo.pop() {
        if key == "A" {
            result += part.iter().map(|(s, e)| (e - s + 1) as u64).product::<u64>();
            continue;
        }
        if key == "R" {
            continue;
        }

        match workflows[key][index] {
            Send(next) => todo.push((next, 0, part)),
            Less(category, value, next) => {
                let (start, end) = part[category];

                if start >= value {
                    todo.push((key, index + 1, part));
                } else if end < value {
                    todo.push((next, 0, part));
                } else {
                    part[category] = (start, value - 1);
                    todo.push((next, 0, part));

                    part[category] = (value, end);
                    todo.push((key, index + 1, part));
                }
            }
            Greater(category, value, next) => {
                let (start, end) = part[category];

                if end <= value {
                    todo.push((key, index + 1, part));
                } else if start > value {
                    todo.push((next, 0, part));
                } else {
                    part[category] = (start, value);
                    todo.push((key, index + 1, part));

                    part[category] = (value + 1, end);
                    todo.push((next, 0, part));
                }
            }
            Equal(category, value, next) => {
                let (start, end) = part[category];

                if start > value || end < value {
                    todo.push((key, index + 1, part));
                } else {
                    part[category] = (start, value - 1);
                    todo.push((key, index + 1, part));

                    part[category] = (value, value);
                    todo.push((next, 0, part));

                    part[category] = (value + 1, end);
                    todo.push((key, index + 1, part));
                }
            }
        }
    }

    result
}
