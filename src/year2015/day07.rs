//! # Some Assembly Required
//!
//! To obtain the result we recursively compute the inputs starting at gate `a` and working
//! backwards. To make things faster we memoize the result of each wire in a cache, so that each
//! wire is computed at most once.
//!
//! For part two we pre-seed the value of `b` in the cache with the result from part one then
//! re-run the same process.
use crate::util::hash::*;
use crate::util::parse::*;

type Result = (u16, u16);

enum Gate<'a> {
    Wire(&'a str),
    Not(&'a str),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    LeftShift(&'a str, u16),
    RightShift(&'a str, u16),
}

pub fn parse(input: &str) -> Result {
    let mut tokens = input.split_ascii_whitespace();
    let mut circuit = FastMap::new();

    while let (Some(first), Some(second)) = (tokens.next(), tokens.next()) {
        let gate = if first == "NOT" {
            let _third = tokens.next().unwrap();
            Gate::Not(second)
        } else if second == "->" {
            Gate::Wire(first)
        } else {
            let third = tokens.next().unwrap();
            let _fourth = tokens.next().unwrap();

            match second {
                "AND" => Gate::And(first, third),
                "OR" => Gate::Or(first, third),
                "LSHIFT" => Gate::LeftShift(first, third.unsigned()),
                "RSHIFT" => Gate::RightShift(first, third.unsigned()),
                _ => unreachable!(),
            }
        };

        let wire = tokens.next().unwrap();
        circuit.insert(wire, gate);
    }

    let mut cache = FastMap::new();
    let result1 = signal("a", &circuit, &mut cache);

    cache.clear();
    cache.insert("b", result1);
    let result2 = signal("a", &circuit, &mut cache);

    (result1, result2)
}

fn signal<'a>(
    key: &'a str,
    circuit: &FastMap<&'a str, Gate<'a>>,
    cache: &mut FastMap<&'a str, u16>,
) -> u16 {
    if let Some(result) = cache.get(key) {
        return *result;
    }

    let result = if key.as_bytes()[0].is_ascii_digit() {
        key.unsigned()
    } else {
        match circuit[key] {
            Gate::Wire(w) => signal(w, circuit, cache),
            Gate::Not(w) => !signal(w, circuit, cache),
            Gate::And(l, r) => signal(l, circuit, cache) & signal(r, circuit, cache),
            Gate::Or(l, r) => signal(l, circuit, cache) | signal(r, circuit, cache),
            Gate::LeftShift(w, n) => signal(w, circuit, cache) << n,
            Gate::RightShift(w, n) => signal(w, circuit, cache) >> n,
        }
    };

    cache.insert(key, result);
    result
}

pub fn part1(input: &Result) -> u16 {
    input.0
}

pub fn part2(input: &Result) -> u16 {
    input.1
}
