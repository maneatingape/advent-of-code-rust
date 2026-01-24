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
use crate::util::bitset::*;
use std::mem::swap;

type Input = (i64, i64);

pub fn parse(input: &str) -> Input {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();

    // Parse initial state
    let initial_state = &lines[0][15..];
    let mut pots = Pots::from(initial_state);

    // Parse rules into a table with all possible 2⁵=32 patterns
    let mut rules = [0; 32];
    for line in &lines[2..] {
        if line[9] == b'#' {
            let binary = (0..5).fold(0, |acc, i| acc | (usize::from(line[i] == b'#') << i));
            rules[binary] = 1;
        }
    }

    // Part 1 - Simulate the first 20 steps
    for _ in 0..20 {
        pots.step(&rules);
    }
    let part_one = pots.sum();

    // Part 2 - Only simulate until the generation repeats
    let mut prev_pos;
    for steps in 20.. {
        prev_pos = pots.pos;
        pots.step(&rules);
        if pots.state == pots.prev_state {
            // Generation has repeated - extrapolate to 50 billion steps
            pots.pos += (pots.pos - prev_pos) * (50_000_000_000 - steps - 1);
            break;
        }
    }
    let part_two = pots.sum();

    // Return answer
    (part_one, part_two)
}

pub fn part1(input: &Input) -> i64 {
    input.0
}

pub fn part2(input: &Input) -> i64 {
    input.1
}

struct Pots {
    /// A bit vector representing the pots. 1 means there is a plant in the pot, 0 means there
    /// isn't.
    state: Vec<usize>,

    /// A copy of the bit vector `state` before [`Self::step`] was called
    prev_state: Vec<usize>,

    /// The ID of the pot at the beginning (least-significant bit) of the bit vector `state`
    pos: i64,
}

impl Pots {
    /// Parses the initial state into a bit vector
    fn from(initial_state: &[u8]) -> Self {
        // Leave four bits empty at the beginning, so extracting bits in `step()` is easier
        let mut state = vec![0];
        let mut index_last = 0;
        for (i, &b) in initial_state.iter().enumerate() {
            let r = (i + 4) % 64;
            if r == 0 {
                state.push(0);
                index_last += 1;
            }
            state[index_last] |= usize::from(b == b'#') << r;
        }

        Self { state, prev_state: Vec::new(), pos: -4 }
    }

    /// Applies the given rules to the pots and updates [`Self::state`]. A copy of the state before
    /// this method was called is left in [`Self::prev_state`].
    fn step(&mut self, rules: &[usize; 32]) {
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

        // Truncate bit vector at the last set bit
        let len = (self.prev_state.len() - 1) * 64
            + (64 - self.prev_state[self.prev_state.len() - 1].leading_zeros() as usize);

        // Apply rules and built up new state
        while i < len {
            let q = i / 64;
            let r = i % 64;

            // Extract up to five bits from the state at index q and position r
            let mut w = (self.prev_state[q] >> r) & 0b11111;

            // If necessary, extract remaining bits from index q + 1
            if r >= 60 && q + 1 < self.prev_state.len() {
                w |= (self.prev_state[q + 1] & ((1 << (r - 59)) - 1)) << (64 - r);
            }

            if j % 64 == 0 {
                self.state.push(0);
                index_last += 1;
                j = 0;
            }
            self.state[index_last] |= rules[w] << j;

            j += 1;
            i += 1;
        }
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
