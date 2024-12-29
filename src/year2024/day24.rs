//! # Crossed Wires
//!
//! Part one is a straightforward simulation of the gates. Part two asks us to fix a broken
//! [ripple carry adder](https://en.wikipedia.org/wiki/Adder_(electronics)).
//!
//! The structure of the adder is:
//!
//! * Half adder for bits `x00` and `y00`. Outputs sum to `z00` and carry to `z01`.
//! * Full adder for bits `x01..x44` and `y01..y44`. Outputs carry to next bit in the chain
//!   "rippling" up to final bit.
//! * `z45` is the carry output from `x44` and `y44`.
//!
//! Implemented in logic gates this looks like:
//!
//! ```none
//!    Half Adder     Full Adder
//!    ┌───┐ ┌───┐    ┌───┐ ┌───┐
//!    |x00| |y00|    |x01| |y01|
//!    └───┘ └───┘    └───┘ └───┘
//!     | | ┌─┘ |      | | ┌─┘ |
//!     | └───┐ |      | └───┐ |
//!     | ┌-┘ | |      | ┌-┘ | |
//!    ┌───┐ ┌───┐    ┌───┐ ┌───┐
//!    |XOR| |AND|    |XOR| |AND|
//!    └───┘ └───┘    └───┘ └───┘
//!      |     |    ┌───┴┐     |
//!      |     └──┬────┐ |     |
//!      |   Carry| | ┌───┐    |
//!      |    out | | |AND|    |
//!      |        | | └───┘    |
//!      |        | |   └────┐ |
//!      |        | └────┐   | |
//!      |        └────┐ |   | |
//!      |            ┌───┐ ┌───┐
//!      |            |XOR| |OR |                                  Carry
//!      |            └───┘ └───┘                                   out
//!      |              |     |                                      |
//!    ┌───┐          ┌───┐   |                                    ┌───┐
//!    |z00|          |z01| Carry    ...repeat for z01 to z44...   |z45|
//!    └───┘          └───┘  out                                   └───┘
//! ```
//!
//! Then we can deduce some rules for the output of each gate type:
//!
//! 1. **XOR** If inputs are `x` and `y` then output must be another XOR gate
//!    (except for inputs `x00` and `y00`) otherwise output must be `z`.
//! 2. **AND** Output must be an OR gate (except for inputs `x00` and `y00`).
//! 3. **OR** Output must be both AND and XOR gate, except for final carry
//!    which must output to `z45`.
//!
//! We only need to find swapped outputs (not fix them) so the result is the labels of gates
//! that breaks the rules in alphabetical order.
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

    // Using an array to store already computed values is much faster than a `HashMap`.
    let mut todo: VecDeque<_> = gates.iter().copied().collect();
    let mut cache = vec![u8::MAX; 1 << 15];
    let mut result = 0;

    // Convert each character to a 5 bit number from 0..31
    // then each group of 3 to a 15 bit index from 0..32768.
    let to_index = |s: &str| {
        let b = s.as_bytes();
        ((b[0] as usize & 31) << 10) + ((b[1] as usize & 31) << 5) + (b[2] as usize & 31)
    };

    // Add input signals to cache.
    for line in prefix.lines() {
        let prefix = &line[..3];
        let suffix = &line[5..];
        cache[to_index(prefix)] = suffix.unsigned();
    }

    // If both inputs are available then add gate output to cache
    // otherwise push back to end of queue for reprocessing later.
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

    // Output 46 bit result.
    for i in (to_index("z00")..to_index("z46")).rev() {
        if cache[i] != u8::MAX {
            result = (result << 1) | (cache[i] as u64);
        }
    }

    result
}

pub fn part2(input: &Input<'_>) -> String {
    let (_, gates) = input;

    let mut output = FastSet::new();
    let mut swapped = FastSet::new();

    // Track the kind of gate that each wire label outputs to.
    for &[left, kind, right, _, _] in gates {
        output.insert((left, kind));
        output.insert((right, kind));
    }

    for &[left, kind, right, _, to] in gates {
        match kind {
            "AND" => {
                // Check that all AND gates point to an OR, except for first AND.
                if left != "x00" && right != "x00" && !output.contains(&(to, "OR")) {
                    swapped.insert(to);
                }
            }
            "OR" => {
                // Check that only XOR gates point to output, except for last carry which is OR.
                if to.starts_with('z') && to != "z45" {
                    swapped.insert(to);
                }
                // OR can never point to OR.
                if output.contains(&(to, "OR")) {
                    swapped.insert(to);
                }
            }
            "XOR" => {
                if left.starts_with('x') || right.starts_with('x') {
                    // Check that first level XOR points to second level XOR, except for first XOR.
                    if left != "x00" && right != "x00" && !output.contains(&(to, "XOR")) {
                        swapped.insert(to);
                    }
                } else {
                    // Second level XOR must point to output.
                    if !to.starts_with('z') {
                        swapped.insert(to);
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    let mut result: Vec<_> = swapped.into_iter().collect();
    result.sort_unstable();
    result.join(",")
}
