//! # The Halting Problem
//!
//! The input is parsed into a 2-dimensional array covering each possible combination of state
//! and tape value at the cursor. Each transition is then computed via a lookup into this array.
//!
//! To speed things up by about ten times, multiple transitions are then precomputed to allow
//! skipping forward multiple steps at a time. Blocks 128 cells wide are cached once the cursor
//! moves off either end.
//!
//! Interestingly the total number of distinct cached blocks is very low, approximately 200.
//! The cursor also doesn't move too far, only covering a range of about 6,000 steps.
use crate::util::hash::*;
use crate::util::parse::*;

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
    next_tape: u128,
    steps: u32,
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

pub fn part1(input: &Input) -> u32 {
    let mut state = input.state;
    let mut remaining = input.steps;
    let mut tape = 0;
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut cache = FastMap::new();

    loop {
        // Lookup the next batch state transition.
        let Skip { next_state, next_tape, steps, advance } = *cache
            .entry((state, tape))
            .or_insert_with(|| turing(&input.rules, state, tape, u32::MAX));

        // Handle any remaining transitions less than the batch size one step at a time.
        if steps > remaining {
            let Skip { next_tape, .. } = turing(&input.rules, state, tape, remaining);
            tape = next_tape;
            break;
        }

        state = next_state;
        tape = next_tape;
        remaining -= steps;

        // Use a vector to simulate an empty tape. In practice the cursor doesn't move more than
        // a few thousand steps in any direction, so this approach is as fast as a fixed size
        // array, but much more robust.
        if advance {
            left.push(tape & 0xffffffffffffffff0000000000000000);
            tape = (tape << 64) | right.pop().unwrap_or(0);
        } else {
            right.push(tape & 0x0000000000000000ffffffffffffffff);
            tape = (tape >> 64) | left.pop().unwrap_or(0);
        }
    }

    tape.count_ones()
        + left.iter().map(|&n| n.count_ones()).sum::<u32>()
        + right.iter().map(|&n| n.count_ones()).sum::<u32>()
}

pub fn part2(_input: &Input) -> &'static str {
    "n/a"
}

// Precompute state transitions up to some maximum value of steps.
fn turing(rules: &[[Rule; 2]], mut state: usize, mut tape: u128, max_steps: u32) -> Skip {
    let mut mask = 1 << 63;
    let mut steps = 0;

    // `0` means the cursor has advanced to the next nibble on the right.
    // `128` means that the cursor is on the left edge of the high nibble.
    while 0 < mask && mask < (1 << 127) && steps < max_steps {
        let current = usize::from(tape & mask != 0);
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
    }

    Skip { next_state: state, next_tape: tape, steps, advance: mask == 0 }
}
