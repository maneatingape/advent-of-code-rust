//! # Subterranean Sustainability
//!
//! The problem is a one dimensional version of
//! [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).
//!
//! We use a vector to store which pots are occupied and which are empty in each generation.
//! When calulating the next step, we truncate the bit vector on the left and right.
//! This makes it easier to compare generations in part two.
//!
//! The trick for part two is that the plants will eventually stabilize into a stable pattern
//! (similar to a [glider](https://en.wikipedia.org/wiki/Glider_(Conway%27s_Game_of_Life)))
//! that moves by the same amount each generation. Once two subsequent generations are the same,
//! except for the starting position, we extrapolate 50 billion generations into the future.
use std::iter::repeat_n;
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
            let binary = (0..5).fold(0, |acc, i| (acc << 1) | usize::from(line[i] == b'#'));
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
    (part_one, part_two)
}

pub fn part1(input: &Input) -> i64 {
    input.0
}

pub fn part2(input: &Input) -> i64 {
    input.1
}

struct Pots {
    /// Vector representing the pots. 1 means there is a plant in the pot, 0 means there isn't.
    state: Vec<u8>,
    /// A copy of the vector `state` before [`Self::step`] was called.
    prev_state: Vec<u8>,
    /// The id of the pot at the beginning of the bit vector `state`.
    pos: i64,
}

impl Pots {
    /// Parses the initial state into a bit vector
    fn from(initial_state: &[u8]) -> Self {
        let state: Vec<_> = initial_state.iter().map(|&b| u8::from(b == b'#')).collect();
        Self { state, prev_state: Vec::new(), pos: 0 }
    }

    /// Applies the given rules to the pots and updates [`Self::state`]. A copy of the state before
    /// this method was called is left in [`Self::prev_state`].
    fn step(&mut self, rules: &[u8; 32]) {
        // Prepare new state
        swap(&mut self.state, &mut self.prev_state);
        self.state.clear();

        let start = self.prev_state.iter().position(|&b| b == 1).unwrap();
        let end = self.prev_state.iter().rposition(|&b| b == 1).unwrap();
        let mut mask = 0;

        // Apply rules and built up new state.
        // Pad zeros onto the end to make handling next state easier.
        for b in self.prev_state[start..=end].iter().copied().chain(repeat_n(0, 4)) {
            mask = ((mask << 1) | b as usize) & 0b11111;
            self.state.push(rules[mask]);
        }

        // Update start position.
        self.pos += start as i64 - 2;
    }

    /// Returns the sum of the numbers of all pots containing plants
    fn sum(&self) -> i64 {
        self.state.iter().enumerate().map(|(i, &s)| (self.pos + i as i64) * s as i64).sum()
    }
}
