//! # Crossed Wires
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;
use std::collections::VecDeque;

type Input<'a> = (&'a str, Vec<[&'a str; 5]>);

pub fn parse(input: &str) -> Input<'_> {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let gates = suffix.split_ascii_whitespace().chunk::<5>().collect();
    (prefix, gates)
}

pub fn part1(input: &Input<'_>) -> u64 {
    let (prefix, gates) = input;

    let mut todo: VecDeque<_> = gates.iter().copied().collect();
    let mut cache = vec![u8::MAX; 1 << 15];
    let mut result = 0;

    let to_index = |s: &str| {
        let b = s.as_bytes();
        ((b[0] as usize & 31) << 10) + ((b[1] as usize & 31) << 5) + (b[2] as usize & 31)
    };

    for line in prefix.lines() {
        let prefix = &line[..3];
        let suffix = &line[5..];
        cache[to_index(prefix)] = suffix.unsigned();
    }

    while let Some(gate @ [left, kind, right, _, to]) = todo.pop_front() {
        let left = cache[to_index(left)];
        let right = cache[to_index(right)];

        if left == u8::MAX || right == u8::MAX {
            todo.push_back(gate);
        } else {
            cache[to_index(to)] = match kind {
                "AND" => left & right,
                "OR" => left | right,
                "XOR" => left ^ right,
                _ => unreachable!(),
            }
        }
    }

    for i in (to_index("z00")..to_index("z64")).rev() {
        if cache[i] != u8::MAX {
            result = (result << 1) | (cache[i] as u64);
        }
    }

    result
}

pub fn part2(input: &Input<'_>) -> String {
    let (_, gates) = input;

    let mut lookup = FastSet::new();
    let mut swapped = FastSet::new();

    for &[left, kind, right, _, _] in gates {
        lookup.insert((left, kind));
        lookup.insert((right, kind));
    }

    for &[left, kind, right, _, to] in gates {
        if kind == "AND" {
            // Check that all AND gates point to an OR, except for first AND.
            if left != "x00" && right != "x00" && !lookup.contains(&(to, "OR")) {
                swapped.insert(to);
            }
        }

        if kind == "OR" {
            // Check that only XOR gates point to output, except for last carry which is OR.
            if to.starts_with('z') && to != "z45" {
                swapped.insert(to);
            }
            // OR can never point to OR.
            if lookup.contains(&(to, "OR")) {
                swapped.insert(to);
            }
        }

        if kind == "XOR" {
            if left.starts_with('x') || right.starts_with('x') {
                // Check that first level XOR points to second level XOR, except for first XOR.
                if left != "x00" && right != "x00" && !lookup.contains(&(to, "XOR")) {
                    swapped.insert(to);
                }
            } else {
                // Second level XOR must point to output.
                if !to.starts_with('z') {
                    swapped.insert(to);
                }
            }
        }
    }

    let mut result: Vec<_> = swapped.into_iter().collect();
    result.sort_unstable();
    result.join(",")
}
