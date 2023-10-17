//! # Radioisotope Thermoelectric Generators
//!
//! Solves using a [BFS](https://en.wikipedia.org/wiki/Breadth-first_search) from the
//! starting position where each next state is the possible elevator moves either one floor up or
//! down. This was faster than using [A*](https://en.wikipedia.org/wiki/A*_search_algorithm)
//! with a heuristic.
//!
//! A huge critical optimization is the observation that generators and chips are *fungible*.
//! Only the total number of generators and chips on each floor is important.
//! The rules for a valid floor are either:
//!
//! * Any amount of microchips only with no generators.
//! * The amount of generators is greater than the number of microchips, ensuring that each chip
//!   is paired with its generator.
//!
//! This allows us to efficiently memoize previously seen states and reject any that we've seen
//! before extremely quickly. Other optimizations:
//!
//! * If we can move 2 items up, then skip only moving 1 item up.
//! * If we can move 1 item down, then skip moving 2 items down
//! * If floor 1 is empty then don't move items back down to it, similarly if both floor 1 and
//!   floor 2 are empty then don't move items to them.
//!
//! As a further optimization we assume that there are no more than 15 generators and microchips
//! and store the total packed into a single byte for each floor. This reduces the size of each
//! state to only 8 bytes making it quick to copy and hash.
use crate::util::hash::*;
use std::collections::VecDeque;

// Interestingly it was slightly faster using a `u32` for `elevator` so that the total size of
// the struct is 8 bytes instead of using a `u8` so that the size is 5 bytes.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct State {
    elevator: u32,
    floor: [Floor; 4],
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Floor {
    both: u8,
}

impl Floor {
    // Pack generators into the high nibble and microchips into the low nibble.
    #[inline]
    fn new(generators: usize, microchips: usize) -> Self {
        Floor { both: ((generators << 4) + microchips) as u8 }
    }

    #[inline]
    fn generators(self) -> u8 {
        self.both >> 4
    }

    #[inline]
    fn microchips(self) -> u8 {
        self.both & 0xf
    }

    #[inline]
    fn total(self) -> u8 {
        self.generators() + self.microchips()
    }

    // Check if we can move the requested number of items from this floor.
    #[inline]
    fn gte(self, other: Floor) -> bool {
        self.generators() >= other.generators() && self.microchips() >= other.microchips()
    }

    // Criticial optimization treating generators and microchips as fungible.
    #[inline]
    fn valid(self) -> bool {
        self.generators() == 0 || self.generators() >= self.microchips()
    }

    // Addition and subtraction can be done in parallel for both generators and microchips.
    #[inline]
    fn add(self, other: Floor) -> Floor {
        Floor { both: self.both + other.both }
    }

    #[inline]
    fn sub(self, other: Floor) -> Floor {
        Floor { both: self.both - other.both }
    }
}

pub fn parse(input: &str) -> State {
    // Only the *total* number of generators and microchips on each floor is important.
    let mut state = State::default();

    for (i, line) in input.lines().enumerate() {
        let generators = line.matches("generator").count();
        let microchips = line.matches("microchip").count();
        state.floor[i] = Floor::new(generators, microchips);
    }

    state
}

pub fn part1(input: &State) -> u32 {
    bfs(*input)
}

pub fn part2(input: &State) -> u32 {
    let mut modified = *input;
    modified.floor[0] = modified.floor[0].add(Floor::new(2, 2));
    bfs(modified)
}

fn bfs(start: State) -> u32 {
    // Get the total number of all generator and microchips so we know when done.
    let complete = start.floor.iter().map(|&f| f.total()).sum();
    // The lift must have a least one item and at most two.
    // As an optimization the list is ordered in *descending* order of number of items.
    let moves =
        [Floor::new(2, 0), Floor::new(1, 1), Floor::new(0, 2), Floor::new(1, 0), Floor::new(0, 1)];

    let mut todo = VecDeque::new();
    let mut seen = FastSet::with_capacity(30_000);

    todo.push_back((start, 0));
    seen.insert(start);

    while let Some((state, steps)) = todo.pop_front() {
        if state.floor[3].total() == complete {
            return steps;
        }

        let current = state.elevator as usize;

        // Only move down if it makes sense.
        if (state.elevator == 1 && state.floor[0].total() > 0)
            || (state.elevator == 2 && (state.floor[0].total() > 0 || state.floor[1].total() > 0))
            || state.elevator == 3
        {
            let below = current - 1;
            let mut min = 2;

            for &delta in moves.iter().rev() {
                // If we can move 1 item down then skip moving 2.
                if delta.total() > min {
                    break;
                }

                // Check we have enough items to move
                if state.floor[current].gte(delta) {
                    let candidate = state.floor[below].add(delta);

                    // Check the destination floor won't fry any microchips.
                    if candidate.valid() {
                        // Compute the next state
                        let mut next = state;
                        next.floor[current] = next.floor[current].sub(delta);
                        next.floor[below] = candidate;
                        next.elevator -= 1;

                        // Reject any previously seen states.
                        if seen.insert(next) {
                            min = delta.total();
                            todo.push_back((next, steps + 1));
                        }
                    }
                }
            }
        }

        if state.elevator < 3 {
            let above = current + 1;
            let mut max = 0;

            for delta in moves {
                // If we can move 2 items up then skip moving just 1.
                if delta.total() < max {
                    break;
                }

                // Check we have enough items to move
                if state.floor[current].gte(delta) {
                    let candidate = state.floor[above].add(delta);

                    // Check the destination floor won't fry any microchips.
                    if candidate.valid() {
                        // Compute the next state
                        let mut next = state;
                        next.floor[current] = next.floor[current].sub(delta);
                        next.floor[above] = candidate;
                        next.elevator += 1;

                        // Reject any previously seen states.
                        if seen.insert(next) {
                            max = delta.total();
                            todo.push_back((next, steps + 1));
                        }
                    }
                }
            }
        }
    }

    unreachable!()
}
