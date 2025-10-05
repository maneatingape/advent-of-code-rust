//! # Supply Stacks
use crate::util::iter::*;
use crate::util::parse::*;

type Stack = Vec<Vec<char>>;
type Move = [usize; 3];
type Input = (Stack, Vec<Move>);

/// Parses the input in 2 stages.
///
/// First, the input is split into a prefix and suffix, using a blank line (or 2 newline characters
/// one after another) as the delimiter.
///
/// The suffix consisting of triplets of (amount, from, to) can be parsed using our utility
/// [`iter_unsigned`] and [`chunk`] methods to tokenize the string into numbers, then group it into
/// triples. One minor nuance is that the `from` and `to` field are *1* based indexing, so we
/// convert them to 0 based for convenience.
///
/// The prefix is a little more complex. The number of columns is the width in characters plus 1
/// divided by 4 (the last column has no trailing space). Then we build the vectors from the bottom
/// up by iterating through the rows in reverse. This places the elements at the top of each stack
/// at the end of the `vec` which is a more natural location for mutation (as removing elements from
/// the start of a `vec` involved moving all remaining elements).
///
/// [`iter_unsigned`]: ParseOps::iter_unsigned
/// [`chunk`]: ChunkOps::chunk
pub fn parse(input: &str) -> Input {
    let (prefix, suffix) = input.split_once("\n\n").unwrap();
    let lines: Vec<_> = prefix.lines().collect();
    let width = (lines[0].len() + 1) / 4;

    let mut stack: Stack = vec![Vec::new(); width];
    for row in lines.iter().rev().skip(1) {
        for (i, c) in row.chars().skip(1).step_by(4).enumerate() {
            if c.is_ascii_alphabetic() {
                stack[i].push(c);
            }
        }
    }

    let moves: Vec<_> = suffix
        .iter_unsigned()
        .chunk::<3>()
        .map(|[amount, from, to]| [amount, from - 1, to - 1])
        .collect();

    (stack, moves)
}

/// Move elements from stack to stack, reversing each time.
pub fn part1(input: &Input) -> String {
    play(input, true)
}

/// Move elements from stack to stack without reversing.
pub fn part2(input: &Input) -> String {
    play(input, false)
}

/// `get_disjoint_mut` allows us to acquire two simultaneous mutable references to disjoint indices.
/// A nice standard library feature is that we can collect an iterator of `char`s into a `String`
/// for the final answer.
fn play(input: &Input, reverse: bool) -> String {
    let (initial, moves) = input;
    let mut stack = initial.clone();

    for &[amount, from, to] in moves {
        let [from, to] = stack.get_disjoint_mut([from, to]).unwrap();
        let start = from.len() - amount;
        let iter = from.drain(start..);

        if reverse {
            to.extend(iter.rev());
        } else {
            to.extend(iter);
        }
    }

    stack.iter().filter_map(|v| v.last()).collect()
}
