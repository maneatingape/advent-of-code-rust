//! # The Halting Problem
//!
//! The input is parsed into a 2 dimensional array covering each possible combination of state
//! and tape value at the cursor. Each transition is then computed via a lookup into this array.
//!
//! To speed things up by about ten times, multiple transitions are then precomputed to allow
//! skipping forward multiple steps at a time.
//!
//! For each of the 6 * 256 = 1536 combinations of a tape with 4 values to the left and 3 values
//! to the right of the cursor, the turing machine is computed one steps at a time until the cursor
//! leaves the area to the left or to the right. For example:
//!
//! ```none
//!     State: A            => State: B
//!       1 0 1 0 [1] 0 0 1     [0] 0 1 0 0 1 1 0
//!
//!     State: B
//!       t t t t [0] 0 1 0
//! ```
//!
//! In this example the tape then advances four bits to the left, loading the four values of `t`
//! then the next batch lookup is performed.
use crate::util::parse::*;
use std::array::from_fn;

pub struct Input {
    state: usize,
    steps: u32,
    rules: Vec<[Rule; 2]>,
}

struct Rule {
    next_state: usize,
    next_tape: usize,
    advance: bool,
}

impl Rule {
    fn parse(block: &[&[u8]]) -> Self {
        let next_tape = (block[0][22] - b'0') as usize;
        let advance = block[1][27] == b'r';
        let next_state = (block[2][26] - b'A') as usize;
        Rule { next_state, next_tape, advance }
    }
}

struct Skip {
    next_state: usize,
    next_tape: usize,
    steps: u32,
    ones: i32,
    advance: bool,
}

/// Parse the input into 12 rules for each possible combination of state and value at the cursor.
pub fn parse(input: &str) -> Input {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();

    let state = (lines[0][15] - b'A') as usize;
    let steps = input.unsigned();
    let rules: Vec<_> = lines[3..]
        .chunks(10)
        .map(|chunk| [Rule::parse(&chunk[2..5]), Rule::parse(&chunk[6..9])])
        .collect();

    Input { state, steps, rules }
}

pub fn part1(input: &Input) -> i32 {
    // Precompute state transitions in larger amount in order to skip forward several transitions
    // at a time. 100 max_steps is a safety valve in case some state transitions do not halt
    // although this is unlikely for the inputs that are provided.
    let table: Vec<[Skip; 256]> = (0..input.rules.len())
        .map(|state| from_fn(|tape| turing(&input.rules, state, tape, 100)))
        .collect();

    let mut state = input.state;
    let mut remaining = input.steps;
    let mut tape = 0;
    let mut checksum = 0;
    let mut left = Vec::new();
    let mut right = Vec::new();

    loop {
        // Lookup the next batch state transition.
        let Skip { next_state, next_tape, steps, ones, advance } = table[state][tape];

        // Handle any remaining transitions less than the batch size one step at a time.
        if steps > remaining {
            let Skip { ones, .. } = turing(&input.rules, state, tape, remaining);
            break checksum + ones;
        }

        state = next_state;
        tape = next_tape;
        remaining -= steps;
        checksum += ones;

        // Use a vector to simulate an empty tape. In practise the cursor doesn't move more than
        // a few thousand steps in any direction, so this approach is as fast as a fixed size
        // array, but much more robust.
        if advance {
            left.push(tape & 0xf0);
            tape = ((tape & 0xf) << 4) | right.pop().unwrap_or(0);
        } else {
            right.push(tape & 0xf);
            tape = (tape >> 4) | left.pop().unwrap_or(0);
        }
    }
}

pub fn part2(_input: &Input) -> &'static str {
    "n/a"
}

// Precompute state transitions up to some maximum value of steps.
fn turing(rules: &[[Rule; 2]], mut state: usize, mut tape: usize, max_steps: u32) -> Skip {
    let mut mask = 0b00001000;
    let mut steps = 0;
    let mut ones = 0;

    // `0`` means the cursor has advanced to the next nibble on the right.
    // `128` means that the cursor is on the left edge of the high nibble.
    while 0 < mask && mask < 128 && steps < max_steps {
        let current = ((tape & mask) != 0) as usize;
        let Rule { next_state, next_tape, advance } = rules[state][current];

        if next_tape == 1 {
            tape |= mask;
        } else {
            tape &= !mask;
        }

        if advance {
            mask >>= 1;
        } else {
            mask <<= 1;
        }

        state = next_state;
        steps += 1;
        // Count the total numbers of ones by summing the number of ones written minus the
        // number of ones erased.
        ones += next_tape as i32 - current as i32;
    }

    Skip { next_state: state, next_tape: tape, steps, ones, advance: mask == 0 }
}
