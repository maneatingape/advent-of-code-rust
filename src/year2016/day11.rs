//! # Radioisotope Thermoelectric Generators
//!
//! Solves using a [BFS](https://en.wikipedia.org/wiki/Breadth-first_search) from the
//! starting position where each next state is the possible elevator moves either one floor up or
//! down. This was faster than using [A*](https://en.wikipedia.org/wiki/A*_search_algorithm)
//! with a heuristic.
//!
//! A huge critical optimization is the observation that generator and chip pairs are *fungible*.
//! A configuration that starts with two pairs on floor one takes the same number of steps to
//! solve whether pair A or pair B is moved first (that is, the setup `[-;-;AG,AM;BG,BM]` while on
//! floor 2 is indistinguishable from `[-;-;BG,BM;AG,AM]` on floor 2 in terms of the final result).
//! However, the relative positions of pairs still matters (the setup `[AM;AG;BG;BM]` on floor two
//! can move BG up or down; but the setup `[AM;BG;AG;BM]` on floor two can only move AG up).  To
//! maximize state sharing, represent each pair's generator and microchip position as hex
//! digits, but merge all permutations by sorting those hex digit pairs during the hash
//! function.  Including the elevator position, the hash value requires up to 30 useful bits
//! (2 + 7*4) if densely packed, although this uses a 64-bit struct with one-hot encodings.
//!
//! Next, observe that adding a chip and generator pair on floor 1 adds 12 moves to the final
//! solution; likewise, removing a pair from floor 1 (but only if there is still something
//! else left on the floor) can be solved in 12 fewer moves.  Tracking a smaller number of
//! chip and generator pairs, then adjusting by the 12 times the number of ignored pairs,
//! is inherently faster.
//!
//! The rules for a valid floor are either:
//!
//! * Any amount of microchips only with no generators.
//! * Any microchip on a floor with at least one generator must have its own generator on that floor.
//!
//! This allows us to efficiently memoize previously seen states and reject any that we've seen
//! before extremely quickly. Other optimizations:
//!
//! * If we can move 2 items up, then skip only moving 1 item up.
//! * If we can move 1 item down, then skip moving 2 items down.
//! * Moving a microchip and generator together is only safe if they are the same type (if they
//!   are not from the same type, then the old floor will necessarily have the generator that
//!   pairs with the chip being moved, leaving that chip to be fried on its new floor).
//! * If floor 1 is empty then don't move items back down to it, similarly if both floor 1 and
//!   floor 2 are empty then don't move items to them.
use crate::util::bitset::*;
use crate::util::hash::*;
use std::collections::VecDeque;

// A one-hot encoding is more efficient than 0-3.  For each byte, the generator is the
// high nibble, and the microchip the low nibble.  Only 5 bytes matter, because the part 2
// pairs contribute a constant input; the used bytes are stored in little-endian order;
// unused lanes will be 0.
const MASK: u64 = 0x0000000101010101;
const FLOOR1: u64 = (MASK << 4) | MASK;
const FLOOR2: u64 = FLOOR1 << 1;
const FLOOR3: u64 = FLOOR2 << 1;
const FLOOR4: u64 = FLOOR3 << 1;
const PAIR1: u8 = (1 << 4) | 1;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct State {
    elevator: u8, // 0-3
    pairs: u64,   // One-hot encoded floors for up to 5 item pairs.
}

impl State {
    // Reject any inconsistent setup.
    fn valid(&self, floor: u8) -> bool {
        let chips = (self.pairs) & (MASK << floor);
        let gens = (self.pairs >> 4) & (MASK << floor);
        gens == 0 || (chips & !gens) == 0
    }

    // Critical optimization treating generators and microchips as fungible.
    // Rearrange the pairs into canonical order; endianness matters for getting valid slice indices.
    fn canon(mut self) -> State {
        let mut array = self.pairs.to_le_bytes();
        array[..5].sort_unstable();
        self.pairs = u64::from_le_bytes(array);
        self
    }

    // Attempt to adjust state by moving one or two items up or down.
    fn move_floor(self, up: bool, item_mask: u64) -> Option<State> {
        // Build the new state
        let mut state = self;

        if up {
            state.pairs += item_mask;
            state.elevator += 1;
        } else {
            state.pairs -= item_mask >> 1;
            state.elevator -= 1;
        }

        (state.valid(self.elevator) && state.valid(state.elevator)).then(|| state.canon())
    }
}

pub fn parse(input: &str) -> u32 {
    let mut pairs = FastMap::new();

    // Find all items, and set an entry in state.pairs for each element name
    let mut floor = 1;
    let words: Vec<_> = input.split(&[' ', ',', '.', '-']).skip(3).collect();

    for w in words.windows(2) {
        match w[1] {
            "floor" => floor <<= 1,
            "compatible" => *pairs.entry(w[0]).or_insert(0) |= floor,
            "generator" => *pairs.entry(w[0]).or_insert(0) |= floor << 4,
            _ => (),
        }
    }

    // Optimize search by pre-handling item pairs starting on non-empty floor 1
    let mut floors = [0_u8; 8];
    let mut non_empty = false;
    let mut steps = 0;
    let mut i = 0;

    for pair in pairs.into_values() {
        if non_empty && pair == PAIR1 {
            steps += 12;
        } else {
            if (pair & PAIR1) != 0 {
                non_empty = true;
            }
            floors[i] = pair;
            i += 1;
        }
    }

    // Little-endian matters, based on the indices that canon() will use.
    let state = State { elevator: 0, pairs: u64::from_le_bytes(floors) };
    bfs(state.canon(), steps)
}

pub fn part1(input: &u32) -> u32 {
    *input
}

pub fn part2(input: &u32) -> u32 {
    // Both pairs add 12 steps each
    *input + 24
}

fn bfs(start: State, steps: u32) -> u32 {
    let mut todo = VecDeque::new();
    let mut seen = FastSet::with_capacity(500);

    todo.push_back((start, steps));
    seen.insert(start);

    while let Some((state, steps)) = todo.pop_front() {
        // Done if all items are on the top floor (the elevator will necessarily be there too).
        if state.pairs & FLOOR4 == state.pairs {
            return steps;
        }

        // Iterate over items that can be moved.
        let items = state.pairs & (FLOOR1 << state.elevator);

        // When moving down, try one item first; try two only if one didn't work.
        // Don't move down from bottom floor, or down into empty region
        if !(state.elevator == 0
            || (state.elevator == 1 && (state.pairs & FLOOR1) == 0)
            || (state.elevator == 2 && (state.pairs & (FLOOR1 | FLOOR2) == 0)))
        {
            let mut added = false;

            for i in items.biterator() {
                if let Some(next) = state.move_floor(false, 1 << i)
                    && seen.insert(next)
                {
                    added = true;
                    todo.push_back((next, steps + 1));
                }
            }

            if !added {
                for i in items.biterator() {
                    for j in items.biterator().take_while(|&j| j < i) {
                        if let Some(next) = state.move_floor(false, (1 << i) | (1 << j))
                            && seen.insert(next)
                        {
                            todo.push_back((next, steps + 1));
                        }
                    }
                }
            }
        }

        // When moving up, try two items first; try one only if two didn't work.
        // Don't move up from top floor.
        if state.elevator < 3 {
            let mut added = false;

            for i in items.biterator() {
                for j in items.biterator().take_while(|&j| j < i) {
                    if let Some(next) = state.move_floor(true, (1 << i) | (1 << j))
                        && seen.insert(next)
                    {
                        added = true;
                        todo.push_back((next, steps + 1));
                    }
                }
            }

            if !added {
                for i in items.biterator() {
                    if let Some(next) = state.move_floor(true, 1 << i)
                        && seen.insert(next)
                    {
                        todo.push_back((next, steps + 1));
                    }
                }
            }
        }
    }

    unreachable!()
}
