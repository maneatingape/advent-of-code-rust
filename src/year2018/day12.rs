//! # Subterranean Sustainability
//!
//! The problem is a one dimensional version of
//! [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).
//!
//! We use a bit vector to store which pots are occupied and which are empty in a generation. The
//! left-most pot is represented by the least-significant bit. To simplify extracting bit patterns,
//! we always leave the first four bits empty. For example, the pattern `#..#.#..##......###...###`
//! becomes `11100011100000011001010010000`. Also, after each step, we truncate the bit vector on
//! the left and right. This makes it easier to compare generations in part two.
//!
//! The trick for part two is that the plants will eventually stabilize into a repeating pattern
//! that expands by the same amount each generation. Once two subsequent generations are the same,
//! we extrapolate 50 billion generations into the future.
use std::mem::swap;

use crate::util::bitset::BitOps as _;

type Input = (i64, i64);

pub fn parse(input: &str) -> Input {
    let mut lines = input.lines().map(str::as_bytes);

    // Parse initial state
    let initial_state = lines.next().unwrap();
    let initial_state = &initial_state[15..];
    let mut pots = Pots::from(initial_state);

    // Parse rules into a table with all possible 2⁵=32 patterns
    lines.next();
    let mut rules = [false; 32];
    for line in lines {
        let from = &line[0..5];
        let to = line[9];
        if to == b'#' {
            let mut p = 0_usize;
            for (i, &b) in from.iter().enumerate() {
                if b == b'#' {
                    p |= 1 << i;
                }
            }
            rules[p] = true;
        }
    }

    // Part 1 - Simulate the first 20 steps
    let mut steps = 0_i64;
    while steps < 20 {
        pots.step(&rules);
        steps += 1;
    }
    let total1 = pots.sum();

    // Part 2 - Only simulate until the generation repeats
    let mut prev_pos = 0;
    while steps < 50_000_000_000 {
        prev_pos = pots.pos;
        pots.step(&rules);
        steps += 1;
        if pots.state == pots.prev_state {
            // Generation has repeated
            break;
        }
    }

    // Extrapolate to 50 billion steps
    pots.pos += (pots.pos - prev_pos) * (50_000_000_000 - steps);
    let total2 = pots.sum();

    // Return answer
    (total1, total2)
}

pub fn part1(input: &Input) -> i64 {
    input.0
}

pub fn part2(input: &Input) -> i64 {
    input.1
}

/// Compute the length of the given bit vector
fn get_len(state: &[u64]) -> usize {
    let l = state.len() - 1;
    l * 64 + (64 - state[l].leading_zeros() as usize)
}

#[derive(Clone)]
struct Pots {
    /// A bit vector representing the pots. 1 means there is a plant in the pot, 0 means there
    /// isn't.
    state: Vec<u64>,

    /// A copy of the bit vector `state` before [`Self::step`] was called
    prev_state: Vec<u64>,

    /// The ID of the pot at the beginning (least-significant bit) of the bit vector `state`
    pos: i64,

    /// The length of the bit vector `state` (in bits)
    len: usize,
}

impl Pots {
    /// Parses the initial state into a bit vector
    fn from(initial_state: &[u8]) -> Self {
        // Leave four bits empty at the beginning, so extracting bits in `step()` is easier
        let mut state: Vec<u64> = vec![0];
        let mut index_last = 0;
        for (i, &b) in initial_state.iter().enumerate() {
            let r = (i + 4) % 64;
            if r == 0 {
                state.push(0);
                index_last += 1;
            }
            if b == b'#' {
                state[index_last] |= 1 << r;
            }
        }

        // Truncate bit vector at the last set bit
        let len = get_len(&state);

        Self { state, prev_state: Vec::new(), pos: -4, len }
    }

    /// Applies the given rules to the pots and updates [`Self::state`]. A copy of the state before
    /// this method was called is left in [`Self::prev_state`].
    fn step(&mut self, rules: &[bool; 32]) {
        // Prepare new state
        swap(&mut self.state, &mut self.prev_state);
        self.state.clear();
        self.state.push(0);
        let mut index_last = 0;

        // Leave four bits empty at the beginning
        let mut j = 4;

        // Skip trailing zeros so the pots always start at the same bit position, regardless of
        // `self::pos`
        let mut i = self.prev_state[0].trailing_zeros() as usize - 4;
        self.pos += i as i64 - 2;

        // Apply rules and built up new state
        while i < self.len {
            let q = i / 64;
            let r = i % 64;

            // Extract up to five bits from the state at index q and position r
            let mut w = ((self.prev_state[q] >> r) & 0b11111) as usize;

            // If necessary, extract remaining bits from index q + 1
            if r >= 60 && q + 1 < self.prev_state.len() {
                w |= (self.prev_state[q + 1] as usize & ((1 << (r - 59)) - 1)) << (64 - r);
            }

            if j % 64 == 0 {
                self.state.push(0);
                index_last += 1;
                j = 0;
            }
            if rules[w] {
                self.state[index_last] |= 1 << j;
            }

            j += 1;
            i += 1;
        }

        // Truncate bit vector at the last set bit
        self.len = get_len(&self.state);
    }

    /// Returns the sum of the numbers of all pots containing plants
    fn sum(&self) -> i64 {
        let mut result = 0;
        for (i, s) in self.state.iter().enumerate() {
            for j in s.biterator() {
                result += (i * 64 + j) as i64 + self.pos;
            }
        }
        result
    }
}
