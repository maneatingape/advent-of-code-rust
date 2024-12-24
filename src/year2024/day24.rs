//! # Crossed Wires
use crate::util::hash::*;
use crate::util::iter::*;
use crate::util::parse::*;

type Input<'a> = (&'a str, Vec<[&'a str; 5]>);

pub fn parse(input: &str) -> Input<'_> {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let gates = suffix.split_ascii_whitespace().chunk::<5>().collect();
    (prefix, gates)
}

pub fn part1(input: &Input<'_>) -> u64 {
    let (prefix, gates) = input;

    let mut names = FastMap::new();
    let mut cache = FastMap::new();
    let mut ops = FastMap::new();

    for line in prefix.lines() {
        let prefix = &line[..3];
        let suffix = &line[5..];

        let size = names.len();
        let index = *names.entry(prefix).or_insert(size);

        cache.insert(index, suffix.unsigned());
    }

    for &[left, kind, right, _, to] in gates {
        let size = names.len();
        let left = *names.entry(left).or_insert(size);

        let size = names.len();
        let right = *names.entry(right).or_insert(size);

        let size = names.len();
        let to = *names.entry(to).or_insert(size);

        ops.insert(to, (left, kind, right));
    }

    let mut result = 0;

    for i in (0..64).rev() {
        let key = format!("z{i:02}");
        if let Some(&key) = names.get(key.as_str()) {
            result = (result << 1) | helper(&mut cache, &mut ops, key);
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

fn helper(
    cache: &mut FastMap<usize, u64>,
    ops: &mut FastMap<usize, (usize, &str, usize)>,
    key: usize,
) -> u64 {
    if let Some(&value) = cache.get(&key) {
        return value;
    }

    let (left, op, right) = ops[&key];
    let left = helper(cache, ops, left);
    let right = helper(cache, ops, right);

    let value = match op {
        "AND" => left & right,
        "OR" => left | right,
        "XOR" => left ^ right,
        _ => unreachable!(),
    };

    cache.insert(key, value);
    value
}
