//! # Some Assembly Required
//!
//! To obtain the result we recursively compute the inputs starting at gate `a` and working
//! backward. To make things faster we memoize the result of each wire in a cache, so that each
//! wire is computed at most once.
//!
//! For part two we pre-seed the value of `b` in the cache with the result from part one then
//! re-run the same process.
use crate::util::parse::*;
use Gate::*;

type Input = (u32, u32);

#[derive(Clone, Copy)]
enum Gate {
    Constant(u32),
    Wire(usize),
    Not(usize),
    Bit(usize),
    And(usize, usize),
    Or(usize, usize),
    LeftShift(usize, u32),
    RightShift(usize, u32),
}

pub fn parse(input: &str) -> Input {
    let mut tokens = input.split_ascii_whitespace();
    let mut circuit = vec![Constant(0); 729];

    while let (Some(first), Some(second)) = (tokens.next(), tokens.next()) {
        let gate = if first == "NOT" {
            let _third = tokens.next().unwrap();
            Not(to_index(second))
        } else if second == "->" {
            if first.as_bytes()[0].is_ascii_digit() {
                Constant(first.unsigned())
            } else {
                Wire(to_index(first))
            }
        } else {
            let third = tokens.next().unwrap();
            let _fourth = tokens.next().unwrap();

            match second {
                "AND" if first == "1" => Bit(to_index(third)),
                "AND" => And(to_index(first), to_index(third)),
                "OR" => Or(to_index(first), to_index(third)),
                "LSHIFT" => LeftShift(to_index(first), third.unsigned()),
                "RSHIFT" => RightShift(to_index(first), third.unsigned()),
                _ => unreachable!(),
            }
        };

        let wire = tokens.next().unwrap();
        circuit[to_index(wire)] = gate;
    }

    let mut cache = vec![u32::MAX; 729];
    let part_one = signal(to_index("a"), &circuit, &mut cache);

    cache.fill(u32::MAX);
    cache[to_index("b")] = part_one;
    let part_two = signal(to_index("a"), &circuit, &mut cache);

    (part_one, part_two)
}

fn signal(key: usize, circuit: &[Gate], cache: &mut [u32]) -> u32 {
    if cache[key] != u32::MAX {
        return cache[key];
    }

    let result = match circuit[key] {
        Constant(c) => c,
        Wire(w) => signal(w, circuit, cache),
        Not(w) => 0xffff & !signal(w, circuit, cache),
        Bit(r) => 1 & signal(r, circuit, cache),
        And(l, r) => signal(l, circuit, cache) & signal(r, circuit, cache),
        Or(l, r) => signal(l, circuit, cache) | signal(r, circuit, cache),
        LeftShift(w, n) => 0xffff & (signal(w, circuit, cache) << n),
        RightShift(w, n) => signal(w, circuit, cache) >> n,
    };

    cache[key] = result;
    result
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}

/// Convert one or two character string to an index.
fn to_index(s: &str) -> usize {
    s.bytes().fold(0, |acc, b| 27 * acc + usize::from(b - b'a' + 1))
}
